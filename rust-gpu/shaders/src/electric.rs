// Seperate shader for the particles
#![allow(clippy::too_many_arguments)]

use crate::shared::ShaderConstants;
use bytemuck::{Pod, Zeroable};
use core::f32::consts::PI;
use glam::{IVec2, UVec2, UVec3, Vec2};
#[allow(unused_imports)]
use spirv_std::num_traits::Float;
use spirv_std::{Image, spirv};

// TODO:
// CONVERT EVERYTHING FROM A DENSITY TEXTURE TO JUST DIRECT SUMMATION OVER EVERY CHARGE.

#[derive(Debug, Clone, Copy, Zeroable, Pod)]
#[repr(C)]
pub struct Charge {
    pub charge: f32,
    pub position: [f32; 2],
}

// Creating storage textures.
// NO SAMPLER NEEDED YAY!!!
pub type ElectricPotential = Image!(2D, format = r32f, sampled = false);
pub type ElectricField = Image!(2D, format = rgba32f, sampled = false);

pub const DV: f32 = 1.0;
pub const H: u32 = 1;

// This will be ran for every pixel on the screen once.
#[spirv(compute(threads(16, 16), entry_point_name = "electric_potential_cs"))]
pub fn electric_potential_cs(
    #[spirv(global_invocation_id)] global_invocation_id: UVec3,
    #[spirv(descriptor_set = 0, binding = 0, storage_buffer)] constants: &ShaderConstants,
    // Let this be a texture so we can use its UV coordinates. This theoretically should have a fixed size
    #[spirv(descriptor_set = 1, binding = 0, storage_buffer)] charges: &[Charge],
    // Electric potential input
    #[spirv(descriptor_set = 1, binding = 1)] electric_potential: &ElectricPotential,
) {
    let coords = global_invocation_id.truncate();
    let current_coords = Vec2::new(
        coords.x as f32 - (constants.width as f32 / 2.0),
        coords.y as f32 - (constants.height as f32 / 2.0),
    );
    let mut potential = 0.0;

    let k = 1.0 / (4.0 * PI * constants.epsilon_naught);

    for charge in 0..constants.num_charges {
        let charge = charges[charge as usize];
        let charge_pos = charge.position;
        let charge_coords = Vec2::new(
            charge_pos[0] - (constants.width as f32 / 2.0),
            charge_pos[1] - (constants.height as f32 / 2.0),
        );

        let q = charge.charge;
        let r = (current_coords - charge_coords).length().max(0.001);
        potential += q / r;
    }

    let final_potential = potential * k;

    // Technically unsafe cause unbound
    unsafe {
        electric_potential.write(coords, final_potential);
    }
}

#[spirv(compute(threads(256), entry_point_name = "electric_field_cs"))]
pub fn electric_field_cs(
    #[spirv(global_invocation_id)] global_invocation_id: UVec3,
    #[spirv(descriptor_set = 0, binding = 0, storage_buffer)] constants: &ShaderConstants,
    #[spirv(descriptor_set = 1, binding = 1)] electric_potential: &ElectricPotential,
    #[spirv(descriptor_set = 1, binding = 2)] electric_field: &ElectricField,
) {
    // Method of central differences to get gradient at any single point.
    // f'(x) = (f(x+h) - f(x-h)) / 2h
    // Then by applying coulombs law, we know that 𝐄⃗=∇⃗φ
    // E = -< ∂φ / ∂x, ∂φ / ∂y>
    let coords = global_invocation_id.truncate();
    // make it signed.
    let icoords = coords.as_ivec2();

    let max = IVec2::new(constants.width as i32 - 1, constants.height as i32 - 1);

    // This ivec and uvec and clamping headache just makes sure my coordinates dont explode.
    let up_sample = electric_potential
        .read((icoords + IVec2::new(0, H as i32).clamp(IVec2::ZERO, max)).as_uvec2());

    let down_sample = electric_potential
        .read((icoords + IVec2::new(0, -(H as i32)).clamp(IVec2::ZERO, max)).as_uvec2());

    let left_sample = electric_potential
        .read((icoords + IVec2::new(H as i32, 0).clamp(IVec2::ZERO, max)).as_uvec2());

    let right_sample = electric_potential
        .read((icoords + IVec2::new(-(H as i32), 0).clamp(IVec2::ZERO, max)).as_uvec2());

    let d_dx = (right_sample - left_sample) / (2.0 * H as f32);
    let d_dy = (up_sample - down_sample) / (2.0 * H as f32);

    let field = Vec2::new(-d_dx, -d_dy);

    unsafe {
        electric_field.write(coords, field.extend(0.0).extend(0.0));
    }
}
