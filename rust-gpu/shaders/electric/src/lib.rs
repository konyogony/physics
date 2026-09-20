#![no_std]
#![allow(clippy::too_many_arguments)]
#![allow(unused_imports)]
use core::f32::consts::PI;
use glam::{UVec3, Vec2, Vec3, Vec4};
use shaders_shared::{
    Charge, Condition, EPSILON, Field, H, Plate, ShaderConstants, TracePoint, is_inside,
};
use spirv_std::num_traits::Float;
use spirv_std::spirv;

pub mod compute;
pub mod fragment;
pub mod vertex;

pub fn fill_from(tracing: &mut [TracePoint], id: usize, from: u32, max: u32, pos: Vec2) {
    for s in from..max {
        tracing[(id as u32 * max + s) as usize].pos = pos.into();
    }
}

// trust me i did not do this function, I hate bilinear interpolation
pub fn sample_field_bilinear(
    pos: Vec2,
    field: &[Field],
    width: usize,
    height: usize,
) -> (Vec2, f32) {
    if pos.x < 0.0 || pos.x >= (width - 1) as f32 || pos.y < 0.0 || pos.y >= (height - 1) as f32 {
        return (Vec2::ZERO, 0.0);
    }

    let x0 = pos.x.floor() as usize;
    let y0 = pos.y.floor() as usize;
    let x1 = x0 + 1;
    let y1 = y0 + 1;

    let fx = pos.x - x0 as f32;
    let fy = pos.y - y0 as f32;

    let f00 = Vec2::from(field[x0 + y0 * width].field);
    let f10 = Vec2::from(field[x1 + y0 * width].field);
    let f01 = Vec2::from(field[x0 + y1 * width].field);
    let f11 = Vec2::from(field[x1 + y1 * width].field);

    let top = f00.lerp(f10, fx);
    let bottom = f01.lerp(f11, fx);
    let e = top.lerp(bottom, fy);

    let strength = e.length();
    let dir = if strength > 1e-6 {
        e / strength
    } else {
        Vec2::ZERO
    };

    (dir, strength)
}

pub fn trace(
    starting_pos: Vec2,
    tracing: &mut [TracePoint],
    electric_field: &mut [Field],
    plates: &[Plate],
    charges: &[Charge],
    constants: &ShaderConstants,
    id: usize,
) {
    // we terminate the tracing when:
    // 1. we leave screen boumdary
    // 2. we touch metal plate
    // 3. we touch charge
    // 4. |E| is approx 0
    let mut current_pos = starting_pos;

    for step in 0..constants.electric_options.max_steps {
        let tracing_index = (id as u32 * constants.electric_options.max_steps + step) as usize;
        tracing[tracing_index].pos = current_pos.into();

        // if we are off-screen
        if current_pos.x <= 0.0
            || current_pos.x >= (constants.width - 1) as f32
            || current_pos.y <= 0.0
            || current_pos.y >= (constants.height - 1) as f32
        {
            fill_from(
                tracing,
                id,
                step,
                constants.electric_options.max_steps,
                current_pos,
            );
            return;
        }

        // if we hit plate
        let mut hit_plate = false;
        for plate_idx in 0..constants.num_plates {
            let plate = plates[plate_idx as usize];
            let edges = [
                Vec2::new(plate.edges[0], plate.edges[1]),
                Vec2::new(plate.edges[2], plate.edges[3]),
                Vec2::new(plate.edges[4], plate.edges[5]),
                Vec2::new(plate.edges[6], plate.edges[7]),
            ];

            if is_inside(current_pos, edges, 1.0) != Condition::Outside {
                hit_plate = true;
                break;
            }
        }

        if hit_plate {
            fill_from(
                tracing,
                id,
                step,
                constants.electric_options.max_steps,
                current_pos,
            );
            return;
        }

        // if we hit charge
        let mut hit_charge = false;
        for charge_idx in 0..constants.num_charges {
            let charge = charges[charge_idx as usize];

            // only -ve
            if charge.charge < 0.0 {
                let charge_pos = charge.position;
                let charge_coords = Vec2::new(charge_pos[0], charge_pos[1]);

                if (current_pos - charge_coords).length()
                    <= constants.electric_options.stop_distance
                {
                    hit_charge = true;
                    break;
                }
            }
        }

        if hit_charge {
            fill_from(
                tracing,
                id,
                step,
                constants.electric_options.max_steps,
                current_pos,
            );
            return;
        }

        // if |E| approx 0

        // gotta be small 1.0-3.0
        let step_size = constants.electric_options.step_size;
        let (k1, strength1) = sample_field_bilinear(
            current_pos,
            electric_field,
            constants.width as usize,
            constants.height as usize,
        );

        if strength1 < EPSILON {
            fill_from(
                tracing,
                id,
                step,
                constants.electric_options.max_steps,
                current_pos,
            );
            return;
        }

        let mid_pos = current_pos + k1 * step_size * 0.5;
        let (k2, _strength2) = sample_field_bilinear(
            mid_pos,
            electric_field,
            constants.width as usize,
            constants.height as usize,
        );

        current_pos += k2 * step_size;
    }
}
