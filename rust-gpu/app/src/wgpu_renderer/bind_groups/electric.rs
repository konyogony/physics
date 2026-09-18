use shaders_shared::{Charge, Field, MAX_CHARGES, Plate, TracePoint};
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, Buffer, BufferBinding, BufferBindingType,
    BufferDescriptor, BufferUsages, Device, Queue, ShaderStages,
};
use winit::dpi::PhysicalSize;

/*
 *  0  charges       — read-only source charges
 *  1  plates        — read-only conductive plates
 *  2  field         — writable electric field output
 *  3  tracing       — writable tracing output
 *  4  potential A   — read-only potential grid
 *  5  potential B   — writable potential grid
 *
 *   AB: A → read,  B → write
 *   BA: B → read,  A → write
 */

#[derive(Debug, Clone)]
pub struct ElectricBindGroupsLayout {
    pub electric: BindGroupLayout,
}

#[derive(Debug, Clone)]
pub struct ElectricStorageBuffers {
    pub charges: Buffer,
    pub plates: Buffer,
    pub field: Buffer,
    pub tracing: Buffer,
    pub potential_a: Buffer,
    pub potential_b: Buffer,
}

#[derive(Debug, Clone)]
pub struct ElectricBindGroups {
    pub electric_compute_ab: BindGroup,
    pub electric_compute_ba: BindGroup,
}

impl ElectricBindGroupsLayout {
    pub fn new(device: &Device) -> Self {
        let electric = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("ElectricBindGroupLayout"),
            entries: &[
                // Charge list.
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::COMPUTE | ShaderStages::VERTEX_FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // Plates list.
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::COMPUTE | ShaderStages::VERTEX_FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // Field
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::COMPUTE | ShaderStages::VERTEX_FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // Tracing
                BindGroupLayoutEntry {
                    binding: 3,
                    visibility: ShaderStages::COMPUTE | ShaderStages::VERTEX_FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // Potential A (read only TRUE)
                BindGroupLayoutEntry {
                    binding: 4,
                    visibility: ShaderStages::COMPUTE | ShaderStages::VERTEX_FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // Potential B (read only FALSE)
                BindGroupLayoutEntry {
                    binding: 5,
                    visibility: ShaderStages::COMPUTE | ShaderStages::VERTEX_FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        Self { electric }
    }

    pub fn create_electric_bind_groups(
        &self,
        device: &Device,
        electric_storage_buffers: &ElectricStorageBuffers,
    ) -> ElectricBindGroups {
        let electric_compute_ab = device.create_bind_group(&BindGroupDescriptor {
            label: Some("ElectricBindGroup"),
            layout: &self.electric,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::Buffer(BufferBinding {
                        buffer: &electric_storage_buffers.charges,
                        offset: 0,
                        size: None,
                    }),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Buffer(BufferBinding {
                        buffer: &electric_storage_buffers.plates,
                        offset: 0,
                        size: None,
                    }),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: BindingResource::Buffer(BufferBinding {
                        buffer: &electric_storage_buffers.field,
                        offset: 0,
                        size: None,
                    }),
                },
                BindGroupEntry {
                    binding: 3,
                    resource: BindingResource::Buffer(BufferBinding {
                        buffer: &electric_storage_buffers.tracing,
                        offset: 0,
                        size: None,
                    }),
                },
                BindGroupEntry {
                    binding: 4,
                    resource: BindingResource::Buffer(BufferBinding {
                        buffer: &electric_storage_buffers.potential_a,
                        offset: 0,
                        size: None,
                    }),
                },
                BindGroupEntry {
                    binding: 5,
                    resource: BindingResource::Buffer(BufferBinding {
                        buffer: &electric_storage_buffers.potential_b,
                        offset: 0,
                        size: None,
                    }),
                },
            ],
        });

        let electric_compute_ba = device.create_bind_group(&BindGroupDescriptor {
            label: Some("ElectricBindGroup"),
            layout: &self.electric,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::Buffer(BufferBinding {
                        buffer: &electric_storage_buffers.charges,
                        offset: 0,
                        size: None,
                    }),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Buffer(BufferBinding {
                        buffer: &electric_storage_buffers.plates,
                        offset: 0,
                        size: None,
                    }),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: BindingResource::Buffer(BufferBinding {
                        buffer: &electric_storage_buffers.field,
                        offset: 0,
                        size: None,
                    }),
                },
                BindGroupEntry {
                    binding: 3,
                    resource: BindingResource::Buffer(BufferBinding {
                        buffer: &electric_storage_buffers.tracing,
                        offset: 0,
                        size: None,
                    }),
                },
                BindGroupEntry {
                    binding: 4,
                    resource: BindingResource::Buffer(BufferBinding {
                        buffer: &electric_storage_buffers.potential_b,
                        offset: 0,
                        size: None,
                    }),
                },
                BindGroupEntry {
                    binding: 5,
                    resource: BindingResource::Buffer(BufferBinding {
                        buffer: &electric_storage_buffers.potential_a,
                        offset: 0,
                        size: None,
                    }),
                },
            ],
        });

        ElectricBindGroups {
            electric_compute_ab,
            electric_compute_ba,
        }
    }

