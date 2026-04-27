use crate::wgpu_renderer::bind_group::{ConstantsBindGroups, GlobalBindGroupLayout};
use shaders::shared::ShaderConstants;
use wgpu::{
    ColorTargetState, ColorWrites, Device, FragmentState, FrontFace, MultisampleState,
    PipelineLayoutDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology, RenderPass,
    RenderPipeline, RenderPipelineDescriptor, TextureFormat, VertexState, include_spirv,
};

// The render pipeline is responsible for creating the shader, the pipeline layout, the pipeline
// and then drawing it onto the screen.
#[derive(Debug, Clone)]
pub struct GridPipeline {
    render_pipeline: RenderPipeline,
}

impl GridPipeline {
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
            label: Some("GridPipelineLayout"),
            bind_group_layouts: &[Some(&global_bind_group_layout.constants)],
            // Have a size of the shader constants.
            immediate_size: size_of::<ShaderConstants>() as u32,
        });

        // Create the pipeline itself
        Ok(Self {
            render_pipeline: device.create_render_pipeline(&RenderPipelineDescriptor {
                label: Some("GridRenderPipeline"),
                layout: Some(&layout),
                vertex: VertexState {
                    // Pass in that shader
                    module: &shader_module,
                    entry_point: Some("grid_vs"),
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
                    entry_point: Some("grid_fs"),
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
    pub fn draw(&self, rpass: &mut RenderPass<'_>, constants_bind_groups: &ConstantsBindGroups) {
        // First set the pipeline that we have created
        rpass.set_pipeline(&self.render_pipeline);
        // Pass in the bind groups
        rpass.set_bind_group(0, &constants_bind_groups.constants, &[]);
        // Since we are just looking to cover whole screen, make 3 vertices, 1 draw pass.
        rpass.draw(0..3, 0..1);
    }
}
