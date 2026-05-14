use crate::wgpu_renderer::ui::manager::UIManager;
use egui::{Button, Context, DragValue, Ui};

pub struct UI;

impl UI {
    pub fn new() -> Self {
        Self
    }

    pub fn main(&self, manager: &mut UIManager, ctx: &Context) {
        egui::Window::new("Configuration")
            .collapsible(true)
            .resizable(true)
            .default_width(400.0)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.draw_options(manager, ui);
                    self.particle_options(manager, ui);
                    self.electric_options(manager, ui);
                    self.drag_value(ui, &mut manager.input_values.color_value, "Color Value");
                    self.drag_value(
                        ui,
                        &mut manager.input_values.electric_ui_options.charge_strength,
                        "Charge Strength",
                    );
                    self.controls(ui);
                });
            });
    }

    pub fn draw_options(&self, manager: &mut UIManager, ui: &mut Ui) {
        ui.collapsing("Draw Options", |ui| {
            ui.vertical(|ui| {
                ui.checkbox(
                    &mut manager.input_values.draw_ui_options.draw_grid,
                    "Draw Grid",
                );
                ui.checkbox(
                    &mut manager.input_values.draw_ui_options.draw_vec,
                    "Draw Vector Arrows",
                );
                ui.checkbox(
                    &mut manager.input_values.draw_ui_options.draw_potential,
                    "Draw Equipotential Lines",
                );
                ui.checkbox(
                    &mut manager.input_values.draw_ui_options.draw_field_lines,
                    "Draw Field Lines",
                );
            });
        });
    }

    pub fn particle_options(&self, manager: &mut UIManager, ui: &mut Ui) {
        ui.collapsing("Particle Options", |ui| {
            ui.vertical(|ui| {
                self.drag_value(
                    ui,
                    &mut manager.input_values.particle_ui_options.polygon_vertices,
                    "Polygon Vertices",
                );
                self.drag_value(
                    ui,
                    &mut manager.input_values.particle_ui_options.time_scale,
                    "Time Scale",
                );
                self.drag_value(
                    ui,
                    &mut manager.input_values.particle_ui_options.particle_radius,
                    "Particle Radius",
                );
            });
        });
    }

    pub fn electric_options(&self, manager: &mut UIManager, ui: &mut Ui) {
        ui.collapsing("Electric Options", |ui| {
            ui.vertical(|ui| {
                self.drag_value(
                    ui,
                    &mut manager.input_values.electric_ui_options.charge_radius,
                    "Charge Radius",
                );

                self.drag_value(
                    ui,
                    &mut manager
                        .input_values
                        .electric_ui_options
                        .num_particles_per_charge,
                    "Tracing Points Per Charge",
                );

                self.drag_value(
                    ui,
                    &mut manager.input_values.electric_ui_options.max_steps,
                    "Max Tracing Steps",
                );

                self.drag_value(
                    ui,
                    &mut manager.input_values.electric_ui_options.step_size,
                    "Tracing Step Size",
                );

                self.drag_value(
                    ui,
                    &mut manager.input_values.electric_ui_options.stop_distance,
                    "Stop Distance",
                );

                self.color_picker(
                    ui,
                    &mut manager
                        .input_values
                        .electric_ui_options
                        .equipotential_color_rgba,
                    "Equipotential Lines Color RGBA",
                );
            });
        });

        ui.collapsing("Charge Creation", |ui| {
            ui.vertical(|ui| {
                self.drag_value(
                    ui,
                    &mut manager.input_values.charge_spawn_ui_options.x,
                    "Charge Spawn X",
                );
                self.drag_value(
                    ui,
                    &mut manager.input_values.charge_spawn_ui_options.y,
                    "Charge Spawn Y",
                );

                self.button_toggle(
                    ui,
                    &mut manager.input_values.charge_spawn_ui_options.toggle_charge,
                    "Switch Charge",
                );

                self.button_toggle(
                    ui,
                    &mut manager.input_values.charge_spawn_ui_options.spawn,
                    "Spawn",
                );
            })
        });
    }

    pub fn controls(&self, ui: &mut Ui) {
        ui.collapsing("Controls", |ui| {
            ui.label("F10 to toggle menu");
            ui.label("F11 to toggle fullscreen");
            ui.label("X to switch charge");
            ui.label("Ctrl+C to clear charges");
            ui.label("Shift+C to clear particles");
        });
    }

    pub fn drag_value<T>(&self, ui: &mut Ui, value: &mut T, label: &str)
    where
        T: egui::emath::Numeric,
    {
        ui.horizontal(|ui| {
            ui.add(DragValue::new(value).update_while_editing(false));
            ui.label(label);
        });
    }

    pub fn button_toggle(&self, ui: &mut Ui, value: &mut bool, label: &str) {
        ui.horizontal(|ui| {
            if ui.add(Button::new(label)).clicked() {
                *value = true
            }
        });
    }

    pub fn color_picker(&self, ui: &mut Ui, value: &mut [f32; 4], label: &str) {
        ui.horizontal(|ui| {
            ui.color_edit_button_rgba_unmultiplied(value);
            ui.label(label);
        });
    }
}
