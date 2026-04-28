use bytemuck::{Pod, Zeroable};
use glam::{Vec2, Vec3, Vec4};
use spirv_std::arch::Derivative;
#[allow(unused_imports)]
use spirv_std::num_traits::Float;

// AI Generated function for testing
pub fn arrow_fn(x: f32, y: f32, t: f32) -> Vec2 {
    let p = Vec2::new(x, y) * 0.005;
    let time = t * 0.4;
    let mut vx = (p.y + time).cos() + (p.y * 0.5 + time * 0.6).cos();
    let mut vy = (p.x - time).sin() + (p.x * 0.4 - time * 0.8).sin();
    let angle = (p.x * 1.2 + time).sin() * (p.y * 1.2 - time).cos();
    vx += angle.cos() * 0.5;
    vy += angle.sin() * 0.5;
    vx += (p.x * 3.0 + p.y * 2.0 + time * 2.0).sin() * 0.2;
    vy += (p.y * 3.0 - p.x * 2.0 - time * 2.0).cos() * 0.2;
    Vec2::new(vx, vy)
}

// These consstants are also defined inside of the rust code and passed in as a storage buffer.
#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct ShaderConstants {
    pub width: u32,
    pub height: u32,
    pub aspect_ratio: f32,
    pub time: f32,
    pub dt: f32,
    // To know if i am within range
    pub num_particles: u32,
    pub epsilon_naught: f32,
    pub num_charges: u32,
}

pub struct SDF;

impl SDF {
    pub fn sdf_line(a_pos: Vec2, b_pos: Vec2, p_pos: Vec2) -> f32 {
        let ab_vec = b_pos - a_pos;
        let ap_vec = p_pos - a_pos;
        // Originally:
        // let h = ab_vec.dot(ap_vec) / ab_vec.length().powi(2);
        // Much faster:
        let h = ab_vec.dot(ap_vec) / ab_vec.dot(ab_vec).max(0.001);
        let h_clamped = h.clamp(0.0, 1.0);
        (ap_vec - h_clamped * ab_vec).length()
    }

    pub fn sdf_rectangle(a_pos: Vec2, b_pos: Vec2, p_pos: Vec2) -> f32 {
        let ab_vec = b_pos - a_pos;
        let ap_vec = p_pos - a_pos;

        // This term remains the same as in line.
        // Bring out denominator since we will use it.
        let ab_len_sq = ab_vec.dot(ab_vec).max(0.001);
        let h = ab_vec.dot(ap_vec) / ab_len_sq;

        // instead of just getting straigt distance, we get perpendicular and longitudanal distance
        // seperatly and then get the maximum one.
        let perp_distance = (ap_vec - h * ab_vec).length();
        let ab_len = ab_len_sq.sqrt();
        let long_distnace = (-h).max(h - 1.0) * ab_len;

        perp_distance.max(long_distnace).max(0.0)
    }

    pub fn sdf_triangle(a_pos: Vec2, b_pos: Vec2, c_pos: Vec2, p_pos: Vec2) -> f32 {
        let ab_vec = b_pos - a_pos;
        let bc_vec = c_pos - b_pos;
        let ca_vec = a_pos - c_pos;

        let ap_vec = p_pos - a_pos;
        let bp_vec = p_pos - b_pos;
        let cp_vec = p_pos - c_pos;

        // A triangle is just a combination of 3 SDFs
        let sdf_ab = SDF::sdf_line(a_pos, b_pos, p_pos);
        let sdf_bc = SDF::sdf_line(b_pos, c_pos, p_pos);
        let sdf_ca = SDF::sdf_line(c_pos, a_pos, p_pos);

        // AB x AP
        // If cross is negative, meaning point is always on the left, hence inside the triangle,
        // if cross is positive, then outside triangle.
        let cross_ab = ab_vec.x * ap_vec.y - ab_vec.y * ap_vec.x;
        let cross_bc = bc_vec.x * bp_vec.y - bc_vec.y * bp_vec.x;
        let cross_ca = ca_vec.x * cp_vec.y - ca_vec.y * cp_vec.x;

        // Gives us the unsigned distance
        let distance = sdf_ab.min(sdf_bc).min(sdf_ca);

        // Actually, instead we have to check if all have SAME sign
        if (cross_ab < 0.0 && cross_bc < 0.0 && cross_ca < 0.0)
            || (cross_ab > 0.0 && cross_bc > 0.0 && cross_ca > 0.0)
        {
            -distance
        } else {
            distance
        }
    }
}

// Utilitiy functions, which i didnt make myself :/

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
