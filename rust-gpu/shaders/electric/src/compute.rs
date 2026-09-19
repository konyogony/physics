use crate::*;

#[spirv(compute(threads(16, 16), entry_point_name = "electric_potential_cs"))]
pub fn electric_potential_cs(
    #[spirv(global_invocation_id)] global_invocation_id: UVec3,
    #[spirv(descriptor_set = 0, binding = 0, uniform)] constants: &ShaderConstants,
    #[spirv(descriptor_set = 1, binding = 0, storage_buffer)] charges: &[Charge],
    #[spirv(descriptor_set = 1, binding = 1, storage_buffer)] plates: &[Plate],
    #[spirv(descriptor_set = 1, binding = 4, storage_buffer)] input: &[f32],
    #[spirv(descriptor_set = 1, binding = 5, storage_buffer)] output: &mut [f32],
) {
    let width = constants.width as usize;
    let height = constants.height as usize;

    let x = global_invocation_id.x as usize;
    let y = global_invocation_id.y as usize;

    // so we dont run extra times
    if x >= constants.width as usize || y >= constants.height as usize {
        return;
    }
    // Make sure to clamp and make these numbers safe to work with;
    let left_x = if x > 0 { x - 1 } else { 0 };
    let right_x = (x + 1).min(width - 1);
    let down_y = if y > 0 { y - 1 } else { 0 };
    let up_y = (y + 1).min(height - 1);

    let center = x + y * width;
    let left = left_x + y * width;
    let right = right_x + y * width;
    let down = x + down_y * width;
    let up = x + up_y * width;

    if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
        output[center] = 0.0;
        return;
    }

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

        match is_inside(current_coords, edges) {
            Condition::Inside | Condition::Boundary => {
                output[center] = plate.potential * constants.electric_options.charge_strength_scale;
                return;
            }
            _ => (),
        }
    }

    // for any point outside the metal plate
    let left_phi = input[left];
    let right_phi = input[right];
    let up_phi = input[up];
    let down_phi = input[down];

    let sum = left_phi + right_phi + up_phi + down_phi;
    let average = sum as f32 / 4.0;

    // Now go through every charge
    let mut rho = 0.0;
    for charge_idx in 0..constants.num_charges {
        let charge = charges[charge_idx as usize];
        let charge_pos = charge.position;
        let charge_coords = Vec2::new(charge_pos[0], charge_pos[1]);
        let charge_charge = charge.charge * constants.electric_options.charge_strength_scale;

        let distance = (current_coords - charge_coords).length();
        // not really in PX's, so gotta play around with scaling this.
        let radius = constants.electric_options.charge_radius;

        if distance < radius {
            rho += charge_charge * (1.0 - distance / radius);
        }
    }
    let source = rho / (4.0 * constants.epsilon_naught);

    output[center] = average + source;
}

#[spirv(compute(threads(16, 16), entry_point_name = "electric_field_cs"))]
pub fn electric_field_cs(
    #[spirv(global_invocation_id)] global_invocation_id: UVec3,
    #[spirv(descriptor_set = 0, binding = 0, uniform)] constants: &ShaderConstants,
    #[spirv(descriptor_set = 1, binding = 2, storage_buffer)] electric_field: &mut [Field],
    #[spirv(descriptor_set = 1, binding = 4, storage_buffer)] electric_potential: &[f32],
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

    let d_dx = (right_sample - left_sample) / (2.0 * H as f32);
    let d_dy = (down_sample - up_sample) / (2.0 * H as f32);

    let field = Field {
        field: [-d_dx, -d_dy],
        _pad: [0.0; 2],
    };

    electric_field[index as usize] = field
}

#[spirv(compute(threads(128), entry_point_name = "electric_tracing_cs"))]
pub fn electric_tracing_cs(
    #[spirv(global_invocation_id)] global_invocation_id: UVec3,
    #[spirv(descriptor_set = 0, binding = 0, uniform)] constants: &ShaderConstants,
    #[spirv(descriptor_set = 1, binding = 0, storage_buffer)] charges: &[Charge],
    #[spirv(descriptor_set = 1, binding = 1, storage_buffer)] plates: &[Plate],
    #[spirv(descriptor_set = 1, binding = 2, storage_buffer)] electric_field: &mut [Field],
    #[spirv(descriptor_set = 1, binding = 3, storage_buffer)] tracing: &mut [TracePoint],
) {
    if constants.num_charges == 0 {
        return;
    }

    // we terminate the tracing when:
    // 1. we leave screen boumdary
    // 2. we touch metal plate
    // 3. we touch charge
    // 4. |E| is approx 0
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

    let mut current_pos = center + local_offset;

    for step in 0..constants.electric_options.max_steps {
        let tracing_index =
            (particle_id as u32 * constants.electric_options.max_steps + step) as usize;
        tracing[tracing_index].pos = current_pos.into();

        // if we are off-screen
        if current_pos.x <= 0.0
            || current_pos.x >= (constants.width - 1) as f32
            || current_pos.y <= 0.0
            || current_pos.y >= (constants.height - 1) as f32
        {
            fill_from(
                tracing,
                particle_id,
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

            if is_inside(current_pos, edges) != Condition::Outside {
                hit_plate = true;
                break;
            }
        }

        if hit_plate {
            fill_from(
                tracing,
                particle_id,
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
                particle_id,
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

        if strength1 < 1e-5 {
            fill_from(
                tracing,
                particle_id,
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
