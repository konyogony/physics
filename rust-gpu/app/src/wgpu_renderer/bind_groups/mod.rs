use crate::wgpu_renderer::bind_groups::{
    constants::ConstantsBindGroupsLayout, electric::ElectricBindGroupsLayout,
    particle::ParticlesBindGroupsLayout,
};
use wgpu::Device;

pub mod constants;
pub mod electric;
pub mod particle;

// Global Bind Group LAYOUT struct, will hold layouts for each bind group.
// To create a bind group, a layout is needed. Layout shows how data is arranged,
// while the bind group itself just places the data in right position.
#[derive(Debug, Clone)]
pub struct GlobalBindGroupLayout {
    pub particles: ParticlesBindGroupsLayout,
    pub electric: ElectricBindGroupsLayout,
    pub constants: ConstantsBindGroupsLayout,
}

// Constants
impl GlobalBindGroupLayout {
    pub fn new(device: &Device) -> Self {
        let particles = ParticlesBindGroupsLayout::new(device);
        let electric = ElectricBindGroupsLayout::new(device);
        let constants = ConstantsBindGroupsLayout::new(device);

        Self {
            particles,
            electric,
            constants,
        }
    }
}
