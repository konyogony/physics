use shaders_shared::ShaderConstants;
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, Buffer, BufferBinding, BufferBindingType,
    BufferUsages, Device, ShaderStages,
    util::{BufferInitDescriptor, DeviceExt},
};

/*
 *  0  constants  — read-only simulation constants
 */

#[derive(Debug, Clone)]
pub struct ConstantsBindGroupsLayout {
    pub constants: BindGroupLayout,
}

#[derive(Debug, Clone)]
pub struct ConstantsBuffers {
    pub constants: Buffer,
}

#[derive(Debug, Clone)]
pub struct ConstantsBindGroups {
    pub constants: BindGroup,
}

impl ConstantsBindGroupsLayout {
    pub fn new(device: &Device) -> Self {
        // The standard layout, holding the constants for the simulation
        let constants = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("ConstantsBindGroupLayout"),
            entries: &[BindGroupLayoutEntry {
                // First binding
                binding: 0,
                // Accessible both in vertex & fragment shader
                visibility: ShaderStages::VERTEX_FRAGMENT | ShaderStages::COMPUTE,
                // Read-only storage buffer
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        Self { constants }
    }

    pub fn create_constant_buffers(
        &self,
        device: &Device,
        shader_constants: &ShaderConstants,
    ) -> ConstantsBuffers {
        let constants = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("ConstantsBuffer"),
            contents: bytemuck::bytes_of(shader_constants),
            usage: BufferUsages::STORAGE | BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });

        ConstantsBuffers { constants }
    }

    // We pass in all the storage buffers into here,
    // this will create the global bind group.
    pub fn create_constant_bind_groups(
        &self,
        device: &Device,
        constants_buffer: &ConstantsBuffers,
    ) -> ConstantsBindGroups {
        let constants = device.create_bind_group(&BindGroupDescriptor {
            label: Some("ConstantsBindGroup"),
            layout: &self.constants,
            entries: &[BindGroupEntry {
                binding: 0,
                // Pass in the storage buffer
                resource: BindingResource::Buffer(BufferBinding {
                    buffer: &constants_buffer.constants,
                    offset: 0,
                    size: None,
                }),
            }],
        });

        ConstantsBindGroups { constants }
    }
}
