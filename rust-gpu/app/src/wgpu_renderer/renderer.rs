#![allow(clippy::too_many_arguments)]
use crate::wgpu_renderer::bind_groups::GlobalBindGroupLayout;
use crate::wgpu_renderer::bind_groups::constants::{ConstantsBindGroups, ConstantsBuffers};
use crate::wgpu_renderer::managers::electric::ElectricManager;
use crate::wgpu_renderer::managers::particle::ParticleManager;
use crate::wgpu_renderer::pipelines::electric::ElectricPipeline;
use crate::wgpu_renderer::pipelines::grid::GridPipeline;
use crate::wgpu_renderer::pipelines::particle::ParticlePipeline;
use crate::wgpu_renderer::ui::manager::UIManager;
use shaders_shared::{Charge, Plate, SEGMENTS_PER_PLATE, ShaderConstants};
use wgpu::wgt::CommandEncoderDescriptor;
use wgpu::{
    Color, ComputePassDescriptor, Device, LoadOp, Operations, Queue, RenderPassColorAttachment,
    RenderPassDescriptor, StoreOp, SurfaceConfiguration, TextureFormat, TextureView,
};
use winit::dpi::PhysicalSize;
use winit::window::Window;

// This file is basically responsible for first of all
// Renderer holds the device & queue + layout & pipeline, responsible for rendering
pub struct Renderer {
    pub device: Device,
    pub queue: Queue,
    // Basically responsible for ALL bind group layouts and the creation of bind groups themselves
    pub global_bind_group_layout: GlobalBindGroupLayout,
    pub grid_pipeline: GridPipeline,
    pub particle_pipeline: ParticlePipeline,
    pub electric_pipeline: ElectricPipeline,
    pub electric_manager: ElectricManager,
    pub particle_manager: ParticleManager,
    pub ui_manager: UIManager,
    constants_buffer: ConstantsBuffers,
    constants_bind_groups: ConstantsBindGroups,
}

impl Renderer {
    pub fn new(
        window: &Window,
        device: Device,
        queue: Queue,
        config: SurfaceConfiguration,
        out_format: TextureFormat,
        size: PhysicalSize<u32>,
        initial_charges: Vec<Charge>,
        initial_plates: Vec<Plate>,
        max_steps: usize,
        num_particles_per_charge: u32,
    ) -> anyhow::Result<Self> {
        // Create all the bind groups first. Global bind group just refers to the one holding
        // shader constants, hence global.
        let global_bind_group_layout = GlobalBindGroupLayout::new(&device);

        // Create all the pipelines that we will use
        let grid_pipeline = GridPipeline::new(&device, &global_bind_group_layout, out_format)?;

        let particle_pipeline =
            ParticlePipeline::new(&device, &global_bind_group_layout, out_format)?;

        // Responsible for persistant buffers, storing count, etc..
        let particle_manager = ParticleManager::new(&device, &global_bind_group_layout);

        let electric_pipeline =
            ElectricPipeline::new(&device, &global_bind_group_layout, out_format)?;

        let electric_manager = ElectricManager::new(
            &device,
            &queue,
            &global_bind_group_layout,
            size,
            initial_charges,
            initial_plates,
            max_steps,
            num_particles_per_charge,
        );

        let ui_manager = UIManager::new(window, &device, &config, out_format);

        // create empty buffer, but only once.
        let constants_buffer = global_bind_group_layout
            .constants
            .create_constant_uniform_buffers(&device, &bytemuck::Zeroable::zeroed());

        let constants_bind_groups = global_bind_group_layout
            .constants
            .create_constant_bind_groups(&device, &constants_buffer);

        // Pass it in
        Ok(Self {
            global_bind_group_layout,
            electric_pipeline,
            grid_pipeline,
            particle_pipeline,
            particle_manager,
            electric_manager,
            ui_manager,
            device,
            queue,
            constants_buffer,
            constants_bind_groups,
        })
    }

