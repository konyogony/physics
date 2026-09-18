use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, Buffer, BufferBinding, BufferBindingType,
    BufferDescriptor, BufferUsages, Device, ShaderStages,
};

/*
 *  Render:
 *   0  particles      — read-only particle buffer
 *
 *  Compute:
 *   0  particles in   — read-only particle input
 *   1  particles out  — writable particle output
 *
 *   AB: A → read,  B → write
 *   BA: B → read,  A → write
 */

#[derive(Debug, Clone)]
pub struct ParticlesBindGroupsLayout {
    // Since layout is same both ways, no need to create AB & BA versions
    // However, we will still need to create separate render & compute layouts, due to read_only
    // flags and gpu rules.
    pub particles_render: BindGroupLayout,
    pub particles_compute: BindGroupLayout,
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

#[derive(Debug, Clone)]
pub struct ParticleBuffers {
    pub particles_buffer_a: Buffer,
    pub particles_buffer_b: Buffer,
}

impl ParticlesBindGroupsLayout {
    pub fn new(device: &Device) -> Self {
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

        Self {
            particles_render,
            particles_compute,
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

    pub fn create_particle_buffers(&self, device: &Device, size: u64) -> ParticleBuffers {
        let particles_buffer_a = device.create_buffer(&BufferDescriptor {
            label: Some("ParticlesBufferA"),
            size,
            mapped_at_creation: false,
            usage: BufferUsages::STORAGE
                | BufferUsages::VERTEX
                | BufferUsages::COPY_DST
                | BufferUsages::COPY_SRC,
        });

        let particles_buffer_b = device.create_buffer(&BufferDescriptor {
            label: Some("ParticlesBufferB"),
            size,
            mapped_at_creation: false,
            usage: BufferUsages::STORAGE
                | BufferUsages::VERTEX
                | BufferUsages::COPY_DST
                | BufferUsages::COPY_SRC,
        });

        ParticleBuffers {
            particles_buffer_a,
            particles_buffer_b,
        }
    }
}
