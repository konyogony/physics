#![no_std]
// Seperate shader for the particles
#![allow(clippy::too_many_arguments)]

use core::f32::consts::PI;
use glam::{UVec3, Vec2, Vec3, Vec4};
use shaders_shared::{Charge, EPSILON_SQ, Field, H, ShaderConstants, TracePoint};
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

    let num_segments = constants.particle_options.polygon_vertices / 3;
    let triangle_id = vtx_id / 3;
    let corner_id = vtx_id % 3;

    let local_offset = if corner_id == 0 {
        Vec2::ZERO
    } else {
        let angle_increment = (2.0 * PI) / num_segments as f32;
        let angle_offset = (triangle_id as f32 + (corner_id - 1) as f32) * angle_increment;
        let radius = constants.electric_options.charge_radius;
        Vec2::new(radius * angle_offset.cos(), radius * angle_offset.sin())
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
        // Forgot to square this aswell, for the softening factor to work
        let r_sq = (current_coords - charge_coords).length_squared();
        // Usually potential is q / r, however for simulation purposes so that test charges dont
        // explode, we will use q / sqrt(r^2 + epsilon^2)
        potential += q / (r_sq + EPSILON_SQ).sqrt();
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

    // First calculate the coordinates and THEN the indices.
    let left_x = (x - H).max(0);
    let right_x = (x + H).min(constants.width as i32 - 1);

    // Since we are centered around top left, the `-` will bring us up and `+` will bring us down.
    let up_y = (y - H).max(0);
    let down_y = (y + H).min(constants.height as i32 - 1);

    let left_sample = electric_potential[(left_x + y * constants.width as i32) as usize];
    let right_sample = electric_potential[(right_x + y * constants.width as i32) as usize];

    let up_sample = electric_potential[(x + up_y * constants.width as i32) as usize];
    let down_sample = electric_potential[(x + down_y * constants.width as i32) as usize];

    // make it signed.
    let d_dx = (right_sample - left_sample) / (2.0 * H as f32);
    let d_dy = (down_sample - up_sample) / (2.0 * H as f32);

    let field = Field {
        field: [-d_dx, -d_dy],
        _pad: [0.0; 2],
    };

    electric_field[index as usize] = field
}

// Compute shader for drawing traces, for field lines. We spawn particles around positive charges
#[spirv(compute(threads(128), entry_point_name = "electric_tracing_cs"))]
pub fn electric_tracing_cs(
    #[spirv(global_invocation_id)] global_invocation_id: UVec3,
    #[spirv(descriptor_set = 0, binding = 0, storage_buffer)] constants: &ShaderConstants,
    #[spirv(descriptor_set = 1, binding = 0, storage_buffer)] charges: &[Charge],
    #[spirv(descriptor_set = 1, binding = 2, storage_buffer)] electric_field: &mut [Field],
    #[spirv(descriptor_set = 1, binding = 3, storage_buffer)] tracing: &mut [TracePoint],
) {
    let particle_id = global_invocation_id.x as usize;
    let charge_id = particle_id / constants.electric_options.num_particles_per_charge as usize;

    // Extract charge
    let charge = charges[charge_id];
    let center: Vec2 = charge.position.into();

    // Only do positive charges
    if charge.charge < 0.0 {
        // Set remaining data to last position so we get lines stopping at correct distance.
        for step in 0..constants.electric_options.max_steps {
            let tracing_index =
                (particle_id as u32 * constants.electric_options.max_steps + step) as usize;
            tracing[tracing_index].pos = center.into();
        }
        return;
    }

    // Calculate the angle offset for each particle to trace around the charge
    let local_offset = {
        let angle_increment =
            (2.0 * PI) / constants.electric_options.num_particles_per_charge as f32;
        let angle_offset = (particle_id as f32) * angle_increment;
        let radius = constants.electric_options.charge_radius;
        Vec2::new(radius * angle_offset.cos(), radius * angle_offset.sin())
    };

    let mut current_pos = center + local_offset;

    // Loop through N iterations
    for step in 0..constants.electric_options.max_steps {
        let tracing_index =
            (particle_id as u32 * constants.electric_options.max_steps + step) as usize;
        tracing[tracing_index].pos = current_pos.into();

        let x = current_pos.x.floor() as i32;
        let y = current_pos.y.floor() as i32;

        let out_of_bounds =
            x <= 0 || x >= constants.width as i32 || y <= 0 || y >= constants.height as i32;

        let mut near_charge = false;
        // Loop throuhg all charges and check if we are close to any of them
        for i in 0..constants.num_charges {
            let charge_pos = charges[i as usize].position;
            let charge_vec = Vec2::new(charge_pos[0], charge_pos[1]);
            let distance = (charge_vec - current_pos).length();
            if distance <= constants.electric_options.stop_distance {
                near_charge = true;
                break;
            }
        }

        // Basically if conditions are met, we just set remaining positions to same position
        if near_charge || out_of_bounds {
            for remaining in (step + 1)..constants.electric_options.max_steps {
                let index = (particle_id as u32 * constants.electric_options.max_steps + remaining)
                    as usize;

                tracing[index].pos = current_pos.into()
            }
            break;
        }

        let pos_index = x + y * constants.width as i32;
        let field_reading = electric_field[pos_index as usize];

        // Extract velocity
        let velocity = field_reading.field;
        let vel = Vec2::new(velocity[0], velocity[1]);
        let strength = vel.length();

        // Terminate since basically stuck.
        if strength < 1e-6 {
            for remaining in (step + 1)..constants.electric_options.max_steps {
                let index = (particle_id as u32 * constants.electric_options.max_steps + remaining)
                    as usize;

                tracing[index].pos = current_pos.into()
            }
            break;
        }

        // Normalized.
        let dir = vel / strength;
        // Update position of particle
        current_pos += dir * constants.electric_options.step_size;
    }
}

// Draw LINES (yes very cool) from P1 to P2, and so on.
#[spirv(vertex(entry_point_name = "electric_tracing_vs"))]
pub fn electric_tracing_vs(
    #[spirv(vertex_index)] vtx_id: i32,
    #[spirv(instance_index)] instance_id: i32,
    #[spirv(position)] vtx_pos: &mut Vec4,
    #[spirv(descriptor_set = 0, binding = 0, storage_buffer)] constants: &ShaderConstants,
    #[spirv(descriptor_set = 1, binding = 3, storage_buffer)] tracing: &mut [TracePoint],
) {
    if constants.draw_options.draw_field_lines == 0 {
        return;
    }

    let segment_id = vtx_id / 2;
    let endpoint = vtx_id % 2;
    let step = segment_id + endpoint;

    let index = instance_id * constants.electric_options.max_steps as i32 + step;
    let point = tracing[index as usize];

    let uv = Vec2::new(
        (point.pos[0] / constants.width as f32) * 2.0 - 1.0,
        (point.pos[1] / constants.height as f32) * -2.0 + 1.0,
    );

    *vtx_pos = uv.extend(0.0).extend(1.0);
}

#[spirv(fragment(entry_point_name = "electric_tracing_fs"))]
pub fn electric_tracing_fs(output: &mut Vec4) {
    *output = Vec4::new(1.0, 1.0, 1.0, 1.0);
}
