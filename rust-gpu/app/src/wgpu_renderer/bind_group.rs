use shaders::ShaderConstants;
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, Buffer, BufferBinding, BufferBindingType,
    BufferUsages, Device, ShaderStages, util::BufferInitDescriptor, util::DeviceExt,
};
// This file is just responsible for creating the:
// 1. Layout of the bind group
// 2. The storage buffer
// 3. Creating the bind group itself

// A tuple struct which holds the whole bind group layout.
// Dont really like structs, but I will roll with it here.
#[derive(Debug, Clone)]
pub struct GlobalBindGroupLayout(pub BindGroupLayout);

impl GlobalBindGroupLayout {
    // Creating a new global bind group LAYOUT
    pub fn new(device: &Device) -> Self {
        // THe standart layout
        Self(device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("GlobalBindGroupLayout"),
            entries: &[BindGroupLayoutEntry {
                // First binding
                binding: 0,
                // Accessible both in vertex & fragment shader
                visibility: ShaderStages::VERTEX_FRAGMENT,
                // Read-only storage buffer
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        }))
    }

    // Creating the storage buffer itself, which holds the shader constants (which are passed in
    // from the renderer)
    pub fn create_bind_group(
        &self,
        device: &Device,
        shader_constants: &ShaderConstants,
    ) -> GlobalBindGroup {
        let shader_constants = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("ShaderConstants"),
            contents: bytemuck::bytes_of(shader_constants),
            usage: BufferUsages::STORAGE,
        });
        self.create_bind_group_from_buffer(device, &shader_constants)
    }

    // Creating a bind group from a storage buffer
    pub fn create_bind_group_from_buffer(
        &self,
        device: &Device,
        shader_constants: &Buffer,
    ) -> GlobalBindGroup {
        GlobalBindGroup(device.create_bind_group(&BindGroupDescriptor {
            label: Some("GlobalBindGroup"),
            // Access first element (the bind group layout) from the struct
            layout: &self.0,
            entries: &[BindGroupEntry {
                binding: 0,
                // Pass in the storage buffer
                resource: BindingResource::Buffer(BufferBinding {
                    buffer: shader_constants,
                    offset: 0,
                    size: None,
                }),
            }],
        }))
    }
}

#[derive(Debug, Clone)]
pub struct GlobalBindGroup(pub BindGroup);
