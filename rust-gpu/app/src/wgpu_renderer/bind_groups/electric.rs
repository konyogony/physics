use shaders_shared::{Charge, Field, Plate, Segment, TracePoint};
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, Buffer, BufferBinding, BufferBindingType,
    BufferDescriptor, BufferUsages, Device, Queue, ShaderStages,
};
use winit::dpi::PhysicalSize;

/*
 *  0  charges       — read-only source charges
 *  1  plates        — read-only conductive plates
 *  2  segments      — read-only segments of plates
 *  3  field         — writable electric field output
 *  4  tracing       — writable tracing output
 *  5  potential     — writable potential grid
 */

#[derive(Debug, Clone)]
pub struct ElectricBindGroupsLayout {
    pub electric: BindGroupLayout,
}

#[derive(Debug, Clone)]
pub struct ElectricStorageBuffers {
    pub charges: Buffer,
    pub plates: Buffer,
    pub segments: Buffer,
    pub field: Buffer,
    pub tracing: Buffer,
    pub potential: Buffer,
}

#[derive(Debug, Clone)]
pub struct ElectricBindGroups {
    pub electric_compute: BindGroup,
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
                // Segments list.
                BindGroupLayoutEntry {
                    binding: 2,
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
                    binding: 3,
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
                    binding: 4,
                    visibility: ShaderStages::COMPUTE | ShaderStages::VERTEX_FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // Potential
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
        let electric_compute = device.create_bind_group(&BindGroupDescriptor {
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
                        buffer: &electric_storage_buffers.segments,
                        offset: 0,
                        size: None,
                    }),
                },
                BindGroupEntry {
                    binding: 3,
                    resource: BindingResource::Buffer(BufferBinding {
                        buffer: &electric_storage_buffers.field,
                        offset: 0,
                        size: None,
                    }),
                },
                BindGroupEntry {
                    binding: 4,
                    resource: BindingResource::Buffer(BufferBinding {
                        buffer: &electric_storage_buffers.tracing,
                        offset: 0,
                        size: None,
                    }),
                },
                BindGroupEntry {
                    binding: 5,
                    resource: BindingResource::Buffer(BufferBinding {
                        buffer: &electric_storage_buffers.potential,
                        offset: 0,
                        size: None,
                    }),
                },
            ],
        });

        ElectricBindGroups { electric_compute }
    }

    pub fn create_electric_buffers(
        &self,
        device: &Device,
        size: PhysicalSize<u32>,
        queue: &Queue,
        charges_buffer_size: u64,
        plates_buffer_size: u64,
        segments_buffer_size: u64,
        charges_vec: Vec<Charge>,
        plates_vec: Vec<Plate>,
        segments_vec: Vec<Segment>,
        max_steps: usize,
        num_particles_per_charge: u32,
    ) -> (ElectricStorageBuffers, usize) {
        let max_index = size.width * size.height;

        let initial_particles =
            (charges_vec.len() * num_particles_per_charge as usize) + segments_vec.len();
        let needed_elements = (initial_particles * max_steps).max(1);

        let trace_size = std::mem::size_of::<TracePoint>();
        let max_allowable_elements =
            (device.limits().max_storage_buffer_binding_size as usize) / trace_size;

        let capacity_elements = (needed_elements + needed_elements / 5)
            .max(4096)
            .min(max_allowable_elements);

        let tracing_buffer_size = (capacity_elements * trace_size) as u64;

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

        let segments = device.create_buffer(&BufferDescriptor {
            label: Some("SegmentsBuffer"),
            usage: BufferUsages::COPY_DST | BufferUsages::STORAGE,
            size: segments_buffer_size,
            mapped_at_creation: false,
        });

        queue.write_buffer(&segments, 0, bytemuck::cast_slice(&segments_vec));

        let field = device.create_buffer(&BufferDescriptor {
            label: Some("FieldBuffer"),
            usage: BufferUsages::COPY_DST | BufferUsages::STORAGE,
            size: (std::mem::size_of::<Field>() * max_index as usize) as u64,
            mapped_at_creation: false,
        });

        let tracing = device.create_buffer(&BufferDescriptor {
            label: Some("TracingBuffer"),
            usage: BufferUsages::COPY_DST | BufferUsages::STORAGE,
            size: tracing_buffer_size,
            mapped_at_creation: false,
        });

        let potential = device.create_buffer(&BufferDescriptor {
            label: Some("PotentialBuffer"),
            usage: BufferUsages::STORAGE
                | BufferUsages::VERTEX
                | BufferUsages::COPY_DST
                | BufferUsages::COPY_SRC,
            size: (std::mem::size_of::<f32>() * max_index as usize) as u64,
            mapped_at_creation: false,
        });

        (
            ElectricStorageBuffers {
                charges,
                plates,
                segments,
                field,
                tracing,
                potential,
            },
            capacity_elements,
        )
    }
}
