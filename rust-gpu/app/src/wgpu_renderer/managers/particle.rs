use crate::wgpu_renderer::bind_group::{
    GlobalBindGroupLayout, ParticleBindGroups, ParticleBuffers,
};
use shaders_shared::{MAX_PARTICLES, Particle};
use wgpu::{BufferDescriptor, BufferUsages, CommandEncoderDescriptor, Device, MapMode, Queue};

pub struct ParticleManager {
    pub staging_buffer: wgpu::Buffer,
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

        // although buffer code doenst really belong here, we js create like an empty buffer to
        // stage and store some data
        let staging_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("particle_staging_buffer"),
            size,
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            staging_buffer,
            particle_buffers,
            particle_bind_groups,
            current_num_of_particles: 0,
        }
    }

    pub fn remove_particle(
        &mut self,
        device: &Device,
        queue: &Queue,
        position: [f32; 2],
        max_distance: Option<f32>,
    ) -> Option<Particle> {
        if self.current_num_of_particles == 0 {
            return None;
        }

        // get basically the offset position
        let active_byte_size =
            (self.current_num_of_particles as usize * std::mem::size_of::<Particle>()) as u64;

        // copy data from gpu to cpu
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor { label: None });
        encoder.copy_buffer_to_buffer(
            &self.particle_buffers.particles_buffer_a,
            0,
            &self.staging_buffer,
            0,
            active_byte_size,
        );
        queue.submit(std::iter::once(encoder.finish()));

        let buffer_slice = self.staging_buffer.slice(0..active_byte_size);
        let (sender, receiver) = std::sync::mpsc::channel();
        buffer_slice.map_async(MapMode::Read, move |v| sender.send(v).unwrap());

        // wait until we finish copying
        device
            .poll(wgpu::wgt::PollType::wait_indefinitely())
            .unwrap();

        if let Ok(Ok(())) = receiver.recv() {
            let data = buffer_slice.get_mapped_range();
            let mut current_particles: Vec<Particle> = bytemuck::cast_slice(&data).to_vec();

            drop(data);
            self.staging_buffer.unmap();

            let (closest_idx, min_dist_sq) = current_particles
                .iter()
                .enumerate()
                .map(|(idx, particle)| {
                    let dx = particle.position[0] - position[0];
                    let dy = particle.position[1] - position[1];
                    let dist_sq = dx * dx + dy * dy;
                    (idx, dist_sq)
                })
                .min_by(|a, b| a.1.total_cmp(&b.1))
                .unwrap();

            if let Some(max_dist) = max_distance
                && min_dist_sq > max_dist * max_dist
            {
                return None;
            }

            let removed = current_particles.swap_remove(closest_idx);
            self.current_num_of_particles -= 1;

            if !current_particles.is_empty() {
                let updated_bytes = bytemuck::cast_slice(&current_particles);
                queue.write_buffer(&self.particle_buffers.particles_buffer_a, 0, updated_bytes);
                queue.write_buffer(&self.particle_buffers.particles_buffer_b, 0, updated_bytes);
            }

            return Some(removed);
        }

        None
    }

    pub fn add_particle(&mut self, queue: &Queue, position: [f32; 2]) {
        if self.current_num_of_particles >= MAX_PARTICLES {
            return;
        }

        let particle = Particle {
            position,
            velocity: [0.0; 2],
            color: [0.2, 0.4, 1.0],
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
        self.current_num_of_particles = 0;
    }
}
