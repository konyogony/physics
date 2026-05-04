use shaders_shared::{Charge, Field, ShaderConstants};
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, Buffer, BufferBinding, BufferBindingType,
    BufferDescriptor, BufferUsages, Device, ShaderStages,
    util::{BufferInitDescriptor, DeviceExt},
};
use winit::dpi::PhysicalSize;

// Global Bind Group LAYOUT struct, will hold layouts for each bind group.
// To create a bind group, a layout is needed. Layout shows how data is arranged,
// while the bind group itself just places the data in right position.
#[derive(Debug, Clone)]
pub struct GlobalBindGroupLayout {
    pub constants: BindGroupLayout,
    // Since layout is same both ways, no need to create AB & BA versions
    // However, we will still need to create separate render & compute layouts, due to read_only
    // flags and gpu rules.
    pub particles_render: BindGroupLayout,
    pub particles_compute: BindGroupLayout,
    pub electric: BindGroupLayout,
}

// Now we split a single global bind group which holds multiple bind groups into their own buffer
// and bind groups struct.

// Constants

#[derive(Debug, Clone)]
pub struct ConstantsBuffers {
    pub constants: Buffer,
}

#[derive(Debug, Clone)]
pub struct ConstantsBindGroups {
    pub constants: BindGroup,
}

// Particles

#[derive(Debug, Clone)]
pub struct ParticleBuffers {
    pub particles_buffer_a: Buffer,
    pub particles_buffer_b: Buffer,
}

#[derive(Debug, Clone)]
pub struct ParticleBindGroups {
    // We will need 2 different bind groups, since we are doing the ping pong model.
    // AND different for render and compute
    pub particles_render_a: BindGroup,
    pub particles_render_b: BindGroup,
    pub particles_compute_ab: BindGroup,
    pub particles_compute_ba: BindGroup,
}

// Electrostatics

#[derive(Debug, Clone)]
pub struct ElectricStorageBuffers {
    pub charges: Buffer,
    pub potential: Buffer,
    pub field: Buffer,
}

#[derive(Debug, Clone)]
pub struct ElectricBindGroups {
    // Will hold density, potential & field
    pub electric: BindGroup,
}

impl GlobalBindGroupLayout {
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

        let particles_render = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("ParticlesBindGroupLayoutRender"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX_FRAGMENT | ShaderStages::COMPUTE,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let particles_compute = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("ParticlesBindGroupLayoutCompute"),
            entries: &[
                BindGroupLayoutEntry {
                    // Always read from this
                    binding: 0,
                    visibility: ShaderStages::VERTEX_FRAGMENT | ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::VERTEX_FRAGMENT | ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

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
                // Potential
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::COMPUTE | ShaderStages::VERTEX_FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: false },
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
            ],
        });

        Self {
            electric,
            constants,
            particles_render,
            particles_compute,
        }
    }

    // Instead of create_buffer_INIT, we use the normal one and just specify the max capacity.
    pub fn create_particle_buffers(&self, device: &Device, size: u64) -> ParticleBuffers {
        let particles_buffer_a = device.create_buffer(&BufferDescriptor {
            label: Some("ParticlesBufferA"),
            size,
            mapped_at_creation: false,
            usage: BufferUsages::STORAGE | BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });

        let particles_buffer_b = device.create_buffer(&BufferDescriptor {
            label: Some("ParticlesBufferB"),
            size,
            mapped_at_creation: false,
            usage: BufferUsages::STORAGE | BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });

        ParticleBuffers {
            particles_buffer_a,
            particles_buffer_b,
        }
    }

    pub fn create_particle_bind_groups(
        &self,
        device: &Device,
        particle_buffers: &ParticleBuffers,
    ) -> ParticleBindGroups {
        let particles_render_a = device.create_bind_group(&BindGroupDescriptor {
            label: Some("ParticlesBindGroupRenderA"),
            layout: &self.particles_render,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: BindingResource::Buffer(BufferBinding {
                    buffer: &particle_buffers.particles_buffer_a,
                    offset: 0,
                    size: None,
                }),
            }],
        });

        let particles_render_b = device.create_bind_group(&BindGroupDescriptor {
            label: Some("ParticlesBindGroupRenderBA"),
            layout: &self.particles_render,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: BindingResource::Buffer(BufferBinding {
                    buffer: &particle_buffers.particles_buffer_b,
                    offset: 0,
                    size: None,
                }),
            }],
        });

        let particles_compute_ab = device.create_bind_group(&BindGroupDescriptor {
            label: Some("ParticlesBindGroupComputeAB"),
            layout: &self.particles_compute,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::Buffer(BufferBinding {
                        buffer: &particle_buffers.particles_buffer_a,
                        offset: 0,
                        size: None,
                    }),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Buffer(BufferBinding {
                        buffer: &particle_buffers.particles_buffer_b,
                        offset: 0,
                        size: None,
                    }),
                },
            ],
        });

        let particles_compute_ba = device.create_bind_group(&BindGroupDescriptor {
            label: Some("ParticlesBindGroupComputeBA"),
            layout: &self.particles_compute,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::Buffer(BufferBinding {
                        buffer: &particle_buffers.particles_buffer_b,
                        offset: 0,
                        size: None,
                    }),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Buffer(BufferBinding {
                        buffer: &particle_buffers.particles_buffer_a,
                        offset: 0,
                        size: None,
                    }),
                },
            ],
        });

        ParticleBindGroups {
            particles_render_a,
            particles_render_b,
            particles_compute_ab,
            particles_compute_ba,
        }
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

    pub fn create_electric_bind_groups(
        &self,
        device: &Device,
        electric_storage_buffers: &ElectricStorageBuffers,
    ) -> ElectricBindGroups {
        let electric = device.create_bind_group(&BindGroupDescriptor {
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
                        buffer: &electric_storage_buffers.potential,
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
            ],
        });

        ElectricBindGroups { electric }
    }

    pub fn create_electric_buffers(
        &self,
        device: &Device,
        size: PhysicalSize<u32>,
        charges_vec: Vec<Charge>,
    ) -> ElectricStorageBuffers {
        let max_index = size.width * size.height;

        let charges = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("ChargeBuffer"),
            usage: BufferUsages::COPY_DST | BufferUsages::STORAGE,
            contents: bytemuck::cast_slice(&charges_vec),
        });

        let potential = device.create_buffer(&BufferDescriptor {
            label: Some("PotentialBuffer"),
            usage: BufferUsages::COPY_DST | BufferUsages::STORAGE,
            size: (std::mem::size_of::<f32>() * max_index as usize) as u64,
            mapped_at_creation: false,
        });

        let field = device.create_buffer(&BufferDescriptor {
            label: Some("FieldBuffer"),
            usage: BufferUsages::COPY_DST | BufferUsages::STORAGE,
            size: (std::mem::size_of::<Field>() * max_index as usize) as u64,
            mapped_at_creation: false,
        });

        ElectricStorageBuffers {
            charges,
            potential,
            field,
        }
    }
}
