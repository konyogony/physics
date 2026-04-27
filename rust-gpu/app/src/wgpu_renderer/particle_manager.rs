use crate::wgpu_renderer::bind_group::{
    GlobalBindGroupLayout, ParticleBindGroups, ParticleBuffers,
};
use shaders::{MAX_PARTICLES, particle::Particle};
use wgpu::{Device, Queue};

pub struct ParticleManager {
    pub particle_buffers: ParticleBuffers,
    pub particle_bind_groups: ParticleBindGroups,
    pub current_num_of_particles: u32,
}

impl ParticleManager {
    pub fn new(device: &Device, global_bind_group_layout: &GlobalBindGroupLayout) -> Self {
        let size = (MAX_PARTICLES as usize * std::mem::size_of::<Particle>()) as u64;

        let particle_buffers = global_bind_group_layout.create_particle_buffers(device, size);
        let particle_bind_groups =
            global_bind_group_layout.create_particle_bind_groups(device, &particle_buffers);

        Self {
            particle_buffers,
            particle_bind_groups,
            current_num_of_particles: 0,
        }
    }

    pub fn add_particle(&mut self, queue: &Queue, position: [f32; 2]) {
        if self.current_num_of_particles >= MAX_PARTICLES {
            return;
        }

        let particle = Particle {
            position,
            velocity: [0.0; 2],
            color: [1.0, 1.0, 1.0],
            _pad: 0.0,
        };

        let offset =
            (self.current_num_of_particles as usize * std::mem::size_of::<Particle>()) as u64;
        let data = bytemuck::bytes_of(&particle);

        // We can INSERT specific pieces of data into the buffer.
        queue.write_buffer(&self.particle_buffers.particles_buffer_a, offset, data);
        queue.write_buffer(&self.particle_buffers.particles_buffer_b, offset, data);

        self.current_num_of_particles += 1;
    }

    pub fn remove_all_particles(&mut self) {
        // Kinda works like a stack, we dont delete the data, we just update the pointer
        self.current_num_of_particles = 0;
    }
}
