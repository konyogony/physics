use crate::*;

// Say each particle will consist of 6 vertices. (Small square / quad)
// For each group of 6 vertices we have to displace them first by the particle position
// and then displace each individual vertex based on its relational position
#[spirv(vertex(entry_point_name = "particle_vs"))]
pub fn particle_vs(
    // Tells us which vertex inside that group we are doing
    #[spirv(vertex_index)] vtx_id: i32,
    // Tells us which group of vertices we are doing
    #[spirv(instance_index)] instance_id: i32,
    #[spirv(position)] vtx_pos: &mut Vec4,
    #[spirv(descriptor_set = 0, binding = 0, uniform)] constants: &ShaderConstants,
    #[spirv(descriptor_set = 1, binding = 0, storage_buffer)] particles: &[Particle],
    #[spirv(location = 0)] vtx_color: &mut Vec3,
) {
    // Extract which particle we are doing based on group index
    let particle = particles[instance_id as usize];
    let center: Vec2 = particle.position.into();

    let num_segments = constants.particle_options.polygon_vertices / 3;
    let triangle_id = vtx_id / 3;
    let corner_id = vtx_id % 3;

    let local_offset = if corner_id == 0 {
        Vec2::ZERO
    } else {
        let angle_increment = (2.0 * PI) / num_segments as f32;
        let angle_offset = (triangle_id as f32 + (corner_id - 1) as f32) * angle_increment;
        Vec2::new(
            constants.particle_options.particle_radius * angle_offset.cos(),
            constants.particle_options.particle_radius * angle_offset.sin(),
        )
    };

    // Extract the offset based on the individual vertex index
    let pos_px = center + local_offset;

    // Conver to propper coordinates (NDC)
    let pos_uv = Vec2::new(
        (pos_px.x / constants.width as f32) * 2.0 - 1.0,
        (pos_px.y / constants.height as f32) * -2.0 + 1.0,
    );

    // Apply the position
    *vtx_pos = pos_uv.extend(0.0).extend(1.0);
    *vtx_color = particle.color.into();
}