    // The render function, we pass in the shader constants which will
    // be converted into a storage buffer, as well as a TextureView acquired from
    // the swapchain
    pub fn render(
        &mut self,
        window: &Window,
        shader_constants: &ShaderConstants,
        output: TextureView,
    ) -> anyhow::Result<()> {
        self.global_bind_group_layout
            .constants
            .update_constants_uniform_buffer(&self.queue, &self.constants_buffer, shader_constants);
        let constants_bind_groups = self.constants_bind_groups.clone();

        let capacity_grew = self
            .electric_manager
            .ensure_tracing_capacity(&self.device, &self.global_bind_group_layout.electric);

        self.electric_manager.solve_for_charges(&self.queue);

        let need_field_update = self.electric_manager.field_dirty || capacity_grew;

        // Create a command encoder, responsible for drawing the stuff
        // Shared between both compute & render pass
        let mut cmd_encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("MainCMDEncoder"),
            });

        if need_field_update {
            // First we have to go through all the pipelies that have a compute pass
            let mut cpass = cmd_encoder.begin_compute_pass(&ComputePassDescriptor {
                label: Some("FieldComputePass"),
                timestamp_writes: None,
            });

            self.electric_pipeline.compute_potential(
                &mut cpass,
                &constants_bind_groups,
                &self.electric_manager.electric_bind_groups,
                self.electric_manager.size,
            );
            drop(cpass);

            let mut cpass = cmd_encoder.begin_compute_pass(&ComputePassDescriptor {
                label: Some("SecondComputePass"),
                timestamp_writes: None,
            });

            self.electric_pipeline.compute_field(
                &mut cpass,
                &constants_bind_groups,
                &self.electric_manager.electric_bind_groups,
                self.electric_manager.size,
            );

            self.electric_pipeline.compute_tracing_plates(
                &mut cpass,
                &constants_bind_groups,
                &self.electric_manager.electric_bind_groups,
                self.electric_manager.plates.len() as u32,
                SEGMENTS_PER_PLATE,
            );

            self.electric_pipeline.compute_tracing_charges(
                &mut cpass,
                &constants_bind_groups,
                &self.electric_manager.electric_bind_groups,
                self.electric_manager.charges.len() as u32,
                self.electric_manager.num_particles_per_charge,
            );

            // Dont forget to drop after each pass
            drop(cpass);
            self.electric_manager.field_dirty = false;
        }

        let mut cpass = cmd_encoder.begin_compute_pass(&ComputePassDescriptor {
            label: Some("ParticleComputePass"),
            timestamp_writes: None,
        });

        self.particle_pipeline.compute(
            &mut cpass,
            &constants_bind_groups,
            &self.particle_manager.particle_bind_groups,
            &self.electric_manager.electric_bind_groups,
            self.particle_manager.current_num_of_particles,
        );

        drop(cpass);

        let labels: Vec<(egui::Pos2, f32)> = self
            .electric_manager
            .charges
            .clone()
            .into_iter()
            .map(|c| (c.position.into(), c.charge))
            .collect();

        // Before render pass we prepare
        self.ui_manager.prepare(
            window,
            &self.device,
            &self.queue,
            &mut cmd_encoder,
            &labels,
            self.ui_manager
                .input_values
                .electric_ui_options
                .charge_radius,
        );

        // After all the computer passes are done, create & call the rneder passes.
        let mut rpass = cmd_encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("MainRenderPass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &output,
                depth_slice: None,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color::BLACK),
                    store: StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });

        // Draw it using our pipeline we created.
        self.grid_pipeline.draw(
            &mut rpass,
            &constants_bind_groups,
            &self.electric_manager.electric_bind_groups,
        );

        self.particle_pipeline.draw(
            &mut rpass,
            &constants_bind_groups,
            &self.particle_manager.particle_bind_groups,
            self.particle_manager.current_num_of_particles,
            self.ui_manager
                .committed_input_values
                .particle_ui_options
                .polygon_vertices,
        );

        self.electric_pipeline.draw_charge(
            &mut rpass,
            &constants_bind_groups,
            &self.electric_manager.electric_bind_groups,
            self.electric_manager.charges.len() as u32,
            self.ui_manager
                .committed_input_values
                .particle_ui_options
                .polygon_vertices,
        );

        self.electric_pipeline.draw_plates(
            &mut rpass,
            &constants_bind_groups,
            &self.electric_manager.electric_bind_groups,
            self.electric_manager.plates.len() as u32,
        );

        let total_instances = (self.electric_manager.charges.len() as u32
            * self.electric_manager.num_particles_per_charge)
            + self.electric_manager.segments.len() as u32;

        self.electric_pipeline.draw_tracing(
            &mut rpass,
            &constants_bind_groups,
            &self.electric_manager.electric_bind_groups,
            self.electric_manager.max_steps,
            total_instances,
        );

        // Yes this is voodo magick.
        let mut rpass = rpass.forget_lifetime();
        // Inside render pass, we draw
        if self.ui_manager.active {
            self.ui_manager.draw(&mut rpass);
        }
        drop(rpass);

        // Submit once the completed draw call.
        self.queue.submit(std::iter::once(cmd_encoder.finish()));
        Ok(())
    }
}
