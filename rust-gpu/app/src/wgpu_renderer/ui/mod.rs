#![allow(deprecated)]
use crate::wgpu_renderer::ui::manager::{CurrentTool, UIManager};
use egui::{Button, DragValue, Pos2, Ui};

pub mod manager;

pub struct UI;

impl UI {
    pub fn new() -> Self {
        Self
    }

    pub fn main(&self, manager: &mut UIManager, ui: &mut Ui) {
        egui::Panel::top("top_panel")
            .resizable(false)
            .show_inside(ui, |ui| {
                self.draw_top_bar(manager, ui);
            });

        egui::Window::new("Configuration")
            .collapsible(true)
            .resizable(true)
            .default_pos(Pos2::new(50.0, 50.0))
            .show(ui.ctx(), |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.simulation_options(manager, ui);
                    self.visual_options(manager, ui);
                    self.entity_options(manager, ui);
                    self.tracing_options(manager, ui);
                    self.charge_creation(manager, ui);
                    self.controls(ui);
                });
            });
    }

    pub fn draw_top_bar(&self, manager: &mut UIManager, ui: &mut Ui) {
        ui.horizontal(|ui| {
            egui::ComboBox::from_label("Tool")
                .selected_text(manager.input_values.tool.to_string())
                .show_ui(ui, |ui| {
                    for tool in enum_iterator::all::<CurrentTool>() {
                        ui.selectable_value(&mut manager.input_values.tool, tool, tool.to_string());
                    }
                });

            if manager.input_values.tool == CurrentTool::Charge {
                self.drag_value(
                    ui,
                    &mut manager.input_values.charge_spawn_ui_options.charge,
                    "Current Charge (C)",
                );
            }

            ui.separator();

            ui.label(format!(
                "Time Scale: {}",
                manager.input_values.particle_ui_options.time_scale.0
            ));

            let paused = manager.input_values.particle_ui_options.time_scale.0 == 0.0;
            if ui
                .button(egui::RichText::new(if paused { "⏸" } else { "▶" }).size(16.0))
                .clicked()
            {
                manager.input_values.particle_ui_options.time_scale = if paused {
                    (manager.input_values.particle_ui_options.time_scale.1, 0.0)
                } else {
                    (0.0, manager.input_values.particle_ui_options.time_scale.0)
                }
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                self.button_toggle(
                    ui,
                    &mut manager.input_values.charge_spawn_ui_options.clear_charges,
                    "Remove all charges",
                );
                self.button_toggle(
                    ui,
                    &mut manager.input_values.particle_ui_options.clear_particles,
                    "Remove all particles",
                );
                //ui.separator();
            });
        });
    }

    pub fn simulation_options(&self, manager: &mut UIManager, ui: &mut Ui) {
        ui.collapsing("Simulation Settings", |ui| {
            ui.vertical(|ui| {
                self.drag_value(
                    ui,
                    &mut manager.input_values.particle_ui_options.time_scale.0,
                    "Time Scale",
                );
                self.drag_value(
                    ui,
                    &mut manager.input_values.particle_ui_options.drag_value,
                    "Environment Drag Value",
                );
                self.drag_value(
                    ui,
                    &mut manager
                        .input_values
                        .electric_ui_options
                        .charge_strength_scale,
                    "Charge Strength Scale",
                );
            });
        });
    }

    pub fn visual_options(&self, manager: &mut UIManager, ui: &mut Ui) {
        ui.collapsing("Display & Visuals", |ui| {
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
                    &mut manager.input_values.draw_ui_options.draw_normalised_vec,
                    "Normalise Vector Arrows",
                );

                ui.checkbox(
                    &mut manager.input_values.draw_ui_options.draw_field_lines,
                    "Draw Field Lines",
                );

                ui.checkbox(
                    &mut manager.input_values.draw_ui_options.draw_potential,
                    "Draw Equipotential Lines",
                );
                self.color_picker(
                    ui,
                    &mut manager
                        .input_values
                        .electric_ui_options
                        .equipotential_color_rgba,
                    "Equipotential Line Color",
                );

                self.drag_value(
                    ui,
                    &mut manager.input_values.color_value,
                    "Global Color Value",
                );
            });
        });
    }

    pub fn entity_options(&self, manager: &mut UIManager, ui: &mut Ui) {
        ui.collapsing("Entity Properties", |ui| {
            ui.vertical(|ui| {
                ui.label(egui::RichText::new("Particles").strong());
                self.drag_value(
                    ui,
                    &mut manager.input_values.particle_ui_options.particle_radius,
                    "Particle Radius",
                );
                self.drag_value(
                    ui,
                    &mut manager.input_values.particle_ui_options.polygon_vertices,
                    "Polygon Vertices",
                );

                ui.label(egui::RichText::new("Charges").strong());
                self.drag_value(
                    ui,
                    &mut manager.input_values.electric_ui_options.charge_radius,
                    "Charge Radius",
                );
            });
        });
    }

    pub fn tracing_options(&self, manager: &mut UIManager, ui: &mut Ui) {
        ui.collapsing("Field Line Tracing", |ui| {
            ui.vertical(|ui| {
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
            });
        });
    }

    pub fn charge_creation(&self, manager: &mut UIManager, ui: &mut Ui) {
        ui.collapsing("Manual Charge Spawning", |ui| {
            ui.vertical(|ui| {
                self.drag_value(
                    ui,
                    &mut manager.input_values.charge_spawn_ui_options.x,
                    "Spawn X",
                );
                self.drag_value(
                    ui,
                    &mut manager.input_values.charge_spawn_ui_options.y,
                    "Spawn Y",
                );
                self.drag_value(
                    ui,
                    &mut manager.input_values.charge_spawn_ui_options.charge,
                    "Charge (C)",
                );
                self.button_toggle(
                    ui,
                    &mut manager.input_values.charge_spawn_ui_options.spawn,
                    "Spawn Charge",
                );
            })
        });
    }

    pub fn controls(&self, ui: &mut Ui) {
        ui.collapsing("Controls", |ui| {
            ui.label("F10 to toggle menu");
            ui.label("F11 to toggle fullscreen");
            ui.label("X to switch signs of next charge");
            ui.label("Z to cycle through tools");
            ui.label("Ctrl+C to clear charges");
            ui.label("Shift+C to clear particles");
            ui.label("LMB to spawn");
            ui.label("RMB to remove");
        });
    }

    pub fn drag_value<T>(&self, ui: &mut Ui, value: &mut T, label: &str)
    where
        T: egui::emath::Numeric,
    {
        ui.horizontal(|ui| {
            ui.label(label);
            ui.add(DragValue::new(value).update_while_editing(false));
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
