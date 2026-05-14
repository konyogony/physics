use crate::wgpu_renderer::state::State;
use crate::wgpu_renderer::ui::manager::InputValues;
use pollster::block_on;
use shaders_shared::{DrawOptions, ElectricOptions, ParticleOptions};
use winit::event_loop::EventLoop;
use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop,
    window::WindowId,
};

mod bind_group;
mod keyboard;
mod managers;
mod mouse;
mod pipelines;
mod renderer;
mod state;
mod swapchain;
mod texture;
mod ui;

// The app struct will store the state of the application
#[derive(Default)]
pub struct App {
    // Default will force this to be None
    state: Option<State>,
}

impl ApplicationHandler for App {
    // Resumed method will be the first one to be called
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // If no state was created yet, then make one
        if self.state.is_none() {
            self.state = Some(block_on(State::new(event_loop)).unwrap());
        }
    }

    // On a window event, get the state and pass the window even to it.
    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        let state = self.state.as_mut().unwrap();
        state.window_event(event_loop, id, event).unwrap();
    }
}

// This is the entry point for the wgpu renderer
pub fn main() -> anyhow::Result<()> {
    env_logger::init();
    // Default stuff like creating an event loop and the app.
    let event_loop = EventLoop::new()?;
    let mut app = App::default();
    event_loop.run_app(&mut app)?;
    Ok(())
}

// Implement all the stuff needed for constnats, didnt know where to put so now it lives here.
// Saves space inside state.rs

impl From<&InputValues> for ElectricOptions {
    fn from(value: &InputValues) -> Self {
        Self {
            charge_radius: value.electric_ui_options.charge_radius,
            num_particles_per_charge: value.electric_ui_options.num_particles_per_charge,
            max_steps: value.electric_ui_options.max_steps as u32,
            step_size: value.electric_ui_options.step_size,
            stop_distance: value.electric_ui_options.stop_distance,
            _pad: [0.0; 3],
            equipotential_color_rgba: value.electric_ui_options.equipotential_color_rgba,
        }
    }
}

impl From<&InputValues> for DrawOptions {
    fn from(value: &InputValues) -> Self {
        Self {
            draw_grid: value.draw_ui_options.draw_grid as u32,
            draw_vec: value.draw_ui_options.draw_vec as u32,
            draw_potential: value.draw_ui_options.draw_potential as u32,
            draw_field_lines: value.draw_ui_options.draw_field_lines as u32,
        }
    }
}

impl From<&InputValues> for ParticleOptions {
    fn from(value: &InputValues) -> Self {
        Self {
            time_scale: value.particle_ui_options.time_scale,
            particle_radius: value.particle_ui_options.particle_radius,
            polygon_vertices: value.particle_ui_options.polygon_vertices,
            _pad: 0.0,
        }
    }
}
