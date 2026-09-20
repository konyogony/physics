use shaders_shared::{Condition, PX_PER_UNIT, SOFTENING, Segment, is_inside};

use crate::*;

#[spirv(compute(threads(16, 16), entry_point_name = "electric_potential_cs"))]
pub fn electric_potential_cs(
    #[spirv(global_invocation_id)] global_invocation_id: UVec3,
    #[spirv(descriptor_set = 0, binding = 0, uniform)] constants: &ShaderConstants,
    #[spirv(descriptor_set = 1, binding = 0, storage_buffer)] charges: &[Charge],
    #[spirv(descriptor_set = 1, binding = 1, storage_buffer)] plates: &[Plate],
    #[spirv(descriptor_set = 1, binding = 2, storage_buffer)] segments: &[Segment],
    #[spirv(descriptor_set = 1, binding = 5, storage_buffer)] potential: &mut [f32],
) {
    let width = constants.width as usize;
    let height = constants.height as usize;

    let x = global_invocation_id.x as usize;
    let y = global_invocation_id.y as usize;

    // so we dont run extra times
    if x >= width || y >= height {
        return;
    }

    let center = x + y * width;
    let current_coords = Vec2::new(x as f32, y as f32);

    // if we are inside any metal plate, get its charge, write and return straight away
    for plate_idx in 0..constants.num_plates {
        let plate = plates[plate_idx as usize];
        let edges = [
            Vec2::new(plate.edges[0], plate.edges[1]),
            Vec2::new(plate.edges[2], plate.edges[3]),
            Vec2::new(plate.edges[4], plate.edges[5]),
            Vec2::new(plate.edges[6], plate.edges[7]),
        ];

        match is_inside(current_coords, edges, 1.0) {
            Condition::Inside | Condition::Boundary => {
                potential[center] = plate.potential;
                return;
            }
            _ => (),
        }
    }

    let current = current_coords / PX_PER_UNIT;
    let mut phi: f32 = 0.0;

    for segment_idx in 0..constants.num_segments {
        let segment = segments[segment_idx as usize];
        let segment_coords = Vec2::from_array(segment.midpoint) / PX_PER_UNIT;

        let distance = (current - segment_coords).length_squared();
        phi += segment.solved_charge / (distance + SOFTENING * SOFTENING).sqrt();
    }

    for charge_idx in 0..constants.num_charges {
        let charge = charges[charge_idx as usize];
        let charge_coords = Vec2::from_array(charge.position) / PX_PER_UNIT;

        let distance = (current - charge_coords).length_squared();
        phi += charge.charge / (distance + SOFTENING * SOFTENING).sqrt();
    }

    potential[center] = phi;
}

#[spirv(compute(threads(16, 16), entry_point_name = "electric_field_cs"))]
pub fn electric_field_cs(
    #[spirv(global_invocation_id)] global_invocation_id: UVec3,
    #[spirv(descriptor_set = 0, binding = 0, uniform)] constants: &ShaderConstants,
    #[spirv(descriptor_set = 1, binding = 3, storage_buffer)] electric_field: &mut [Field],
    #[spirv(descriptor_set = 1, binding = 5, storage_buffer)] electric_potential: &mut [f32],
) {
    let x = global_invocation_id.x as i32;
    let y = global_invocation_id.y as i32;
    let index = x + y * constants.width as i32;

    if x >= constants.width as i32 || y >= constants.height as i32 {
        return;
    }

    let left_x = (x - H).max(0);
    let right_x = (x + H).min(constants.width as i32 - 1);
    let up_y = (y - H).max(0);
    let down_y = (y + H).min(constants.height as i32 - 1);

    let left_sample = electric_potential[(left_x + y * constants.width as i32) as usize];
    let right_sample = electric_potential[(right_x + y * constants.width as i32) as usize];
    let up_sample = electric_potential[(x + up_y * constants.width as i32) as usize];
    let down_sample = electric_potential[(x + down_y * constants.width as i32) as usize];

    let dx_px = (right_x - left_x).max(1) as f32;
    let dy_px = (down_y - up_y).max(1) as f32;
    let d_dx = (right_sample - left_sample) / dx_px * PX_PER_UNIT;
    let d_dy = (down_sample - up_sample) / dy_px * PX_PER_UNIT;

    let field = Field {
        field: [-d_dx, -d_dy],
        _pad: [0.0; 2],
    };

    electric_field[index as usize] = field
}

