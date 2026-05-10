use crate::wgpu_renderer::keyboard::InputActions;
use crate::wgpu_renderer::renderer::Renderer;
use crate::wgpu_renderer::swapchain::SwapchainManager;
use crate::wgpu_renderer::{keyboard::Keyboard, mouse::Mouse};
use anyhow::Context;
use shaders_shared::{Charge, DrawOptions, ShaderConstants};
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
    mouse: Mouse,
    keyboard: Keyboard,
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
        let required_features = wgpu::Features::IMMEDIATES
            | wgpu::Features::VERTEX_WRITABLE_STORAGE
            | wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES
            | wgpu::Features::STORAGE_RESOURCE_BINDING_ARRAY;
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

        let size = swapchain.get_size();
        let config = swapchain.get_config().unwrap();
        let format = swapchain.get_format();

        let charges = vec![Charge {
            position: [size.width as f32 / 2.0, size.height as f32 / 2.0],
            charge: 1.0,
            _pad: 0.0,
        }];

        // Create a renderer
        let renderer = Renderer::new(&window, device, queue, config, format, size, charges)?;

        // Create a mouse manager-ish
        let mouse = Mouse::new();
        let keyboard = Keyboard::new();

        // Initialise the state
        Ok(Self {
            start: Instant::now(),
            last_frame: Instant::now(),
            is_full_screen: false,
            mouse,
            keyboard,
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
        // Basically feed the input into UI and if it consumes it, then stop processes.
        if self.renderer.ui_manager.handle_event(&self.window, &event) {
            return Ok(());
        }

        match event {
            // So if a draw is requested
            WindowEvent::RedrawRequested => self.handle_redraw()?,
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse
                    .update_pos([position.x as f32, position.y as f32]);
            }

            WindowEvent::MouseInput { state, button, .. } => {
                self.mouse.update_button(button, state);

                if self.mouse.buttons_state.lmb == ElementState::Pressed {
                    self.renderer
                        .particle_manager
                        .add_particle(&self.renderer.queue, self.mouse.position);
                }

                if self.mouse.buttons_state.rmb == ElementState::Pressed {
                    self.renderer
                        .electric_manager
                        .add_charge(&self.renderer.queue, self.mouse.position);
                }
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
            WindowEvent::Resized(new_size) => {
                self.swapchain.set_should_recreate_true();
                self.swapchain.recreate()?;

                self.renderer.electric_manager.resize(
                    &self.renderer.device,
                    &self.renderer.queue,
                    new_size,
                    &self.renderer.global_bind_group_layout,
                );

                let config = self.swapchain.get_config().unwrap();
                self.renderer.ui_manager.resize(&self.window, &config);
            }

            WindowEvent::KeyboardInput { event, .. } => {
                self.keyboard.update_key(event.physical_key, event.state);

                let input_actions = self
                    .keyboard
                    .get_input_actions(event.physical_key, event.state);
                self.handle_input_actions(input_actions);
            }
            _ => (),
        }
        Ok(())
    }

    pub fn handle_input_actions(&mut self, input_actions: InputActions) {
        if input_actions.toggle_fullscreen {
            match self.is_full_screen {
                true => self.window.set_fullscreen(None),
                false => self
                    .window
                    .set_fullscreen(Some(winit::window::Fullscreen::Borderless(
                        self.window.current_monitor(),
                    ))),
            }
            self.is_full_screen = !self.is_full_screen
        }

        if input_actions.increment_color_fast {
            self.renderer.ui_manager.input_values.color_value += 5.0;
        }
        if input_actions.increment_color {
            self.renderer.ui_manager.input_values.color_value += 0.5;
        }

        if input_actions.decrement_color_fast
            && self.renderer.ui_manager.input_values.color_value >= 5.0
        {
            self.renderer.ui_manager.input_values.color_value -= 5.0;
        }

        if input_actions.decrement_color && self.renderer.ui_manager.input_values.color_value >= 0.5
        {
            self.renderer.ui_manager.input_values.color_value -= 0.5;
        }

        if input_actions.remove_particles {
            self.renderer.particle_manager.remove_all_particles();
        }
        if input_actions.remove_charges {
            self.renderer.electric_manager.remove_all_charges();
        }

        if input_actions.toggle_charge {
            self.renderer.electric_manager.toggle_charge();
        }
        if input_actions.toggle_ui {
            self.renderer.ui_manager.toggle_active();
        }
    }

    pub fn handle_redraw(&mut self) -> anyhow::Result<()> {
        let dt = self.last_frame.elapsed().as_secs_f32();
        // Update the last frame feild.
        self.last_frame = Instant::now();

        // We call the render function, which will give us the view texture
        self.swapchain.render(|render_target| {
            // Then we call the renderer and pass in all the params
            self.renderer.render(
                &self.window,
                &ShaderConstants {
                    // Pretty cool method to get current time in the application ngl
                    time: self.start.elapsed().as_secs_f32(),
                    dt,
                    width: render_target.texture().width(),
                    height: render_target.texture().height(),
                    aspect_ratio: render_target.texture().width() as f32
                        / render_target.texture().height() as f32,
                    num_particles: self.renderer.particle_manager.current_num_of_particles,
                    // This is the real life value, however, its NOT going to work great for
                    // pixels, hence we will use a different one.
                    //epsilon_naught: ((8.9_f32).powi(-12)),
                    epsilon_naught: ((8.9_f32).powi(-8)),
                    num_charges: self.renderer.electric_manager.charges.len() as u32,
                    color_value: self.renderer.ui_manager.input_values.color_value,
                    draw_options: DrawOptions {
                        draw_grid: self.renderer.ui_manager.input_values.draw_grid as u32,
                        draw_vec: self.renderer.ui_manager.input_values.draw_vec as u32,
                        draw_potential: self.renderer.ui_manager.input_values.draw_potential as u32,
                    },
                },
                render_target,
            )
        })?;

        self.window.request_redraw();
        Ok(())
    }
}
