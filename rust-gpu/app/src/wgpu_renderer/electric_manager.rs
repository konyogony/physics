use crate::wgpu_renderer::bind_group::{
    ElectricBindGroups, ElectricStorageBuffers, GlobalBindGroupLayout,
};
use shaders_shared::Charge;
use wgpu::Device;
use winit::dpi::PhysicalSize;

pub struct ElectricManager {
    pub electric_storage_buffers: ElectricStorageBuffers,
    pub electric_bind_groups: ElectricBindGroups,
    pub size: PhysicalSize<u32>,
}

impl ElectricManager {
    pub fn new(
        device: &Device,
        global_bind_group_layout: &GlobalBindGroupLayout,
        size: PhysicalSize<u32>,
        charges_vec: Vec<Charge>,
    ) -> Self {
        let electric_storage_buffers =
            global_bind_group_layout.create_electric_buffers(device, size, charges_vec);
        let electric_bind_groups =
            global_bind_group_layout.create_electric_bind_groups(device, &electric_storage_buffers);

        Self {
            electric_bind_groups,
            electric_storage_buffers,
            size,
        }
    }

    pub fn resize(
        &mut self,
        device: &Device,
        new_size: PhysicalSize<u32>,
        global_bind_group_layout: &GlobalBindGroupLayout,
        charges_vec: Vec<Charge>,
    ) {
        self.electric_storage_buffers =
            global_bind_group_layout.create_electric_buffers(device, new_size, charges_vec);
        self.electric_bind_groups = global_bind_group_layout
            .create_electric_bind_groups(device, &self.electric_storage_buffers);
        self.size = new_size
    }
}
