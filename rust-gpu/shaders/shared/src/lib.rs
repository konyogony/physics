#![no_std]
#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]
use bytemuck::{Pod, Zeroable};
use glam::{Vec2, Vec3, Vec4};
use spirv_std::arch::Derivative;
use spirv_std::num_traits::Float;

pub mod sdf;
pub mod util;

// --- From Particle Shader ---

#[derive(Debug, Clone, Copy, Zeroable, Pod)]
#[repr(C)]
pub struct Particle {
    pub position: [f32; 2],
    pub velocity: [f32; 2],
    pub color: [f32; 3],
    pub _pad: f32,
}

#[derive(Default, Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct ParticleOptions {
    pub time_scale: f32,
    pub particle_radius: f32,
    pub polygon_vertices: u32,
    pub drag_value: f32,
}

pub const MAX_PARTICLES: u32 = 262144;

// --- From Electric Shader ---
// Softening factor
pub const EPSILON_SQ: f32 = 1.0;
pub const MAX_CHARGES: u32 = 100;
pub const MAX_PLATES: u32 = 10;
pub const DV: f32 = 1.0;
pub const H: i32 = 1;

#[derive(Debug, Clone, Copy, Zeroable, Pod)]
#[repr(C)]
pub struct Plate {
    pub edges: [f32; 8],
    pub potential: f32,
    pub _pad: [f32; 3],
}

#[derive(Debug, Clone, Copy, Zeroable, Pod)]
#[repr(C)]
pub struct Charge {
    pub position: [f32; 2],
    pub charge: f32,
    pub _pad: f32,
}

#[derive(Debug, Clone, Copy, Zeroable, Pod)]
#[repr(C)]
pub struct Field {
    pub field: [f32; 2],
    pub _pad: [f32; 2],
}

#[derive(Debug, Clone, Copy, Zeroable, Pod)]
#[repr(C)]
pub struct TracePoint {
    pub pos: [f32; 2],
}

#[derive(Debug, Clone, Copy, Zeroable, Pod)]
#[repr(C)]
pub struct Color4 {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Default for Color4 {
    fn default() -> Self {
        Self {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        }
    }
}

#[derive(Default, Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct ElectricOptions {
    pub charge_radius: f32,
    pub num_particles_per_charge: u32,
    pub max_steps: u32,
    pub step_size: f32,
    pub stop_distance: f32,
    pub charge_strength_scale: f32,
    //  cannot use arrays to pad uniform buffers
    pub _pad0: f32,
    pub _pad1: f32,
    pub equipotential_color_rgba: Color4,
}

// --- From Grid Shader ---

// colors RGBA
pub const GRID_COLOR: Vec4 = Vec4::new(0.3, 0.3, 0.3, 0.05);
pub const AXIS_COLOR: Vec4 = Vec4::new(1.0, 1.0, 1.0, 0.8);
pub const BG_COLOR: Vec4 = Vec4::new(0.0, 0.0, 0.0, 1.0);
pub const HIGHLIGHT_COLOR: Vec4 = Vec4::new(0.0, 1.0, 1.0, 0.4);
pub const GRID_THICKNESS_PX: f32 = 1.0;
pub const GRID_SPACING_PX: f32 = 50.0;
pub const ARROW_THICKNESS_PX: f32 = 1.0;
pub const ARROW_HEAD_WIDTH_PX: f32 = 4.0;
pub const ARROW_HEAD_HEIGHT_PX: f32 = 10.0;
pub const HIGHLIGHT_SQUARES: f32 = 3.0;
pub const ARROW_SCALE: f32 = 25.0;
pub const MIN_ARROW_SCALE: f32 = 0.7;

// --- General Code ---

// These consstants are also defined inside of the rust code and passed in as a storage buffer.
#[derive(Default, Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct ShaderConstants {
    pub width: u32,
    pub height: u32,
    pub aspect_ratio: f32,
    pub time: f32,
    pub dt: f32,
    pub num_particles: u32,
    pub epsilon_naught: f32,
    pub num_charges: u32,
    pub num_plates: u32,
    pub color_value: f32,
    //  cannot use arrays to pad uniform buffers
    pub _pad0: f32,
    pub _pad1: f32,
    pub draw_options: DrawOptions,
    pub particle_options: ParticleOptions,
    pub electric_options: ElectricOptions,
}

#[derive(Default, Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct DrawOptions {
    pub draw_grid: u32,
    pub draw_vec: u32,
    pub draw_potential: u32,
    pub draw_field_lines: u32,
    pub draw_normalised_vec: u32,
    //  cannot use arrays to pad uniform buffers
    pub _pad0: f32,
    pub _pad1: f32,
    pub _pad2: f32,
}