    pub fn create_electric_buffers(
        &self,
        device: &Device,
        size: PhysicalSize<u32>,
        queue: &Queue,
        charges_buffer_size: u64,
        plates_buffer_size: u64,
        charges_vec: Vec<Charge>,
        plates_vec: Vec<Plate>,
        max_steps: usize,
        num_particles_per_charge: u32,
    ) -> ElectricStorageBuffers {
        let max_index = size.width * size.height;
        let max_trace_index =
            (MAX_CHARGES as usize + 1) * max_steps * num_particles_per_charge as usize;

        let charges = device.create_buffer(&BufferDescriptor {
            label: Some("ChargeBuffer"),
            usage: BufferUsages::COPY_DST | BufferUsages::STORAGE,
            size: charges_buffer_size,
            mapped_at_creation: false,
        });

        queue.write_buffer(&charges, 0, bytemuck::cast_slice(&charges_vec));

        let plates = device.create_buffer(&BufferDescriptor {
            label: Some("PlatesBuffer"),
            usage: BufferUsages::COPY_DST | BufferUsages::STORAGE,
            size: plates_buffer_size,
            mapped_at_creation: false,
        });

        queue.write_buffer(&plates, 0, bytemuck::cast_slice(&plates_vec));

        let field = device.create_buffer(&BufferDescriptor {
            label: Some("FieldBuffer"),
            usage: BufferUsages::COPY_DST | BufferUsages::STORAGE,
            size: (std::mem::size_of::<Field>() * max_index as usize) as u64,
            mapped_at_creation: false,
        });

        let tracing = device.create_buffer(&BufferDescriptor {
            label: Some("TracingBuffer"),
            usage: BufferUsages::COPY_DST | BufferUsages::STORAGE,
            size: (std::mem::size_of::<TracePoint>() * max_trace_index) as u64,
            mapped_at_creation: false,
        });

        let potential_a = device.create_buffer(&BufferDescriptor {
            label: Some("PotentialBufferA"),
            usage: BufferUsages::STORAGE
                | BufferUsages::VERTEX
                | BufferUsages::COPY_DST
                | BufferUsages::COPY_SRC,
            size: (std::mem::size_of::<f32>() * max_index as usize) as u64,
            mapped_at_creation: false,
        });

        let potential_b = device.create_buffer(&BufferDescriptor {
            label: Some("PotentialBufferB"),
            usage: BufferUsages::STORAGE
                | BufferUsages::VERTEX
                | BufferUsages::COPY_DST
                | BufferUsages::COPY_SRC,
            size: (std::mem::size_of::<f32>() * max_index as usize) as u64,
            mapped_at_creation: false,
        });

        ElectricStorageBuffers {
            charges,
            plates,
            field,
            tracing,
            potential_a,
            potential_b,
        }
    }
}
