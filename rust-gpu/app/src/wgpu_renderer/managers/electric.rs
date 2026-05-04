use crate::wgpu_renderer::bind_group::{
    ElectricBindGroups, ElectricStorageBuffers, GlobalBindGroupLayout,
};
use shaders_shared::Charge;
use wgpu::Device;
use winit::dpi::PhysicalSize;

pub struct ElectricManager {
    pub charges: Vec<Charge>,
    pub electric_storage_buffers: ElectricStorageBuffers,
    pub electric_bind_groups: ElectricBindGroups,
    pub size: PhysicalSize<u32>,
}

impl ElectricManager {
    pub fn new(
        device: &Device,
        global_bind_group_layout: &GlobalBindGroupLayout,
        size: PhysicalSize<u32>,
        charges: Vec<Charge>,
    ) -> Self {
        let electric_storage_buffers =
            global_bind_group_layout.create_electric_buffers(device, size, charges.clone());
        let electric_bind_groups =
            global_bind_group_layout.create_electric_bind_groups(device, &electric_storage_buffers);

        Self {
            electric_bind_groups,
            electric_storage_buffers,
            size,
            charges,
        }
    }

    pub fn resize(
        &mut self,
        device: &Device,
        new_size: PhysicalSize<u32>,
        global_bind_group_layout: &GlobalBindGroupLayout,
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
        }

        self.electric_storage_buffers = global_bind_group_layout.create_electric_buffers(
            device,
            new_size,
            self.charges.clone(),
        );
        self.electric_bind_groups = global_bind_group_layout
            .create_electric_bind_groups(device, &self.electric_storage_buffers);
        self.size = new_size
    }
}
