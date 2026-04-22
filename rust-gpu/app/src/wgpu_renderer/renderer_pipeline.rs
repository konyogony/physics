use crate::wgpu_renderer::bind_group::{GlobalBindGroup, GlobalBindGroupLayout};
use bytemuck::{Pod, Zeroable};
use shaders::ShaderConstants;
use wgpu::{
    Buffer, BufferAddress, BufferUsages, ColorTargetState, ColorWrites, Device, FragmentState,
    FrontFace, MultisampleState, PipelineLayoutDescriptor, PolygonMode, PrimitiveState,
    PrimitiveTopology, RenderPass, RenderPipeline, RenderPipelineDescriptor, TextureFormat,
    VertexAttribute, VertexBufferLayout, VertexState, include_spirv,
    util::{BufferInitDescriptor, DeviceExt},
};

// const VERTICES: &[Vertex] = &[
//     Vertex([-1.0, 1.0]),
//     Vertex([-1.0, -1.0]),
//     Vertex([1.0, 1.0]),
//     Vertex([1.0, 1.0]),
//     Vertex([-1.0, -1.0]),
//     Vertex([1.0, -1.0]),
// ];

// The render pipeline is responsible for creating the shader, the pipeline layout, the pipeline
// and then drawing it onto the screen.
#[derive(Debug, Clone)]
pub struct RendererPipeline {
    pipeline: RenderPipeline,
    // vertex_buffer: Buffer,
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

        // let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
        //     label: Some("vertex buffer"),
        //     contents: bytemuck::cast_slice(VERTICES),
        //     usage: BufferUsages::VERTEX,
        // });

        // Create the pipeline itself
        Ok(Self {
            // vertex_buffer,
            pipeline: device.create_render_pipeline(&RenderPipelineDescriptor {
                label: Some("render_pipeline"),
                layout: Some(&layout),
                vertex: VertexState {
                    // Pass in that shader
                    module: &shader_module,
                    entry_point: Some("main_vs"),
                    compilation_options: Default::default(),
                    buffers: &[
                        //Vertex::desc()
                        ],
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
        //         rpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        // Vertices 0->3, instances 0->1
        rpass.draw(0..3, 0..1);
    }
}

// #[derive(Clone, Copy, Zeroable, Pod)]
// #[repr(C)]
// pub struct Vertex([f32; 2]);
//
// impl Vertex {
//     pub fn desc() -> wgpu::VertexBufferLayout<'static> {
//         VertexBufferLayout {
//             array_stride: std::mem::size_of::<Vertex>() as BufferAddress,
//             step_mode: wgpu::VertexStepMode::Vertex,
//             attributes: &[
//                 VertexAttribute {
//                     offset: 0,
//                     shader_location: 0,
//                     format: wgpu::VertexFormat::Float32x2,
//                 },
//                 // VertexAttribute {
//                 //     offset: std::mem::size_of::<[f32; 2]>() as BufferAddress,
//                 //     shader_location: 1,
//                 //     format: wgpu::VertexFormat::Float32x2,
//                 // },
//             ],
//         }
//     }
// }
