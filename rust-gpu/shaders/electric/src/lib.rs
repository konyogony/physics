#![no_std]
#![allow(clippy::too_many_arguments)]
#![allow(unused_imports)]
use core::f32::consts::PI;
use glam::{UVec3, Vec2, Vec3, Vec4};
use shaders_shared::{Charge, EPSILON_SQ, Field, H, Plate, ShaderConstants, TracePoint};
use spirv_std::num_traits::Float;
use spirv_std::spirv;

pub mod compute;
pub mod fragment;
pub mod vertex;

pub const MIN_FIELD_STRENGTH: f32 = 1e-4;
pub const EPSILON: f32 = 1e-5;
pub const MIN_STEP: f32 = 0.5;
pub const DIST_SCALE: f32 = 0.05;

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Condition {
    Inside = 0,
    Outside = 1,
    Boundary = 2,
}

pub fn is_inside(point: Vec2, vertices: [Vec2; 4]) -> Condition {
    let mut sign: i32 = 0;
    let mut on_boundary = false;
    const EPSILON: f32 = 1e-5;

    for i in 0..4 {
        let a = vertices[i];
        let b = vertices[(i + 1) % 4];

        let edge = b - a;
        let to_p = point - a;
        // turns out theres a better method
        let cross = edge.perp_dot(to_p);

        // use epsilon, since we cant get true 0.0 value if on boundary (floating point inaccuracy)
        if cross.abs() < EPSILON {
            on_boundary = true;
        } else if cross > 0.0 {
            if sign == -1 {
                return Condition::Outside;
            }
            sign = 1;
        } else {
            if sign == 1 {
                return Condition::Outside;
            }
            sign = -1;
        }
    }

    if on_boundary {
        Condition::Boundary
    } else {
        Condition::Inside
    }
}

pub fn fill_from(tracing: &mut [TracePoint], pid: usize, from: u32, max: u32, pos: Vec2) {
    for s in from..max {
        tracing[(pid as u32 * max + s) as usize].pos = pos.into();
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
