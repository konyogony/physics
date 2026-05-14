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
    pub charge_strength: f32,
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
        charge_strength: f32,
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
            next_charge: charge_strength,
            max_steps,
            num_particles_per_charge,
            charge_strength,
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
        charge_strength: f32,
    ) {
        if new_size.width == 0 || new_size.height == 0 {
            return;
        }

        let old_size = self.size;
        let width_transform: f32 = new_size.width as f32 / old_size.width as f32;
        let height_transform: f32 = new_size.height as f32 / old_size.height as f32;

        for charge in self.charges.iter_mut() {
            charge.position[0] *= width_transform;
            charge.position[1] *= height_transform;

            // Basically replace all charges with a new value + check if negative
            if charge.charge < 0.0 {
                charge.charge = -charge_strength
            } else {
                charge.charge = charge_strength
            }
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

    pub fn toggle_charge(&mut self) {
        let current_charge = self.next_charge;
        if current_charge > 0.0 {
            self.next_charge = -self.charge_strength;
        } else {
            self.next_charge = self.charge_strength
        }
    }

    pub fn remove_all_charges(&mut self) {
        self.charges = Vec::new()
    }
}
