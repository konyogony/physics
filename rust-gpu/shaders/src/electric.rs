// Seperate shader for the particles
#![allow(clippy::too_many_arguments)]

use crate::shared::ShaderConstants;
use core::f32::consts::PI;
use glam::{UVec2, UVec3, Vec2};
#[allow(unused_imports)]
use spirv_std::num_traits::Float;
use spirv_std::{Image, spirv};

// TODO:
// CONVERT EVERYTHING FROM A DENSITY TEXTURE TO JUST DIRECT SUMMATION OVER EVERY CHARGE.

// Creating storage textures.
// NO SAMPLER NEEDED YAY!!!
pub type ElectricDensity = Image!(2D, format = r32f, sampled = false);
pub type ElectricPotential = Image!(2D, format = r32f, sampled = false);
pub type ElectricField = Image!(2D, format = rgba32f, sampled = false);

pub const DV: f32 = 1.0;
pub const H: u32 = 1;

// This will be ran for every pixel on the screen once.
#[spirv(compute(threads(256), entry_point_name = "electric_potential_cs"))]
pub fn electric_potential_cs(
    #[spirv(global_invocation_id)] global_invocation_id: UVec3,
    #[spirv(descriptor_set = 0, binding = 0, storage_buffer)] constants: &ShaderConstants,
    // Let this be a texture so we can use its UV coordinates. This theoretically should have a fixed size
    #[spirv(descriptor_set = 1, binding = 0)] density: &ElectricDensity,
    // Electric potential input
    #[spirv(descriptor_set = 1, binding = 1)] electric_potential: &ElectricPotential,
) {
    // Vec3 -> Vec2
    let coords = global_invocation_id.truncate();
    // UV Coords to screen space.
    let source_coords = Vec2::new(
        coords.x as f32 - (constants.width as f32 / 2.0),
        coords.y as f32 - (constants.height as f32 / 2.0),
    );
    let mut potential = 0.0;

    let k = 1.0 / (4.0 * PI * constants.epsilon_naught);

    for x in 0..constants.width {
        for y in 0..constants.height {
            let coords_centered = Vec2::new(
                x as f32 - constants.width as f32 / 2.0,
                y as f32 - constants.height as f32 / 2.0,
            );
            let density = density.read(UVec2::new(x, y));
            // Distance from observer to the source
            let r = (source_coords - coords_centered).length().max(0.001);
            potential += (density / r) * DV;
        }
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
    #[spirv(descriptor_set = 1, binding = 1)] electric_potential: &ElectricPotential,
    #[spirv(descriptor_set = 1, binding = 2)] electric_field: &ElectricField,
) {
    // Method of central differences to get gradient at any single point.
    // f'(x) = (f(x+h) - f(x-h)) / 2h
    // Then by applying coulombs law, we know that 𝐄⃗=∇⃗φ
    // E = -< ∂φ / ∂x, ∂φ / ∂y>
    let coords = global_invocation_id.truncate();

    // TODO: Make sure it doesnt explode
    let up_vec = UVec2::new(coords.x, coords.y + H);
    let down_vec = UVec2::new(coords.x, coords.y - H);
    let right_vec = UVec2::new(coords.x + H, coords.y);
    let left_vec = UVec2::new(coords.x - H, coords.y);

    let up_sample = electric_potential.read(up_vec);
    let down_sample = electric_potential.read(down_vec);
    let right_sample = electric_potential.read(right_vec);
    let left_sample = electric_potential.read(left_vec);

    let d_dx = (right_sample - left_sample) / (2.0 * H as f32);
    let d_dy = (up_sample - down_sample) / (2.0 * H as f32);

    let field = Vec2::new(-d_dx, -d_dy);

    unsafe {
        electric_field.write(coords, field.extend(0.0).extend(0.0));
    }
}
