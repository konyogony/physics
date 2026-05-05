#![no_std]
// Seperate shader for the particles
#![allow(clippy::too_many_arguments)]

use core::f32::consts::PI;
use glam::{UVec3, Vec2, Vec3, Vec4};
use shaders_shared::{
    CHARGE_RADIUS, Charge, EPSILON_SQ, Field, H, POLYGON_VERTICES, ShaderConstants,
};
#[allow(unused_imports)]
use spirv_std::num_traits::Float;
use spirv_std::spirv;

// EXACT SAME CODE AS IN PARTICLE, JUST ADAPTED FOR CHARGES NOW
#[spirv(vertex(entry_point_name = "electric_vs"))]
pub fn electric_vs(
    #[spirv(vertex_index)] vtx_id: i32,
    #[spirv(instance_index)] instance_id: i32,
    #[spirv(position)] vtx_pos: &mut Vec4,
    #[spirv(descriptor_set = 0, binding = 0, storage_buffer)] constants: &ShaderConstants,
    #[spirv(descriptor_set = 1, binding = 0, storage_buffer)] charges: &[Charge],
    #[spirv(location = 0)] vtx_color: &mut Vec3,
) {
    let charge = charges[instance_id as usize];
    let center: Vec2 = charge.position.into();

    let num_segments = POLYGON_VERTICES / 3;
    let triangle_id = vtx_id / 3;
    let corner_id = vtx_id % 3;

    let local_offset = if corner_id == 0 {
        Vec2::ZERO
    } else {
        let angle_increment = (2.0 * PI) / num_segments as f32;
        let angle_offset = (triangle_id as f32 + (corner_id - 1) as f32) * angle_increment;
        Vec2::new(
            CHARGE_RADIUS * angle_offset.cos(),
            CHARGE_RADIUS * angle_offset.sin(),
        )
    };

    let pos_px = center + local_offset;
    let pos_uv = Vec2::new(
        (pos_px.x / constants.width as f32) * 2.0 - 1.0,
        (pos_px.y / constants.height as f32) * -2.0 + 1.0,
    );

    *vtx_pos = pos_uv.extend(0.0).extend(1.0);
    // For charges, we check if its negative or positive and apply color accordingly.
    if charge.charge < 0.0 {
        *vtx_color = Vec3::new(0.0, 1.0, 1.0);
    } else {
        *vtx_color = Vec3::new(1.0, 0.5, 0.0);
    }
}

#[spirv(fragment(entry_point_name = "electric_fs"))]
pub fn electric_fs(#[spirv(location = 0)] vtx_color: Vec3, output: &mut Vec4) {
    *output = vtx_color.extend(1.0);
}

// This will be ran for every pixel on the screen once.
#[spirv(compute(threads(16, 16), entry_point_name = "electric_potential_cs"))]
pub fn electric_potential_cs(
    #[spirv(global_invocation_id)] global_invocation_id: UVec3,
    #[spirv(descriptor_set = 0, binding = 0, storage_buffer)] constants: &ShaderConstants,
    // No more textures. ONLY buffers.
    #[spirv(descriptor_set = 1, binding = 0, storage_buffer)] charges: &[Charge],
    #[spirv(descriptor_set = 1, binding = 1, storage_buffer)] electric_potential: &mut [f32],
) {
    let x = global_invocation_id.x as usize;
    let y = global_invocation_id.y as usize;
    let index = x + y * constants.width as usize;

    if x >= constants.width as usize || y >= constants.height as usize {
        return;
    }

    let current_coords = Vec2::new(x as f32, y as f32);
    let mut potential = 0.0;

    let k = 1.0 / (4.0 * PI * constants.epsilon_naught);
    for charge in 0..constants.num_charges {
        let charge = charges[charge as usize];
        let charge_pos = charge.position;
        // since not centered js remove the centering hjere aswell
        let charge_coords = Vec2::new(charge_pos[0], charge_pos[1]);

        let q = charge.charge;
        let r = (current_coords - charge_coords).length();
        // Usually potential is q / r, however for simulation purposes so that test charges dont
        // explode, we will use q / sqrt(r^2 + epsilon^2)
        potential += q / (r + EPSILON_SQ).sqrt();
    }

    let final_potential = potential * k;

    electric_potential[index] = final_potential;
}

#[spirv(compute(threads(16, 16), entry_point_name = "electric_field_cs"))]
pub fn electric_field_cs(
    #[spirv(global_invocation_id)] global_invocation_id: UVec3,
    #[spirv(descriptor_set = 0, binding = 0, storage_buffer)] constants: &ShaderConstants,
    #[spirv(descriptor_set = 1, binding = 1, storage_buffer)] electric_potential: &mut [f32],
    #[spirv(descriptor_set = 1, binding = 2, storage_buffer)] electric_field: &mut [Field],
) {
    // Method of central differences to get gradient at any single point.
    // f'(x) = (f(x+h) - f(x-h)) / 2h
    // Then by applying coulombs law, we know that 𝐄⃗=-∇⃗φ
    // E = -< ∂φ / ∂x, ∂φ / ∂y>
    let x = global_invocation_id.x as i32;
    let y = global_invocation_id.y as i32;
    let index = x + y * constants.width as i32;

    if x >= constants.width as i32 || y >= constants.height as i32 {
        return;
    }

    let max_index = constants.width as i32 * constants.height as i32;

    let up_index = (index + H * constants.width as i32).min(max_index - 1);
    let down_index = (index - H * constants.width as i32).max(0);
    let right_index = (index + H).min(max_index - 1);
    let left_index = (index - H).max(0);

    //     if left_index < 0 || down_index < 0 || right_index > max_index || up_index > max_index {
    //         return;
    //     }

    let up_sample = electric_potential[up_index as usize];
    let down_sample = electric_potential[down_index as usize];
    let left_sample = electric_potential[left_index as usize];
    let right_sample = electric_potential[right_index as usize];

    // make it signed.
    let d_dx = (right_sample - left_sample) / (2.0 * H as f32);
    let d_dy = (up_sample - down_sample) / (2.0 * H as f32);

    let field = Field {
        field: [-d_dx, -d_dy],
        _pad: [0.0; 2],
    };

    electric_field[index as usize] = field
}
