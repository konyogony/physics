use crate::wgpu_renderer::bind_group::{
    ElectricBindGroups, ElectricStorageBuffers, GlobalBindGroupLayout,
};
use shaders::Charge;
use wgpu::Device;

pub struct ElectricManager {
    pub electric_storage_buffers: ElectricStorageBuffers,
    pub electric_bind_groups: ElectricBindGroups,
    pub size: (u32, u32),
}

impl ElectricManager {
    pub fn new(
        device: &Device,
        global_bind_group_layout: &GlobalBindGroupLayout,
        (width, height): (u32, u32),
        charges_vec: Vec<Charge>,
    ) -> Self {
        let electric_storage_buffers =
            global_bind_group_layout.create_electric_buffers(device, (width, height), charges_vec);
        let electric_bind_groups =
            global_bind_group_layout.create_electric_bind_groups(device, &electric_storage_buffers);

        Self {
            electric_bind_groups,
            electric_storage_buffers,
            size: (width, height),
        }
    }
}