#[spirv(compute(threads(128), entry_point_name = "electric_tracing_charges_cs"))]
pub fn electric_tracing_charges_cs(
    #[spirv(global_invocation_id)] global_invocation_id: UVec3,
    #[spirv(descriptor_set = 0, binding = 0, uniform)] constants: &ShaderConstants,
    #[spirv(descriptor_set = 1, binding = 0, storage_buffer)] charges: &[Charge],
    #[spirv(descriptor_set = 1, binding = 1, storage_buffer)] plates: &[Plate],
    #[spirv(descriptor_set = 1, binding = 3, storage_buffer)] electric_field: &mut [Field],
    #[spirv(descriptor_set = 1, binding = 4, storage_buffer)] tracing: &mut [TracePoint],
) {
    if constants.num_charges == 0 {
        return;
    }

    let particle_id = global_invocation_id.x as usize;
    let charge_id = particle_id / constants.electric_options.num_particles_per_charge as usize;

    if particle_id
        >= (constants.num_charges * constants.electric_options.num_particles_per_charge) as usize
    {
        return;
    }

    let charge = charges[charge_id];
    let center: Vec2 = charge.position.into();

    if charge.charge < 0.0 {
        for step in 0..constants.electric_options.max_steps {
            let tracing_index =
                (particle_id as u32 * constants.electric_options.max_steps + step) as usize;
            tracing[tracing_index].pos = center.into();
        }
        return;
    }

    let local_offset = {
        let angle_increment =
            (2.0 * PI) / constants.electric_options.num_particles_per_charge as f32;
        let angle_offset = (particle_id as f32) * angle_increment;
        let radius = constants.electric_options.charge_radius;
        Vec2::new(radius * angle_offset.cos(), radius * angle_offset.sin())
    };

    trace(
        center + local_offset,
        tracing,
        electric_field,
        plates,
        charges,
        constants,
        particle_id,
    );
}

#[spirv(compute(threads(128), entry_point_name = "electric_tracing_plates_cs"))]
pub fn electric_tracing_plates_cs(
    #[spirv(global_invocation_id)] global_invocation_id: UVec3,
    #[spirv(descriptor_set = 0, binding = 0, uniform)] constants: &ShaderConstants,
    #[spirv(descriptor_set = 1, binding = 0, storage_buffer)] charges: &[Charge],
    #[spirv(descriptor_set = 1, binding = 1, storage_buffer)] plates: &[Plate],
    #[spirv(descriptor_set = 1, binding = 2, storage_buffer)] segments: &[Segment],
    #[spirv(descriptor_set = 1, binding = 3, storage_buffer)] electric_field: &mut [Field],
    #[spirv(descriptor_set = 1, binding = 4, storage_buffer)] tracing: &mut [TracePoint],
) {
    if constants.num_segments == 0 {
        return;
    }

    let segment_idx = global_invocation_id.x as usize;
    if segment_idx >= constants.num_segments as usize {
        return;
    }

    let segment = segments[segment_idx];
    let midpoint = Vec2::from_array(segment.midpoint);

    let charge_particles =
        (constants.num_charges * constants.electric_options.num_particles_per_charge) as usize;
    let particle_id = charge_particles + segment_idx;

    if segment.solved_charge <= 0.0 {
        fill_from(
            tracing,
            particle_id,
            0,
            constants.electric_options.max_steps,
            midpoint,
        );
        return;
    }

    // basically we cant start right from edge since then potential is 0.0, so we gotta offset
    // slightlyy
    let normal = Vec2::from_array(segment.normal);
    let start_pos = midpoint + normal * constants.particle_options.particle_radius / 2.0;

    trace(
        start_pos,
        tracing,
        electric_field,
        plates,
        charges,
        constants,
        particle_id,
    );
}
