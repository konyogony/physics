#![allow(deprecated)]
use crate::wgpu_renderer::ui::manager::{CurrentTool, PlateSpawnStage, UIManager};
use egui::{Button, Color32, DragValue, Pos2, Stroke, Ui};

const COMPONENT_NAMES: [&str; 4] = ["X", "Y", "Z", "W"];

pub mod manager;

pub struct UI;

impl UI {
    pub fn new() -> Self {
        Self
    }

    pub fn main(&self, manager: &mut UIManager, ui: &mut Ui) {
        self.show_potential_popup(manager, ui.ctx());

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
                    //self.charge_creation(manager, ui);
                    //self.plate_creation(manager, ui);
                    //self.controls(ui);
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

            match manager.input_values.tool {
                CurrentTool::SpawnCharge => {
                    self.drag_value(
                        ui,
                        &mut manager.input_values.charge_spawn_ui_options.charge,
                        "Current Charge (C)",
                    );
                }
                // CurrentTool::SpawnPlate => {
                //     self.drag_value(
                //         ui,
                //         &mut manager.input_values.plate_spawn_ui_options.potential,
                //         "Current Potential (V)",
                //     );
                // }
                CurrentTool::RemoveItem => {
                    ui.label("Click on item to remove");
                }
                _ => (),
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
                        .equipotential_spacing_mv,
                    "Equipotential ΔV (mV)",
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

    //pub fn plate_creation(&self, manager: &mut UIManager, ui: &mut Ui) {
    //    ui.collapsing("Manual Plate Creation", |ui| {
    //        ui.vertical(|ui| {
    //            self.vec::<2>(
    //                ui,
    //                &mut manager.input_values.plate_spawn_ui_options.p1,
    //                "Point 1",
    //                1.0,
    //            );
    //            self.vec::<2>(
    //                ui,
    //                &mut manager.input_values.plate_spawn_ui_options.p2,
    //                "Point 2",
    //                1.0,
    //            );
    //            self.vec::<2>(
    //                ui,
    //                &mut manager.input_values.plate_spawn_ui_options.p3,
    //                "Point 3",
    //                1.0,
    //            );
    //            self.vec::<2>(
    //                ui,
    //                &mut manager.input_values.plate_spawn_ui_options.p4,
    //                "Point 4",
    //                1.0,
    //            );
    //            self.drag_value(
    //                ui,
    //                &mut manager.input_values.plate_spawn_ui_options.potential,
    //                "Potential (V)",
    //            );
    //            self.button_toggle(
    //                ui,
    //                &mut manager.input_values.plate_spawn_ui_options.spawn,
    //                "Create",
    //            );
    //        })
    //    });
    //}

    //pub fn charge_creation(&self, manager: &mut UIManager, ui: &mut Ui) {
    //    ui.collapsing("Manual Charge Spawning", |ui| {
    //        ui.vertical(|ui| {
    //            self.drag_value(
    //                ui,
    //                &mut manager.input_values.charge_spawn_ui_options.x,
    //                "Spawn X",
    //            );
    //            self.drag_value(
    //                ui,
    //                &mut manager.input_values.charge_spawn_ui_options.y,
    //                "Spawn Y",
    //            );
    //            self.drag_value(
    //                ui,
    //                &mut manager.input_values.charge_spawn_ui_options.charge,
    //                "Charge (C)",
    //            );
    //            self.button_toggle(
    //                ui,
    //                &mut manager.input_values.charge_spawn_ui_options.spawn,
    //                "Spawn Charge",
    //            );
    //        })
    //    });
    //}

    pub fn show_potential_popup(&self, manager: &mut UIManager, ctx: &egui::Context) {
        let mut confirm_spawn = false;
        let mut cancel = false;

        if let Some(ref mut options) = manager.plate_spawn_options {
            if options.stage == PlateSpawnStage::ChoosePotential {
                egui::Window::new("Set Plate Potential")
                    .collapsible(false)
                    .resizable(true)
                    .movable(true)
                    .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                    .show(ctx, |ui| {
                        ui.horizontal(|ui| {
                            ui.label("Potential (V):");
                            ui.add(
                                DragValue::new(&mut options.potential)
                                    .speed(1.0)
                                    .update_while_editing(false),
                            );
                        });
                        ui.horizontal(|ui| {
                            if ui.button("Confirm").clicked() {
                                confirm_spawn = true;
                            }
                            if ui.button("Cancel").clicked() {
                                cancel = true;
                            }
                        });
                    });
            }
        }

        if confirm_spawn {
            if let Some(ref mut options) = manager.plate_spawn_options {
                options.stage = PlateSpawnStage::Confirmed;
            }
        } else if cancel {
            manager.plate_spawn_options = None;
        }
    }

    // previews

    pub fn draw_particle_preview(painter: &egui::Painter, vertex_radius: f32) {
        let mouse_pos = painter.ctx().pointer_latest_pos().unwrap_or(Pos2::ZERO);

        painter.circle_filled(
            mouse_pos,
            vertex_radius / 1.5,
            Color32::from_rgb(52, 103, 255),
        );
    }

    pub fn draw_charge_preview(painter: &egui::Painter, vertex_radius: f32, charge: f32) {
        let mouse_pos = painter.ctx().pointer_latest_pos().unwrap_or(Pos2::ZERO);

        let line_color = if charge < 0.0 {
            Color32::from_rgb(0, 255, 255)
        } else {
            Color32::from_rgb(255, 128, 0)
        };
        painter.circle_filled(mouse_pos, vertex_radius / 1.5, line_color);

        let text_pos = mouse_pos + egui::vec2(vertex_radius, -vertex_radius);
        let label = format!("{:.1} C", charge);
        painter.text(
            text_pos,
            egui::Align2::CENTER_CENTER,
            label,
            egui::FontId::proportional(11.0 * (vertex_radius / 15.0)),
            egui::Color32::WHITE,
        );
    }

    pub fn draw_plate_preview(manager: &mut UIManager, painter: &egui::Painter, ppp: f32) {
        let line_color = Color32::from_rgb(0, 210, 255);
        let stroke = Stroke::new(2.0_f32, line_color);
        let preview_stroke =
            Stroke::new(1.5_f32, Color32::from_rgba_unmultiplied(0, 210, 255, 160));
        let fill_color = Color32::from_rgba_unmultiplied(0, 180, 255, 35);
        let vertex_radius = 4.0;

        let mouse_pos = painter.ctx().pointer_latest_pos().unwrap_or(Pos2::ZERO);

        painter.circle_filled(mouse_pos, vertex_radius, line_color);

        let Some(ref options) = manager.plate_spawn_options else {
            return;
        };

        let to_pos2 = |v: egui::Vec2| -> Pos2 { Pos2::new(v.x / ppp, v.y / ppp) };
        let p0 = to_pos2(options.vertices[0]);
        let p1 = to_pos2(options.vertices[1]);
        let p2 = to_pos2(options.vertices[2]);
        let p3 = to_pos2(options.vertices[3]);

        match options.stage {
            PlateSpawnStage::Point1 => {
                painter.circle_filled(p0, vertex_radius, line_color);
                painter.line_segment([p0, mouse_pos], preview_stroke);
            }
            PlateSpawnStage::Point2 => {
                painter.circle_filled(p0, vertex_radius, line_color);
                painter.circle_filled(p1, vertex_radius, line_color);

                painter.line_segment([p0, p1], stroke);
                painter.line_segment([p1, mouse_pos], preview_stroke);
            }
            PlateSpawnStage::Point3 => {
                let quad = [p0, p1, p2, mouse_pos];

                painter.add(egui::Shape::convex_polygon(
                    quad.to_vec(),
                    fill_color,
                    Stroke::NONE,
                ));

                painter.line_segment([p0, p1], stroke);
                painter.line_segment([p1, p2], stroke);
                painter.line_segment([p2, mouse_pos], preview_stroke);
                painter.line_segment([mouse_pos, p0], preview_stroke);

                for &p in &[p0, p1, p2] {
                    painter.circle_filled(p, vertex_radius, line_color);
                }
            }
            PlateSpawnStage::ChoosePotential | PlateSpawnStage::Confirmed => {
                let quad = [p0, p1, p2, p3];

                painter.add(egui::Shape::convex_polygon(
                    quad.to_vec(),
                    fill_color,
                    stroke,
                ));

                for &p in &quad {
                    painter.circle_filled(p, vertex_radius, line_color);
                }
            }
            _ => {}
        }
    }

    //pub fn controls(&self, ui: &mut Ui) {
    //    ui.collapsing("Controls", |ui| {
    //        ui.label("F10 to toggle menu");
    //        ui.label("F11 to toggle fullscreen");
    //        ui.label("X to switch signs of next charge");
    //        ui.label("Z to cycle through tools");
    //        ui.label("Ctrl+C to clear charges");
    //        ui.label("Shift+C to clear particles");
    //        ui.label("LMB to spawn");
    //        ui.label("RMB to remove");
    //    });
    //}

    // Kinda like the re-usuable components

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

    pub fn vec<const N: usize>(&self, ui: &mut Ui, values: &mut [f32; N], label: &str, speed: f32) {
        ui.horizontal(|ui| {
            ui.label(label);
            for (i, val) in values.iter_mut().enumerate() {
                if i < COMPONENT_NAMES.len() {
                    ui.add(
                        DragValue::new(val)
                            .prefix(format!("{}: ", COMPONENT_NAMES[i]))
                            .speed(speed),
                    );
                } else {
                    ui.add(DragValue::new(val).prefix(format!("#{}: ", i)).speed(speed));
                }
            }
        });
    }
}
