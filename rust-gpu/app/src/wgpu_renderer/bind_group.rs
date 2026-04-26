use shaders::{particle::Particle, shared::ShaderConstants};
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, Buffer, BufferBinding, BufferBindingType,
    BufferUsages, Device, ShaderStages, util::BufferInitDescriptor, util::DeviceExt,
};

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
}

// Global Bind group wil hold multiple bind groups, such as constants & particles
// Each bind group can have multiple bindings, for example, particles will have a read & write
#[derive(Debug, Clone)]
pub struct GlobalBindGroup {
    pub constants: BindGroup,
    // We will need 2 different bind groups, since we are doing the ping pong model.
    // AND different for render and compute
    pub particles_render_ab: BindGroup,
    pub particles_render_ba: BindGroup,
    pub particles_compute_ab: BindGroup,
    pub particles_compute_ba: BindGroup,
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
            entries: &[
                BindGroupLayoutEntry {
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
                        ty: BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
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
            constants,
            particles_render,
            particles_compute,
        }
    }

    // We pass in raw values into here, which will get converted to buffers, then converted to bind
    // groups, then passed out.
    pub fn create_bind_groups(
        &self,
        device: &Device,
        shader_constants: &ShaderConstants,
        particles: &[Particle],
    ) -> GlobalBindGroup {
        let constant_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("ConstantsBuffer"),
            contents: bytemuck::bytes_of(shader_constants),
            usage: BufferUsages::STORAGE | BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });

        let particles_buffer_a = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("ParticlesBufferA"),
            contents: bytemuck::cast_slice(particles),
            usage: BufferUsages::STORAGE | BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });
        let particles_buffer_b = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("ParticlesBufferB"),
            contents: bytemuck::cast_slice(particles),
            usage: BufferUsages::STORAGE | BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });

        self.create_bind_group_from_buffer(
            device,
            &constant_buffer,
            &particles_buffer_a,
            &particles_buffer_b,
        )
    }

    // We pass in all the storage buffers into here,
    // this will create the global bind group.
    pub fn create_bind_group_from_buffer(
        &self,
        device: &Device,
        constants_buffer: &Buffer,
        particles_buffer_a: &Buffer,
        particles_buffer_b: &Buffer,
    ) -> GlobalBindGroup {
        let constants = device.create_bind_group(&BindGroupDescriptor {
            label: Some("ConstantsBindGroup"),
            // Access first element (the bind group layout) from the struct
            layout: &self.constants,
            entries: &[BindGroupEntry {
                binding: 0,
                // Pass in the storage buffer
                resource: BindingResource::Buffer(BufferBinding {
                    buffer: constants_buffer,
                    offset: 0,
                    size: None,
                }),
            }],
        });

        let particles_render_ab = device.create_bind_group(&BindGroupDescriptor {
            label: Some("ParticlesBindGroupRenderAB"),
            layout: &self.particles_render,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::Buffer(BufferBinding {
                        buffer: particles_buffer_a,
                        offset: 0,
                        size: None,
                    }),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Buffer(BufferBinding {
                        buffer: particles_buffer_b,
                        offset: 0,
                        size: None,
                    }),
                },
            ],
        });

        let particles_render_ba = device.create_bind_group(&BindGroupDescriptor {
            label: Some("ParticlesBindGroupRenderBA"),
            layout: &self.particles_render,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::Buffer(BufferBinding {
                        buffer: particles_buffer_b,
                        offset: 0,
                        size: None,
                    }),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Buffer(BufferBinding {
                        buffer: particles_buffer_a,
                        offset: 0,
                        size: None,
                    }),
                },
            ],
        });

        let particles_compute_ab = device.create_bind_group(&BindGroupDescriptor {
            label: Some("ParticlesBindGroupComputeAB"),
            layout: &self.particles_compute,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::Buffer(BufferBinding {
                        buffer: particles_buffer_a,
                        offset: 0,
                        size: None,
                    }),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Buffer(BufferBinding {
                        buffer: particles_buffer_b,
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
                        buffer: particles_buffer_b,
                        offset: 0,
                        size: None,
                    }),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Buffer(BufferBinding {
                        buffer: particles_buffer_a,
                        offset: 0,
                        size: None,
                    }),
                },
            ],
        });

        GlobalBindGroup {
            constants,
            particles_render_ab,
            particles_render_ba,
            particles_compute_ab,
            particles_compute_ba,
        }
    }
}
