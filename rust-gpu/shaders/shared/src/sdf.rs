use glam::Vec2;
#[allow(unused_imports)]
use spirv_std::num_traits::Float;

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
