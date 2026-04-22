use crate::wgpu_renderer::renderer::Renderer;
use crate::wgpu_renderer::swapchain::SwapchainManager;
use anyhow::Context;
use shaders::ShaderConstants;
use std::sync::Arc;
use std::time::Instant;
use winit::{
    dpi::LogicalSize,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{Key, NamedKey},
    window::{Window, WindowId},
};

// State struct will be managing all the sub-processes
pub struct State {
    start: Instant,
    window: Arc<Window>,
    renderer: Renderer,
    swapchain: SwapchainManager<'static>,
}

impl State {
    pub async fn new(event_loop: &ActiveEventLoop) -> anyhow::Result<Self> {
        // Firstly, create a new window
        let window = Arc::new(
            event_loop.create_window(
                Window::default_attributes()
                    .with_title("Physics")
                    .with_inner_size(LogicalSize::new(2560, 1440)),
            )?,
        );

        // Create a new instance
        let instance =
            wgpu::Instance::new(wgpu::InstanceDescriptor::new_with_display_handle_from_env(
                Box::new(event_loop.owned_display_handle()),
            ));

        // Get the surface and a good adapter
        let surface = instance.create_surface(window.clone())?;
        let adapter =
            wgpu::util::initialize_adapter_from_env_or_default(&instance, Some(&surface)).await?;

        // Small fast bits of memory that can be updated in a render pass
        let required_features = wgpu::Features::IMMEDIATES;
        let required_limits = wgpu::Limits {
            // Only 128 bits, shocker
            max_immediate_size: 128,
            ..Default::default()
        };

        // Get the device and queue as usual
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features,
                required_limits,
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                memory_hints: wgpu::MemoryHints::Performance,
                trace: Default::default(),
            })
            .await
            .context("Failed to create device")?;

        // Create a swapchain, which handles the surface as well as view texture
        let swapchain = SwapchainManager::new(
            instance.clone(),
            adapter.clone(),
            device.clone(),
            window.clone(),
            surface,
        );

        // Create a renderer
        let renderer = Renderer::new(device, queue, swapchain.get_format())?;

        // Initialise the state
        Ok(Self {
            start: Instant::now(),
            window,
            swapchain,
            renderer,
        })
    }

    // Handles all the window requests
    pub fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _id: WindowId,
        event: WindowEvent,
    ) -> anyhow::Result<()> {
        match event {
            // So if a draw is requested
            WindowEvent::RedrawRequested => {
                // We call the render function, which will give us the view texture
                self.swapchain.render(|render_target| {
                    // Then we call the renderer and pass in all the params
                    self.renderer.render(
                        &ShaderConstants {
                            time: self.start.elapsed().as_secs_f32(),
                            width: render_target.texture().width(),
                            height: render_target.texture().height(),
                            aspect_ratio: render_target.texture().width() as f32 / render_target.texture().height() as f32
                        },
                        render_target,
                    )
                })?;
                self.window.request_redraw();
            }
            // Handle user key input, specfiically if its Escape thats pressed
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key: Key::Named(NamedKey::Escape),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            }
            // Or if directly requested to close, exit the loop
            | WindowEvent::CloseRequested => event_loop.exit(),
            // If a window is resized, we have to recreate the surface
            WindowEvent::Resized(_) => self.swapchain.set_should_recreate_true(),
            _ => (),
        }
        Ok(())
    }
}
