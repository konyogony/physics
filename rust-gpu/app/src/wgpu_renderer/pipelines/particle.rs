use crate::wgpu_renderer::bind_group::{
    ConstantsBindGroups, ElectricBindGroups, GlobalBindGroupLayout, ParticleBindGroups,
};
use shaders_shared::{POLYGON_VERTICES, ShaderConstants};
use wgpu::{
    ColorTargetState, ColorWrites, ComputePass, Device, FragmentState, FrontFace, MultisampleState,
    PipelineLayoutDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology, RenderPass,
    RenderPipeline, RenderPipelineDescriptor, TextureFormat, VertexState, include_spirv,
};
use wgpu::{ComputePipeline, ComputePipelineDescriptor};

pub struct ParticlePipeline {
    compute_pipeline: ComputePipeline,
    render_pipeline: RenderPipeline,
    // Since its a ping pong model, we have to keep track of what the latest data buffer is
    out_is_buffer_a: bool,
}

impl ParticlePipeline {
    pub fn new(
        device: &Device,
        global_bind_group_layout: &GlobalBindGroupLayout,
        out_format: TextureFormat,
    ) -> anyhow::Result<Self> {
        let shader_module =
            device.create_shader_module(include_spirv!(env!("PARTICLE_SHADER_PATH")));

        // Layout cannot be shared anymore
        let layout_render = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("ParticleRenderPipelineLayout"),
            // Since global bind group layout stores all layouts, we have to pass in the layouts we
            // will actually use.
            bind_group_layouts: &[
                Some(&global_bind_group_layout.constants),
                Some(&global_bind_group_layout.particles_render),
            ],
            // Have a size of the shader constants.
            immediate_size: size_of::<ShaderConstants>() as u32,
        });

        let layout_compute = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("ParticleComputePipelineLayout"),
            bind_group_layouts: &[
                Some(&global_bind_group_layout.constants),
                Some(&global_bind_group_layout.particles_compute),
                Some(&global_bind_group_layout.electric),
            ],
            immediate_size: size_of::<ShaderConstants>() as u32,
        });

        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("ParticleRenderPipeline"),
            layout: Some(&layout_render),
            vertex: VertexState {
                // Pass in that shader
                module: &shader_module,
                entry_point: Some("particle_vs"),
                compilation_options: Default::default(),
                buffers: &[],
            },
            // Default culling & settings.
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState::default(),
            fragment: Some(FragmentState {
                // Pass in that shader
                module: &shader_module,
                entry_point: Some("particle_fs"),
                compilation_options: Default::default(),
                targets: &[Some(ColorTargetState {
                    format: out_format,
                    blend: None,
                    write_mask: ColorWrites::ALL,
                })],
            }),
            multiview_mask: None,
            cache: None,
        });

        let compute_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("ParticleComputePipeline"),
            layout: Some(&layout_compute),
            module: &shader_module,
            entry_point: Some("particle_cs"),
            compilation_options: Default::default(),
            cache: None,
        });

        Ok(Self {
            render_pipeline,
            compute_pipeline,
            out_is_buffer_a: false,
        })
    }

    // Instead of passing in a global bind group -> pass in individual bind groups
    pub fn draw(
        &self,
        rpass: &mut RenderPass<'_>,
        constants_bind_groups: &ConstantsBindGroups,
        particle_bind_groups: &ParticleBindGroups,
        num_particles: u32,
    ) {
        rpass.set_pipeline(&self.render_pipeline);
        rpass.set_bind_group(0, &constants_bind_groups.constants, &[]);

        // Use the correct bind group when rendering.
        if self.out_is_buffer_a {
            rpass.set_bind_group(1, &particle_bind_groups.particles_render_a, &[]);
        } else {
            rpass.set_bind_group(1, &particle_bind_groups.particles_render_b, &[]);
        }

        // For now we draw N number of particles, where each one consists of 6 vertices
        rpass.draw(0..POLYGON_VERTICES, 0..num_particles);
    }

    pub fn compute(
        &mut self,
        cpass: &mut ComputePass<'_>,
        constants_bind_groups: &ConstantsBindGroups,
        particle_bind_groups: &ParticleBindGroups,
        electric_bind_groups: &ElectricBindGroups,
        num_particles: u32,
    ) {
        cpass.set_pipeline(&self.compute_pipeline);
        cpass.set_bind_group(0, &constants_bind_groups.constants, &[]);

        if self.out_is_buffer_a {
            cpass.set_bind_group(1, &particle_bind_groups.particles_compute_ab, &[]);
        } else {
            cpass.set_bind_group(1, &particle_bind_groups.particles_compute_ba, &[]);
        }

        cpass.set_bind_group(2, &electric_bind_groups.electric, &[]);
        cpass.dispatch_workgroups(num_particles.div_ceil(256), 1, 1);
        self.out_is_buffer_a = !self.out_is_buffer_a;
    }
}
