use crate::wgpu_renderer::mouse::Mouse;
use crate::wgpu_renderer::swapchain::SwapchainManager;
use crate::wgpu_renderer::{particle_manager::ParticleManager, renderer::Renderer};
use anyhow::Context;
use shaders::Particle;
use shaders::shared::ShaderConstants;
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
    // We need last frame to calculate dt
    last_frame: Instant,
    is_full_screen: bool,
    window: Arc<Window>,
    renderer: Renderer,
    swapchain: SwapchainManager<'static>,
    particle_manager: ParticleManager,
    mouse: Mouse,
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
        // Vertex writable storage is required so that we can mutate a storage buffer and still use
        // it in the vertex shader
        let required_features =
            wgpu::Features::IMMEDIATES | wgpu::Features::VERTEX_WRITABLE_STORAGE;
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

        // Create a particle particle manager
        let mut particle_manager = ParticleManager::new();
        // Add 1 so that it aint empty initially
        particle_manager.particles.push(Particle {
            position: [1440.0 / 2.0, 2560.0 / 2.0],
            velocity: [0.0; 2],
            color: [1.0; 3],
            _pad: 0.0,
        });

        // Create a mouse manager-ish
        let mouse = Mouse::new();

        // Initialise the state
        Ok(Self {
            start: Instant::now(),
            last_frame: Instant::now(),
            is_full_screen: false,
            particle_manager,
            mouse,
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
            WindowEvent::RedrawRequested => self.handle_redraw()?,
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse
                    .update_pos([position.x as f32, position.y as f32]);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                self.mouse.update_button(button, state);
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key: Key::Named(NamedKey::F11),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                match self.is_full_screen {
                    true => self.window.set_fullscreen(None),
                    false => {
                        self.window
                            .set_fullscreen(Some(winit::window::Fullscreen::Borderless(
                                self.window.current_monitor(),
                            )))
                    }
                }
                self.is_full_screen = !self.is_full_screen
            }
            // ESC or CloseRequested exit the event loop
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key: Key::Named(NamedKey::Escape),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            }
            | WindowEvent::CloseRequested => event_loop.exit(),
            // If a window is resized, we have to recreate the surface
            WindowEvent::Resized(_) => self.swapchain.set_should_recreate_true(),
            _ => (),
        }
        Ok(())
    }

    pub fn handle_redraw(&mut self) -> anyhow::Result<()> {
        if self.mouse.buttons_state.lmb == ElementState::Pressed {
            self.particle_manager.create_particle(self.mouse.position);
        }

        if self.mouse.buttons_state.rmb == ElementState::Pressed {
            self.particle_manager.remove_all_particles();
        }

        let dt = self.last_frame.elapsed().as_secs_f32();
        // Update the last frame feild.
        self.last_frame = Instant::now();

        // We call the render function, which will give us the view texture
        self.swapchain.render(|render_target| {
            // Then we call the renderer and pass in all the params
            self.renderer.render(
                &ShaderConstants {
                    // Pretty cool method to get current time in the application ngl
                    time: self.start.elapsed().as_secs_f32(),
                    dt,
                    width: render_target.texture().width(),
                    height: render_target.texture().height(),
                    aspect_ratio: render_target.texture().width() as f32
                        / render_target.texture().height() as f32,
                    num_particles: self.particle_manager.particles.len() as u32,
                    _pad: [0.0; 2],
                },
                render_target,
                &self.particle_manager.particles,
            )
        })?;

        self.window.request_redraw();
        Ok(())
    }
}
