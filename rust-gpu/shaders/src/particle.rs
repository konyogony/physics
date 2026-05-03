// Seperate shader for the particles
#![allow(clippy::too_many_arguments)]

use crate::Field;
use crate::shared::ShaderConstants;
use bytemuck::{Pod, Zeroable};
use core::f32::consts::PI;
use glam::{UVec3, Vec2, Vec3, Vec4};
#[allow(unused_imports)]
use spirv_std::num_traits::Float;
use spirv_std::spirv;

#[derive(Debug, Clone, Copy, Zeroable, Pod)]
#[repr(C)]
pub struct Particle {
    pub position: [f32; 2],
    pub velocity: [f32; 2],
    pub color: [f32; 3],
    pub _pad: f32,
}

pub const TIME_SCALE: f32 = 20.0;
pub const MAX_PARTICLES: u32 = 262144;
pub const PARTICLE_RADIUS: f32 = 10.0;
pub const POLYGON_VERTICES: u32 = 48;

// Say each particle will consist of 6 vertices. (Small square / quad)
// For each group of 6 vertices we have to displace them first by the particle position
// and then displace each individual vertex based on its relational position
#[spirv(vertex(entry_point_name = "particle_vs"))]
pub fn particle_vs(
    // Tells us which vertex inside that group we are doing
    #[spirv(vertex_index)] vtx_id: i32,
    // Tells us which group of vertices we are doing
    #[spirv(instance_index)] instance_id: i32,
    #[spirv(position)] vtx_pos: &mut Vec4,
    #[spirv(descriptor_set = 0, binding = 0, storage_buffer)] constants: &ShaderConstants,
    #[spirv(descriptor_set = 1, binding = 0, storage_buffer)] particles: &[Particle],
    #[spirv(location = 0)] vtx_color: &mut Vec3,
) {
    // Extract which particle we are doing based on group index
    let particle = particles[instance_id as usize];
    let center: Vec2 = particle.position.into();

    let num_segments = POLYGON_VERTICES / 3;
    let triangle_id = vtx_id / 3;
    let corner_id = vtx_id % 3;

    let local_offset = if corner_id == 0 {
        Vec2::ZERO
    } else {
        let angle_increment = (2.0 * PI) / num_segments as f32;
        let angle_offset = (triangle_id as f32 + (corner_id - 1) as f32) * angle_increment;
        Vec2::new(
            PARTICLE_RADIUS * angle_offset.cos(),
            PARTICLE_RADIUS * angle_offset.sin(),
        )
    };

    // Extract the offset based on the individual vertex index
    let pos_px = center + local_offset;

    // Conver to propper coordinates (NDC)
    let pos_uv = Vec2::new(
        (pos_px.x / constants.width as f32) * 2.0 - 1.0,
        (pos_px.y / constants.height as f32) * -2.0 + 1.0,
    );

    // Apply the position
    *vtx_pos = pos_uv.extend(0.0).extend(1.0);
    *vtx_color = particle.color.into();
}

#[spirv(fragment(entry_point_name = "particle_fs"))]
pub fn particle_fs(#[spirv(location = 0)] vtx_color: Vec3, output: &mut Vec4) {
    *output = vtx_color.extend(1.0);
}

#[spirv(compute(threads(256), entry_point_name = "particle_cs"))]
pub fn particle_cs(
    // The absolute index of the current data piece
    #[spirv(global_invocation_id)] global_invocation_id: UVec3,
    #[spirv(descriptor_set = 0, binding = 0, storage_buffer)] constants: &ShaderConstants,
    #[spirv(descriptor_set = 1, binding = 0, storage_buffer)] input: &[Particle],
    #[spirv(descriptor_set = 1, binding = 1, storage_buffer)] output: &mut [Particle],
    #[spirv(descriptor_set = 2, binding = 2, storage_buffer)] electric_field: &mut [Field],
) {
    // Extract the index using the invocation id
    let particle_index = global_invocation_id.x as usize;
    // Do math only if its within the range of particles that actually exist
    if particle_index < constants.num_particles as usize {
        let mut particle = input[particle_index];
        let index = particle.position[0] + particle.position[1] * constants.width as f32;
        // Calculate the velocity of the particle at its specific point in space & time.
        let velocity = electric_field[index as usize].field;
        // Apply that velocity
        particle.position[0] += velocity[0] * constants.dt * TIME_SCALE;
        particle.position[1] -= velocity[1] * constants.dt * TIME_SCALE;

        // Not to lose data, we create mut var, and we assign whole particle to the output.
        output[particle_index] = particle;
    }
}

// Guide on compute shaders
//
// Absolute index of data piece
// #[spirv(global_invocation_id)] global_invocation_id: UVec3,
//
// Within that 256 thread work group, which one im in
// #[spirv(local_invocation_id)] local_invocation_id: UVec3,
//
// More hardware related, id inside the hardware cluster
// #[spirv(subgroup_local_invocation_id)] subgroup_local_invocation_id: u32,
//
// 512 / 256 = 2 -> tell us which subdisivison
// #[spirv(workgroup_id)] workgroup_id: UVec3,
//
// Which part of workgroup
// #[spirv(subgroup_id)] subgroup_id: u32,
//
// How many parts exist
// #[spirv(num_subgroups)] num_subgroups: u32,
//
// Storage buffers that we can pass in
// #[spirv(descriptor_set = 0, binding = 0, storage_buffer)] constants: &ShaderConstants,
// #[spirv(descriptor_set = 0, binding = 1, storage_buffer)] input: &[ParticleUniform],
// #[spirv(descriptor_set = 0, binding = 2, storage_buffer)] output: &mut [ParticleUniform],
//
// Shared memory within a workgroup, only between those 256 items
// #[spirv(workgroup)] shared: &mut [u32; 256],
