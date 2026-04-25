// Utilitiy functions, which i didnt make myself :/

use glam::{Vec3, Vec4};
use spirv_std::arch::Derivative;
// DONT REMOVE
use spirv_std::num_traits::Float;

pub fn antialias_no_fwidth(dist: f32, thickness: f32) -> f32 {
    let edge: f32 = 1.0;
    1.0 - smoothstep(thickness - edge, thickness + edge, dist)
}

pub fn antialias(dist: f32, thickness: f32) -> f32 {
    let edge = dist.fwidth();
    1.0 - smoothstep(thickness - edge, thickness + edge, dist)
}

pub fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

pub fn map_range(val: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    (val - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

pub fn hsv(h: f32, s: f32, v: f32) -> Vec4 {
    let r = (((h + 1.0).fract() * 6.0 - 3.0).abs() - 1.0).clamp(0.0, 1.0);
    let g = (((h + 2.0 / 3.0).fract() * 6.0 - 3.0).abs() - 1.0).clamp(0.0, 1.0);
    let b = (((h + 1.0 / 3.0).fract() * 6.0 - 3.0).abs() - 1.0).clamp(0.0, 1.0);

    let rgb = Vec3::new(1.0, 1.0, 1.0).lerp(Vec3::new(r, g, b), s) * v;

    rgb.extend(1.0)
}
