#![no_std]

// So this is the shader code itself, the fragment and vertex shaders are stored here.
// They can recieve inputs and give outputs by making them mutable and using pointers.
// No std librarires are allowed here.

use bytemuck::{Pod, Zeroable};
use core::f32::consts::PI;
use glam::{Vec3, Vec4, vec2, vec3};
#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;
use spirv_std::spirv;

// These consstants are also defined inside of the rust code and passed in as a storage buffer.
#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct ShaderConstants {
    pub width: u32,
    pub height: u32,
    pub time: f32,
}

#[spirv(vertex)]
pub fn main_vs(
    // We get the input for vertex id, its constants?, and its position + color.
    #[spirv(vertex_index)] vert_id: i32,
    // This is the bind group that we have created, with the 0th index binding being our shader
    // constants.
    #[spirv(descriptor_set = 0, binding = 0, storage_buffer)] constants: &ShaderConstants,
    #[spirv(position)] vtx_pos: &mut Vec4,
    vtx_color: &mut Vec3,
) {
    let speed = 0.4;
    // Just varying the position of the triangle with time
    let time = constants.time * speed + vert_id as f32 * (2.0 * PI * 120.0 / 360.0);
    let position = vec2(f32::sin(time), f32::cos(time));

    *vtx_pos = Vec4::from((position, 0.0, 1.0));
    *vtx_color = [
        vec3(1.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        vec3(0.0, 0.0, 1.0),
    ][vert_id as usize % 3];
}

#[spirv(fragment)]
pub fn main_fs(vtx_color: Vec3, output: &mut Vec4) {
    // Output on the fragment shader will be the vertex color w alpha of 1
    *output = Vec4::from((vtx_color, 1.0));
}
