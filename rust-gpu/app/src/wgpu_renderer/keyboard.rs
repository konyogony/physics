use std::collections::HashMap;

use winit::{
    event::ElementState,
    keyboard::{KeyCode, PhysicalKey},
};

#[derive(Default, Debug, Clone, Copy)]
pub struct InputActions {
    pub toggle_fullscreen: bool,
    pub increment_color: bool,
    pub increment_color_fast: bool,
    pub decrement_color: bool,
    pub decrement_color_fast: bool,
    pub remove_particles: bool,
    pub remove_charges: bool,
    pub toggle_charge: bool,
    pub toggle_ui: bool,
}

#[derive(Default, Debug, Clone)]
pub struct Keyboard {
    key_states: HashMap<PhysicalKey, ElementState>,
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            key_states: HashMap::new(),
        }
    }

    pub fn update_key(&mut self, key: PhysicalKey, state: ElementState) {
        self.key_states.insert(key, state);
    }

    pub fn is_pressed(&self, key: PhysicalKey) -> bool {
        if let Some(k) = self.key_states.get(&key) {
            return k.is_pressed();
        };
        false
    }

    pub fn is_just_pressed(&self, key: PhysicalKey, event_state: ElementState) -> bool {
        event_state == ElementState::Pressed && self.is_pressed(key)
    }

    pub fn get_input_actions(
        &self,
        current_key: PhysicalKey,
        current_state: ElementState,
    ) -> InputActions {
        let mut actions = InputActions::default();

        let shift = self.is_pressed(PhysicalKey::Code(KeyCode::ShiftLeft));
        let cntrl = self.is_pressed(PhysicalKey::Code(KeyCode::ControlLeft));

        if current_state == ElementState::Pressed {
            match current_key {
                PhysicalKey::Code(KeyCode::F11) => actions.toggle_fullscreen = true,
                PhysicalKey::Code(KeyCode::Period) => {
                    if shift {
                        actions.increment_color_fast = true
                    } else {
                        actions.increment_color = true
                    }
                }
                PhysicalKey::Code(KeyCode::Comma) => {
                    if shift {
                        actions.decrement_color_fast = true
                    } else {
                        actions.decrement_color = true
                    }
                }
                PhysicalKey::Code(KeyCode::KeyC) => {
                    // Shift + C -> Remove Particles
                    if shift {
                        actions.remove_particles = true
                    }
                    // Cntrl + C -> Remove Particles
                    if cntrl {
                        actions.remove_charges = true
                    }
                }
                PhysicalKey::Code(KeyCode::KeyX) => actions.toggle_charge = true,
                PhysicalKey::Code(KeyCode::F10) => actions.toggle_ui = true,
                _ => {}
            }
        }

        actions
    }
}
