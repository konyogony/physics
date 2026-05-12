use wgpu::{CommandEncoder, Device, Queue, SurfaceConfiguration, TextureFormat};
use winit::{event::WindowEvent, window::Window};

use crate::wgpu_renderer::ui::ui::UI;

pub const DEFAULT_CONFIG: InputValues = InputValues {
    draw_grid: true,
    draw_vec: true,
    draw_potential: false,
    draw_field_lines: false,
    color_value: 5.0,
    time_scale: 1.0,
    particle_radius: 10.0,
    polygon_vertices: 48,
    charge_radius: 15.0,
    num_particles_per_charge: 60,
    max_steps: 1100,
    step_size: 3.0,
    stop_distance: 10.0,
    charge_strength: 0.5,
};

#[derive(Default, Clone, Copy)]
pub struct InputValues {
    pub draw_grid: bool,
    pub draw_vec: bool,
    pub draw_potential: bool,
    pub draw_field_lines: bool,
    pub color_value: f32,
    pub time_scale: f32,
    pub particle_radius: f32,
    pub polygon_vertices: u32,
    pub charge_radius: f32,
    pub num_particles_per_charge: u32,
    pub max_steps: usize,
    pub step_size: f32,
    pub stop_distance: f32,
    pub charge_strength: f32,
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
            input_values: DEFAULT_CONFIG,
            committed_input_values: DEFAULT_CONFIG,
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

        let output = context.run_ui(raw_input, |ctx| {
            UI::new().main(self, ctx);
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
