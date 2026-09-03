use crate::wgpu_renderer::ui::ui::UI;
use enum_iterator::Sequence;
use strum_macros::Display;
use wgpu::{CommandEncoder, Device, Queue, SurfaceConfiguration, TextureFormat};
use winit::{event::WindowEvent, window::Window};

#[derive(Clone, Copy, PartialEq, Eq, Default, Sequence, Display)]
pub enum CurrentTool {
    #[strum(to_string = "Spawn Test Charge")]
    Particle,
    #[default]
    #[strum(to_string = "Spawn Charge")]
    Charge,
}

#[derive(Clone, Copy, PartialEq)]
pub struct InputValues {
    pub draw_ui_options: DrawUIOptions,
    pub particle_ui_options: ParticleUIOptions,
    pub electric_ui_options: ElectricUIOptions,
    pub charge_spawn_ui_options: ChargeSpawnUIOptions,
    pub tool: CurrentTool,
    pub color_value: f32,
}

impl Default for InputValues {
    fn default() -> Self {
        Self {
            draw_ui_options: DrawUIOptions {
                draw_grid: true,
                draw_vec: true,
                draw_potential: false,
                draw_field_lines: false,
                draw_normalised_vec: true,
            },
            particle_ui_options: ParticleUIOptions {
                time_scale: (1.0, 0.0),
                particle_radius: 10.0,
                polygon_vertices: 72,
            },
            electric_ui_options: ElectricUIOptions {
                charge_radius: 15.0,
                num_particles_per_charge: 60,
                max_steps: 550,
                step_size: 3.0,
                stop_distance: 14.5,
                charge_strength_scale: 1.0,
                equipotential_color_rgba: [0.0, 0.545, 0.545, 1.0],
            },
            charge_spawn_ui_options: ChargeSpawnUIOptions {
                x: 0.0,
                y: 0.0,
                charge: 1.0,
                spawn: false,
            },
            tool: CurrentTool::default(),
            color_value: 5.0,
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq)]
pub struct DrawUIOptions {
    pub draw_grid: bool,
    pub draw_vec: bool,
    pub draw_potential: bool,
    pub draw_field_lines: bool,
    pub draw_normalised_vec: bool,
}

// current, previous
pub type TimeScale = (f32, f32);

#[derive(Default, Clone, Copy, PartialEq)]
pub struct ParticleUIOptions {
    pub time_scale: TimeScale,
    pub particle_radius: f32,
    pub polygon_vertices: u32,
}

#[derive(Default, Clone, Copy, PartialEq)]
pub struct ChargeSpawnUIOptions {
    // These two r gonna be relative to center of screen, and then in state we move them to correct
    // position in accordance to the current screen size.
    pub x: f32,
    pub y: f32,
    // For now we are only limiting to charges with same strength, but only neg / pos
    pub charge: f32,
    pub spawn: bool,
}

#[derive(Default, Clone, Copy, PartialEq)]
pub struct ElectricUIOptions {
    pub charge_radius: f32,
    pub num_particles_per_charge: u32,
    pub max_steps: usize,
    pub step_size: f32,
    pub stop_distance: f32,
    pub charge_strength_scale: f32,
    pub equipotential_color_rgba: [f32; 4],
}

pub struct UIManager {
    pub active: bool,
    pub state: egui_winit::State,
    pub renderer: egui_wgpu::Renderer,
    pub screen_descriptor: egui_wgpu::ScreenDescriptor,
    pub input_values: InputValues,
    pub committed_input_values: InputValues,
    pub clipped_primitives: Vec<egui::ClippedPrimitive>,
}

const VIEWPORT_ID: egui::ViewportId = egui::ViewportId::ROOT;

impl UIManager {
    pub fn new(
        window: &Window,
        device: &Device,
        config: &SurfaceConfiguration,
        out_format: TextureFormat,
    ) -> Self {
        let context = egui::Context::default();
        let state = egui_winit::State::new(context.clone(), VIEWPORT_ID, window, None, None, None);

        let renderer_options = egui_wgpu::RendererOptions {
            msaa_samples: 1,
            dithering: true,
            depth_stencil_format: None,
            predictable_texture_filtering: true,
        };
        let renderer = egui_wgpu::Renderer::new(device, out_format, renderer_options);

        let screen_descriptor = egui_wgpu::ScreenDescriptor {
            size_in_pixels: [config.width, config.height],
            pixels_per_point: window.scale_factor() as f32,
        };

        Self {
            active: true,
            renderer,
            state,
            screen_descriptor,
            input_values: InputValues::default(),
            committed_input_values: InputValues::default(),
            clipped_primitives: Vec::new(),
        }
    }

    pub fn cycle_tool(&mut self) {
        let mut iter = enum_iterator::all::<CurrentTool>().cycle();
        iter.find(|i| i == &self.input_values.tool);
        self.input_values.tool = iter.next().unwrap_or_default();
    }

    pub fn toggle_active(&mut self) {
        self.active = !self.active
    }

    pub fn resize(&mut self, window: &Window, config: &SurfaceConfiguration) {
        self.screen_descriptor = egui_wgpu::ScreenDescriptor {
            size_in_pixels: [config.width, config.height],
            pixels_per_point: window.scale_factor() as f32,
        };
    }

    pub fn handle_event(&mut self, window: &Window, event: &WindowEvent) -> bool {
        self.state.on_window_event(window, event).consumed
    }

    pub fn prepare(
        &mut self,
        window: &Window,
        device: &Device,
        queue: &Queue,
        encoder: &mut CommandEncoder,
        charges: &[(egui::Pos2, f32)],
        radius: f32,
    ) {
        let raw_input = self.state.take_egui_input(window);
        let context = self.state.egui_ctx().clone();
        let ppp = self.screen_descriptor.pixels_per_point;

        let output = context.run_ui(raw_input, |ctx| {
            UI::new().main(self, ctx);

            // Awesome way to draw text, was easier than wgpu_text
            let painter = ctx.layer_painter(egui::LayerId::new(
                egui::Order::Foreground,
                egui::Id::new("charge_labels"),
            ));

            for (pos, charge) in charges {
                let screen_pos = egui::pos2(pos.x / ppp, pos.y / ppp);
                let label = format!("{:.1} C", charge);
                painter.text(
                    screen_pos,
                    egui::Align2::CENTER_CENTER,
                    label,
                    egui::FontId::proportional(11.0 * (radius / 15.0)),
                    egui::Color32::BLACK,
                );
            }
        });

        self.state
            .handle_platform_output(window, output.platform_output);

        for (id, image_delta) in &output.textures_delta.set {
            self.renderer
                .update_texture(device, queue, *id, image_delta);
        }

        for id in &output.textures_delta.free {
            self.renderer.free_texture(id);
        }

        self.clipped_primitives =
            context.tessellate(output.shapes, self.screen_descriptor.pixels_per_point);

        self.renderer.update_buffers(
            device,
            queue,
            encoder,
            &self.clipped_primitives,
            &self.screen_descriptor,
        );
    }

    pub fn draw(&mut self, rpass: &mut wgpu::RenderPass<'static>) {
        self.renderer
            .render(rpass, &self.clipped_primitives, &self.screen_descriptor);
    }
}
