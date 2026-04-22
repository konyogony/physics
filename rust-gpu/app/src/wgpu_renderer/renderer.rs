use crate::wgpu_renderer::bind_group::GlobalBindGroupLayout;
use crate::wgpu_renderer::renderer_pipeline::RendererPipeline;
use shaders::ShaderConstants;
use wgpu::wgt::CommandEncoderDescriptor;
use wgpu::{
    Color, Device, LoadOp, Operations, Queue, RenderPassColorAttachment, RenderPassDescriptor,
    StoreOp, TextureFormat, TextureView,
};

// This file is basically responsible for first of all

// Renderer holds the device & queue + layout & pipeline, responsible for rendering
pub struct Renderer {
    pub device: Device,
    pub queue: Queue,
    global_bind_group_layout: GlobalBindGroupLayout,
    pipeline: RendererPipeline,
}

impl Renderer {
    pub fn new(device: Device, queue: Queue, out_format: TextureFormat) -> anyhow::Result<Self> {
        // Create a "global" bind group layout
        let global_bind_group_layout = GlobalBindGroupLayout::new(&device);
        // Create a new pipeline
        let pipeline = RendererPipeline::new(&device, &global_bind_group_layout, out_format)?;

        // Pass it in
        Ok(Self {
            global_bind_group_layout,
            pipeline,
            device,
            queue,
        })
    }

    // The render function, we pass in the shader constants which will
    // be converted into a storage buffer, as well as a TextureView acquired from
    // the swapchain
    pub fn render(
        &self,
        shader_constants: &ShaderConstants,
        output: TextureView,
    ) -> anyhow::Result<()> {
        // Create a bind group by passing it the shader consnats
        let global_bind_group = self
            .global_bind_group_layout
            .create_bind_group(&self.device, shader_constants);

        // Create a command encoder, responsible for drawing the stuff
        let mut cmd = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("main_draw"),
            });

        // Create a render pass
        let mut rpass = cmd.begin_render_pass(&RenderPassDescriptor {
            label: Some("main_renderpass"),
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
        self.pipeline.draw(&mut rpass, &global_bind_group);
        // Drop so that memory is freed up.
        drop(rpass);

        // Submit once the completed draw call.
        self.queue.submit(std::iter::once(cmd.finish()));
        Ok(())
    }
}
