use crate::*;

#[spirv(compute(threads(256), entry_point_name = "particle_cs"))]
pub fn particle_cs(
    // The absolute index of the current data piece
    #[spirv(global_invocation_id)] global_invocation_id: UVec3,
    #[spirv(descriptor_set = 0, binding = 0, uniform)] constants: &ShaderConstants,
    #[spirv(descriptor_set = 1, binding = 0, storage_buffer)] input: &[Particle],
    #[spirv(descriptor_set = 1, binding = 1, storage_buffer)] output: &mut [Particle],
    // The charge buffer already present in the electric bind group (bind group 2) at binding 0
    #[spirv(descriptor_set = 2, binding = 0, storage_buffer)] charges: &[Charge],
    #[spirv(descriptor_set = 2, binding = 3, storage_buffer)] electric_field: &mut [Field],
) {
    // Extract the index using the invocation id
    let particle_index = global_invocation_id.x as usize;
    // Do math only if its within the range of particles that actually exist
    if particle_index < constants.num_particles as usize {
        let mut particle = input[particle_index];
        let px = particle.position[0] as i32;
        let py = particle.position[1] as i32;
        let index = (px as u32 + py as u32 * constants.width) as usize;
        // If off-screen, freeze
        if px < 0 || py < 0 || px >= constants.width as i32 || py >= constants.height as i32 {
            output[particle_index] = particle;
            return;
        }

        let current_pos = Vec2::new(px as f32, py as f32);

        // If near charge, freeze
        // copied code from electric shader... Dont know how much this well affect perforamance..
        let mut near_charge = false;
        for i in 0..constants.num_charges {
            let charge_pos = charges[i as usize].position;
            let charge_vec = Vec2::new(charge_pos[0], charge_pos[1]);
            let distance = (charge_vec - current_pos).length();
            if distance <= constants.electric_options.stop_distance {
                near_charge = true;
                break;
            }
        }

        if near_charge {
            output[particle_index] = particle;
            return;
        }

        // Calculate the velocity of the particle at its specific point in space & time.
        let mut velocity = electric_field[index].field;
        let damping = 1.0 - constants.particle_options.drag_value;
        velocity[0] *= damping;
        velocity[1] *= damping;

        // Apply that velocity
        particle.position[0] += velocity[0] * constants.dt * constants.particle_options.time_scale;
        particle.position[1] += velocity[1] * constants.dt * constants.particle_options.time_scale;

        // Not to lose data, we create mut var, and we assign whole particle to the output.
        output[particle_index] = particle;
    }
}

// Guide on compute shaders
//
// Absolute index of data piece
// #[spirv(global_invocation_id)] global_invocation_id: UVec3,
//
// Within that 256 thread work group, which one im in
// #[spirv(local_invocation_id)] local_invocation_id: UVec3,
//
// More hardware related, id inside the hardware cluster
// #[spirv(subgroup_local_invocation_id)] subgroup_local_invocation_id: u32,
//
// 512 / 256 = 2 -> tell us which subdisivison
// #[spirv(workgroup_id)] workgroup_id: UVec3,
//
// Which part of workgroup
// #[spirv(subgroup_id)] subgroup_id: u32,
//
// How many parts exist
// #[spirv(num_subgroups)] num_subgroups: u32,
//
// Storage buffers that we can pass in
// #[spirv(descriptor_set = 0, binding = 0, storage_buffer)] constants: &ShaderConstants,
// #[spirv(descriptor_set = 0, binding = 1, storage_buffer)] input: &[ParticleUniform],
// #[spirv(descriptor_set = 0, binding = 2, storage_buffer)] output: &mut [ParticleUniform],
//
// Shared memory within a workgroup, only between those 256 items
// #[spirv(workgroup)] shared: &mut [u32; 256],
