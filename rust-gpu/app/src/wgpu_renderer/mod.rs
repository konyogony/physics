use crate::wgpu_renderer::state::State;
use pollster::block_on;
use winit::event_loop::EventLoop;
use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop,
    window::WindowId,
};

mod bind_group;
mod managers;
mod mouse;
mod pipelines;
mod renderer;
mod state;
mod swapchain;
mod texture;

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
