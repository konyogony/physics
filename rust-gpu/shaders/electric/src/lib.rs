#![no_std]
// Seperate shader for the particles
#![allow(clippy::too_many_arguments)]

use core::f32::consts::PI;
use glam::{UVec3, Vec2};
use shaders_shared::{Charge, Field, H, ShaderConstants};
#[allow(unused_imports)]
use spirv_std::num_traits::Float;
use spirv_std::spirv;

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
        let r = (current_coords - charge_coords).length().max(0.001);
        potential += q / r;
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
    // Then by applying coulombs law, we know that 𝐄⃗=∇⃗φ
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
    };

    electric_field[index as usize] = field
}
