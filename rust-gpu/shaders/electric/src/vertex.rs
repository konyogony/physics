use crate::*;

#[spirv(vertex(entry_point_name = "electric_vs"))]
pub fn electric_vs(
    #[spirv(vertex_index)] vtx_id: i32,
    #[spirv(instance_index)] instance_id: i32,
    #[spirv(position)] vtx_pos: &mut Vec4,
    #[spirv(descriptor_set = 0, binding = 0, uniform)] constants: &ShaderConstants,
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
    if charge.charge < 0.0 {
        *vtx_color = Vec3::new(0.0, 1.0, 1.0);
    } else {
        *vtx_color = Vec3::new(1.0, 0.5, 0.0);
    }
}

#[spirv(vertex(entry_point_name = "electric_plates_vs"))]
pub fn electric_plates_vs(
    #[spirv(vertex_index)] vtx_id: i32,
    #[spirv(instance_index)] instance_id: i32,
    #[spirv(position)] vtx_pos: &mut Vec4,
    #[spirv(descriptor_set = 0, binding = 0, uniform)] constants: &ShaderConstants,
    #[spirv(descriptor_set = 1, binding = 1, storage_buffer)] plates: &[Plate],
    #[spirv(location = 0)] vtx_color: &mut Vec3,
) {
    let plate = plates[instance_id as usize];
    let edges = [
        Vec2::new(plate.edges[0], plate.edges[1]), // A (bottom-left)
        Vec2::new(plate.edges[2], plate.edges[3]), // B (bottom-right)
        Vec2::new(plate.edges[4], plate.edges[5]), // C (top-right)
        Vec2::new(plate.edges[6], plate.edges[7]), // D (top-left)
    ];

    // Only 2 triangles really needed for a whole palte
    // A->B->D and then B->C->D
    let pos_px = match vtx_id {
        0 => edges[0], // A
        1 => edges[1], // B
        2 => edges[3], // D

        3 => edges[1], // B
        4 => edges[2], // C
        _ => edges[3], // D
    };

    let pos_uv = Vec2::new(
        (pos_px.x / constants.width as f32) * 2.0 - 1.0,
        (pos_px.y / constants.height as f32) * -2.0 + 1.0,
    );

    *vtx_pos = pos_uv.extend(0.0).extend(1.0);
    *vtx_color = Vec3::new(0.722, 0.451, 0.200);
}

#[spirv(vertex(entry_point_name = "electric_tracing_vs"))]
pub fn electric_tracing_vs(
    #[spirv(vertex_index)] vtx_id: i32,
    #[spirv(instance_index)] instance_id: i32,
    #[spirv(position)] vtx_pos: &mut Vec4,
    #[spirv(descriptor_set = 0, binding = 0, uniform)] constants: &ShaderConstants,
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
