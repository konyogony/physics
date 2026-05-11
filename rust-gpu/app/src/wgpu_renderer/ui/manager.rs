use egui::{Context, DragValue};
use wgpu::{CommandEncoder, Device, Queue, SurfaceConfiguration, TextureFormat};
use winit::{event::WindowEvent, window::Window};

pub const DEFAULT_CONFIG: InputValues = InputValues {
    draw_grid: true,
    draw_vec: true,
    draw_potential: false,
    draw_field_lines: false,
    color_value: 5.0,
};

#[derive(Default, Clone, Copy)]
pub struct InputValues {
    pub draw_grid: bool,
    pub draw_vec: bool,
    pub draw_potential: bool,
    pub draw_field_lines: bool,
    pub color_value: f32,
}

pub struct UIManager {
    pub active: bool,
    pub state: egui_winit::State,
    pub renderer: egui_wgpu::Renderer,
    pub screen_descriptor: egui_wgpu::ScreenDescriptor,
    pub input_values: InputValues,
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
            input_values: DEFAULT_CONFIG,
            clipped_primitives: Vec::new(),
        }
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
    ) {
        let raw_input = self.state.take_egui_input(window);
        let context = self.state.egui_ctx().clone();

        let output = context.run_ui(raw_input, |ctx| self.ui(ctx));

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

    pub fn ui(&mut self, ctx: &Context) {
        egui::Window::new("Configuration")
            .collapsible(true)
            .resizable(true)
            .default_width(400.0)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.checkbox(&mut self.input_values.draw_grid, "Draw Grid");
                        ui.checkbox(&mut self.input_values.draw_vec, "Draw Vector Arrows");
                        ui.checkbox(
                            &mut self.input_values.draw_potential,
                            "Draw Equipotential Lines",
                        );
                        ui.checkbox(&mut self.input_values.draw_field_lines, "Draw Field Lines");
                    });
                    ui.horizontal(|ui| {
                        ui.add(DragValue::new(&mut self.input_values.color_value));
                        ui.label("Color Value");
                    });
                    ui.collapsing("Controls", |ui| {
                        ui.label("F10 to toggle menu");
                        ui.label("F11 to toggle fullscreen");
                        ui.label("X to switch charge");
                        ui.label("Ctrl+C to clear charges");
                        ui.label("Shift+C to clear particles");
                    });
                });
            });
    }
}
