use crate::wgpu_renderer::bind_group::{GlobalBindGroup, GlobalBindGroupLayout};
use shaders::ShaderConstants;
use wgpu::{
    ColorTargetState, ColorWrites, Device, FragmentState, FrontFace, MultisampleState,
    PipelineLayoutDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology, RenderPass,
    RenderPipeline, RenderPipelineDescriptor, TextureFormat, VertexState, include_spirv,
};

// The render pipeline is responsible for creating the shader, the pipeline layout, the pipeline
// and then drawing it onto the screen.
#[derive(Debug, Clone)]
pub struct RendererPipeline {
    pipeline: RenderPipeline,
}

impl RendererPipeline {
    pub fn new(
        device: &Device,
        global_bind_group_layout: &GlobalBindGroupLayout,
        out_format: TextureFormat,
    ) -> anyhow::Result<Self> {
        // Create a shader module, which will already be present in the correct location via this
        // env variable.
        let shader_module = device.create_shader_module(include_spirv!(env!("SHADER_SPV_PATH")));

        // Create the render pipeline layout,
        let layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("render_pipeline_layout"),
            // Pass in the bind group, as provided in the input.
            bind_group_layouts: &[Some(&global_bind_group_layout.0)],
            // Have a size of the shader constants.
            immediate_size: size_of::<ShaderConstants>() as u32,
        });

        // Create the pipeline itself
        Ok(Self {
            pipeline: device.create_render_pipeline(&RenderPipelineDescriptor {
                label: Some("render_pipeline"),
                layout: Some(&layout),
                vertex: VertexState {
                    // Pass in that shader
                    module: &shader_module,
                    entry_point: Some("main_vs"),
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
                    entry_point: Some("main_fs"),
                    compilation_options: Default::default(),
                    targets: &[Some(ColorTargetState {
                        format: out_format,
                        blend: None,
                        write_mask: ColorWrites::ALL,
                    })],
                }),
                multiview_mask: None,
                cache: None,
            }),
        })
    }

    // Draw call
    pub fn draw(&self, rpass: &mut RenderPass<'_>, global_bind_group: &GlobalBindGroup) {
        // First set the pipeline that we have created
        rpass.set_pipeline(&self.pipeline);
        // Pass in the bind groups
        rpass.set_bind_group(0, &global_bind_group.0, &[]);
        // Vertices 0->3, instances 0->1
        rpass.draw(0..3, 0..1);
    }
}
