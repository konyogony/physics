use crate::wgpu_renderer::{
    bind_group::{ElectricBindGroups, GlobalBindGroupLayout},
    texture::{ElectricStorageTextures, Texture},
};
use wgpu::Device;

pub struct ElectricManager {
    pub electric_storage_textures: ElectricStorageTextures,
    pub electric_bind_groups: ElectricBindGroups,
    pub size: (u32, u32),
}

impl ElectricManager {
    pub fn new(
        device: &Device,
        global_bind_group_layout: &GlobalBindGroupLayout,
        (width, height): (u32, u32),
    ) -> Self {
        let electric_storage_textures = Texture::create_electric_textures(device, (width, height));
        let electric_bind_groups = global_bind_group_layout
            .create_electric_bind_groups(device, &electric_storage_textures);

        Self {
            electric_bind_groups,
            electric_storage_textures,
            size: (width, height),
        }
    }
}
