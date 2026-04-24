#![no_std]

// So this is the shader code itself, the fragment and vertex shaders are stored here.
// They can recieve inputs and give outputs by making them mutable and using pointers.
// No std librarires are allowed here.

use bytemuck::{Pod, Zeroable};
use glam::{Vec2, Vec4, Vec4Swizzles};
use spirv_std::arch::Derivative;
use spirv_std::spirv;

const GRID_COLOR: Vec4 = Vec4::new(0.3, 0.3, 0.3, 0.05);
const AXIS_COLOR: Vec4 = Vec4::new(1.0, 1.0, 1.0, 0.8);
const BG_COLOR: Vec4 = Vec4::new(0.0, 0.0, 0.0, 1.0);
const HIGHLIGHT_COLOR: Vec4 = Vec4::new(0.0, 1.0, 1.0, 0.4);
const THICKNESS: f32 = 1.0;
const GRID_SPACING: f32 = 30.0;
const HIGHLIGHT_SQUARES: f32 = 3.0;

// These consstants are also defined inside of the rust code and passed in as a storage buffer.
#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct ShaderConstants {
    pub width: u32,
    pub height: u32,
    pub aspect_ratio: f32,
    pub time: f32,
}

#[spirv(vertex)]
pub fn main_vs(#[spirv(vertex_index)] vert_id: i32, #[spirv(position)] vtx_pos: &mut Vec4) {
    // fancy bitwise manipulations
    let uv = Vec2::new(((vert_id << 1) & 2) as f32, (vert_id & 2) as f32);
    // Mapping to the correct range
    let pos = Vec2::new(uv.x * 2.0 - 1.0, uv.y * 2.0 - 1.0);
    // Basically, we are covering the entire screen here.
    *vtx_pos = pos.extend(0.0).extend(1.0);
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Status {
    GridLine,
    AxisLine,
    HighlightLine,
    None,
}

#[spirv(fragment)]
pub fn main_fs(
    #[spirv(descriptor_set = 0, binding = 0, storage_buffer)] constants: &ShaderConstants,
    #[spirv(frag_coord)] frag_coords: Vec4,
    output: &mut Vec4,
) {
    // Store the initial rgb values.
    let output_color = BG_COLOR.xyz();

    // Center first
    let uv = frag_coords / Vec4::new(constants.width as f32, constants.height as f32, 0.0, 0.0);
    let centered_uv = uv - 0.5;

    // Convert to pixels
    let px_x = centered_uv.x * constants.width as f32;
    let px_y = centered_uv.y * constants.height as f32;

    // Get how many times spacing wraps.
    let grid_distance_x = (px_x % GRID_SPACING).abs();
    let grid_distance_y = (px_y % GRID_SPACING).abs();
    // Get closest one
    let grid_distance = grid_distance_x.min(grid_distance_y);
    // Make sure lines dont look ugly and appear on all screen sizes.
    let grid_alpha = antialias(grid_distance, THICKNESS);

    // Same for highlights, but different scale
    let highlight_distance_x = (px_x % (GRID_SPACING * HIGHLIGHT_SQUARES)).abs();
    let highlight_distance_y = (px_y % (GRID_SPACING * HIGHLIGHT_SQUARES)).abs();
    let highlight_distance = highlight_distance_x.min(highlight_distance_y);
    let highlight_alpha = antialias(highlight_distance, THICKNESS);

    let axis_distance = px_x.abs().min(px_y.abs());
    let axis_alpha = antialias(axis_distance, THICKNESS);

    // Now the alpha channels are applied SEPERATLY to preserve the original alpha
    // Lerp allows us to apply a mask with specific colors.
    *output = output_color
        .lerp(GRID_COLOR.xyz(), grid_alpha * GRID_COLOR.w)
        .lerp(HIGHLIGHT_COLOR.xyz(), highlight_alpha * HIGHLIGHT_COLOR.w)
        .lerp(AXIS_COLOR.xyz(), axis_alpha * AXIS_COLOR.w)
        .extend(1.0);
}

// Utilitiy functions

pub fn antialias(dist: f32, thickness: f32) -> f32 {
    let edge = dist.fwidth();
    1.0 - smoothstep(thickness - edge, thickness + edge, dist)
}

fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}
