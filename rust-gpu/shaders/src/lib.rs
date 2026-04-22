#![no_std]

// So this is the shader code itself, the fragment and vertex shaders are stored here.
// They can recieve inputs and give outputs by making them mutable and using pointers.
// No std librarires are allowed here.

use bytemuck::{Pod, Zeroable};
use glam::{Vec2, Vec3, Vec4, Vec4Swizzles};
use spirv_std::num_traits::Float;
use spirv_std::spirv;

const GRID_COLOR: Vec4 = Vec4::new(0.3, 0.3, 0.3, 0.05);
const AXIS_COLOR: Vec4 = Vec4::new(1.0, 1.0, 1.0, 0.8);
const HIGHLIGHT_COLOR: Vec4 = Vec4::new(0.0, 1.0, 1.0, 0.4);
const THICKNESS: f32 = 0.5;
const GRID_SPACING: f32 = 100.0;
const HIGHLIGHT_SQUARES: i32 = 3;

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
    let mut status = Status::None;

    let uv = frag_coords / Vec4::new(constants.width as f32, constants.height as f32, 0.0, 0.0);
    let centered_uv = uv - 0.5;

    let grid = (centered_uv * GRID_SPACING).fract();

    if (grid.x < THICKNESS && grid.x > -THICKNESS) || (grid.y < THICKNESS && grid.y > -THICKNESS) {
        status = Status::AxisLine
    }

    *output = match status {
        Status::GridLine => GRID_COLOR,
        Status::AxisLine => AXIS_COLOR,
        Status::HighlightLine => HIGHLIGHT_COLOR,
        Status::None => Vec4::new(0.0, 0.0, 0.0, 1.0),
    }
}
