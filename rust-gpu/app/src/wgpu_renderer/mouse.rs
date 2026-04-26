use winit::event::{ElementState, MouseButton};

pub struct Mouse {
    pub position: [f32; 2],
    pub buttons_state: ButtonsState,
}

pub struct ButtonsState {
    pub lmb: ElementState,
    pub rmb: ElementState,
}

impl Mouse {
    pub fn new() -> Self {
        Self {
            position: [0.0; 2],
            buttons_state: ButtonsState {
                lmb: ElementState::Released,
                rmb: ElementState::Released,
            },
        }
    }

    pub fn update_pos(&mut self, new_position: [f32; 2]) {
        println!("{:?}", new_position);
        self.position = new_position;
    }

    pub fn update_button(&mut self, mouse_button: MouseButton, new_state: ElementState) {
        match mouse_button {
            MouseButton::Left => self.buttons_state.lmb = new_state,
            MouseButton::Right => self.buttons_state.lmb = new_state,
            _ => (),
        }
    }
}
