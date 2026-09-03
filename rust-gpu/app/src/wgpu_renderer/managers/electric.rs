#![allow(clippy::too_many_arguments)]
use crate::wgpu_renderer::bind_group::{
    ElectricBindGroups, ElectricStorageBuffers, GlobalBindGroupLayout,
};
use shaders_shared::Charge;
use shaders_shared::MAX_CHARGES;
use wgpu::Device;
use wgpu::Queue;
use winit::dpi::PhysicalSize;

pub struct ElectricManager {
    pub charges: Vec<Charge>,
    pub electric_storage_buffers: ElectricStorageBuffers,
    pub electric_bind_groups: ElectricBindGroups,
    pub size: PhysicalSize<u32>,
    pub buffer_size: u64,
    pub next_charge: f32,
    pub num_particles_per_charge: u32,
    pub max_steps: usize,
}

impl ElectricManager {
    pub fn new(
        device: &Device,
        queue: &Queue,
        global_bind_group_layout: &GlobalBindGroupLayout,
        size: PhysicalSize<u32>,
        charges: Vec<Charge>,
        max_steps: usize,
        num_particles_per_charge: u32,
    ) -> Self {
        let buffer_size = (std::mem::size_of::<Charge>() * MAX_CHARGES as usize) as u64;
        let electric_storage_buffers = global_bind_group_layout.create_electric_buffers(
            device,
            size,
            queue,
            buffer_size,
            charges.clone(),
            max_steps,
            num_particles_per_charge,
        );
        let electric_bind_groups =
            global_bind_group_layout.create_electric_bind_groups(device, &electric_storage_buffers);

        Self {
            electric_bind_groups,
            electric_storage_buffers,
            size,
            charges,
            buffer_size,
            next_charge: 1.0,
            max_steps,
            num_particles_per_charge,
        }
    }

    // resize can also act as a recreate function
    pub fn resize(
        &mut self,
        device: &Device,
        queue: &Queue,
        new_size: PhysicalSize<u32>,
        global_bind_group_layout: &GlobalBindGroupLayout,
        max_steps: usize,
        num_particles_per_charge: u32,
    ) {
        if new_size.width == 0 || new_size.height == 0 {
            return;
        }

        let old_size = self.size;
        let width_transform: f32 = new_size.width as f32 / old_size.width as f32;
        let height_transform: f32 = new_size.height as f32 / old_size.height as f32;

        // First modify AND store the new positions
        for charge in self.charges.iter_mut() {
            charge.position[0] *= width_transform;
            charge.position[1] *= height_transform;
        }

        self.electric_storage_buffers = global_bind_group_layout.create_electric_buffers(
            device,
            new_size,
            queue,
            self.buffer_size,
            self.charges.clone(),
            max_steps,
            num_particles_per_charge,
        );
        self.electric_bind_groups = global_bind_group_layout
            .create_electric_bind_groups(device, &self.electric_storage_buffers);
        self.size = new_size;
        self.max_steps = max_steps;
        self.num_particles_per_charge = num_particles_per_charge;
    }

    pub fn remove_charge(
        &mut self,
        queue: &Queue,
        position: [f32; 2],
        max_distance: Option<f32>,
    ) -> Option<Charge> {
        if self.charges.is_empty() {
            return None;
        }

        // go through all charges, enumerate to get index and charge.
        // then calculate distance between the charge and position of cursor
        // then get closest one
        let (closest_idx, min_dist_sq) = self
            .charges
            .iter()
            .enumerate()
            .map(|(idx, charge)| {
                let dx = charge.position[0] - position[0];
                let dy = charge.position[1] - position[1];
                let dist_sq = dx * dx + dy * dy;
                (idx, dist_sq)
            })
            .min_by(|a, b| a.1.total_cmp(&b.1))?;

        // make sure its actually close tho
        if let Some(max_dist) = max_distance
            && min_dist_sq > max_dist * max_dist
        {
            return None;
        }

        let removed = self.charges.swap_remove(closest_idx);
        if !self.charges.is_empty() {
            queue.write_buffer(
                &self.electric_storage_buffers.charges,
                0,
                bytemuck::cast_slice(&self.charges),
            );
        }

        Some(removed)
    }

    pub fn add_charge(&mut self, queue: &Queue, position: [f32; 2]) {
        if self.charges.len() >= MAX_CHARGES as usize {
            return;
        }

        let charge = Charge {
            position,
            charge: self.next_charge,
            _pad: 0.0,
        };

        let offset = (self.charges.len() * std::mem::size_of::<Charge>()) as u64;
        let data = bytemuck::bytes_of(&charge);

        // We can INSERT specific pieces of data into the buffer.
        queue.write_buffer(&self.electric_storage_buffers.charges, offset, data);
        self.charges.push(charge);
    }

    pub fn set_next_charge(&mut self, next_value: f32) {
        self.next_charge = next_value
    }

    pub fn toggle_charge(&mut self) {
        self.next_charge = -self.next_charge;
    }

    pub fn remove_all_charges(&mut self) {
        self.charges = Vec::new()
    }
}
