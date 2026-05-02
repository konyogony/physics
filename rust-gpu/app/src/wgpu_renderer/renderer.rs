use crate::wgpu_renderer::bind_group::GlobalBindGroupLayout;
use crate::wgpu_renderer::electric_manager::ElectricManager;
use crate::wgpu_renderer::particle_manager::ParticleManager;
use crate::wgpu_renderer::pipelines::electric::ElectricPipeline;
use crate::wgpu_renderer::pipelines::grid::GridPipeline;
use crate::wgpu_renderer::pipelines::particle::ParticlePipeline;
use shaders::Charge;
use shaders::shared::ShaderConstants;
use wgpu::wgt::CommandEncoderDescriptor;
use wgpu::{
    Color, ComputePassDescriptor, Device, LoadOp, Operations, Queue, RenderPassColorAttachment,
    RenderPassDescriptor, StoreOp, TextureFormat, TextureView,
};

// This file is basically responsible for first of all
// Renderer holds the device & queue + layout & pipeline, responsible for rendering
pub struct Renderer {
    pub device: Device,
    pub queue: Queue,
    // Basically responsible for ALL bind group layouts and the creation of bind groups themselves
    global_bind_group_layout: GlobalBindGroupLayout,
    grid_pipeline: GridPipeline,
    particle_pipeline: ParticlePipeline,
    electric_pipeline: ElectricPipeline,
    electric_manger: ElectricManager,
    pub particle_manager: ParticleManager,
}

impl Renderer {
    pub fn new(
        device: Device,
        queue: Queue,
        out_format: TextureFormat,
        (width, height): (u32, u32),
        charges_vec: Vec<Charge>,
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

        let electric_pipeline = ElectricPipeline::new(&device, &global_bind_group_layout)?;

        let electric_manger = ElectricManager::new(
            &device,
            &global_bind_group_layout,
            (width, height),
            charges_vec,
        );

        // Pass it in
        Ok(Self {
            global_bind_group_layout,
            electric_pipeline,
            grid_pipeline,
            particle_pipeline,
            particle_manager,
            electric_manger,
            device,
            queue,
        })
    }

    // The render function, we pass in the shader constants which will
    // be converted into a storage buffer, as well as a TextureView acquired from
    // the swapchain
    pub fn render(
        &mut self,
        shader_constants: &ShaderConstants,
        output: TextureView,
    ) -> anyhow::Result<()> {
        // Create a bind group by passing it the shader consnats
        // TODO: Make this not re-create itself 1000 times.
        let constant_buffer = self
            .global_bind_group_layout
            .create_constant_buffers(&self.device, shader_constants);

        let constant_bind_groups = self
            .global_bind_group_layout
            .create_constant_bind_groups(&self.device, &constant_buffer);

        // Create a command encoder, responsible for drawing the stuff
        // Shared between both compute & render pass
        let mut cmd_encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("MainCMDEncoder"),
            });

        // First we have to go through all the pipelies that have a compute pass
        let mut cpass = cmd_encoder.begin_compute_pass(&ComputePassDescriptor {
            label: Some("FirstComputePass"),
            timestamp_writes: None,
        });

        self.electric_pipeline.compute_potential(
            &mut cpass,
            &constant_bind_groups,
            &self.electric_manger.electric_bind_groups,
            self.electric_manger.size,
        );
        drop(cpass);

        let mut cpass = cmd_encoder.begin_compute_pass(&ComputePassDescriptor {
            label: Some("SecondComputePass"),
            timestamp_writes: None,
        });

        self.electric_pipeline.compute_field(
            &mut cpass,
            &constant_bind_groups,
            &self.electric_manger.electric_bind_groups,
            self.electric_manger.size,
        );

        self.particle_pipeline.compute(
            &mut cpass,
            &constant_bind_groups,
            &self.particle_manager.particle_bind_groups,
            self.particle_manager.current_num_of_particles,
        );
        // Dont forget to drop after each pass
        drop(cpass);

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
            &constant_bind_groups,
            &self.electric_manger.electric_bind_groups,
        );

        self.particle_pipeline.draw(
            &mut rpass,
            &constant_bind_groups,
            &self.particle_manager.particle_bind_groups,
            self.particle_manager.current_num_of_particles,
        );
        drop(rpass);

        // Submit once the completed draw call.
        self.queue.submit(std::iter::once(cmd_encoder.finish()));
        Ok(())
    }
}
