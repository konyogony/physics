use wgpu::{
    ColorTargetState, ColorWrites, ComputePipeline, ComputePipelineDescriptor, FragmentState,
    FrontFace, MultisampleState, PolygonMode, PrimitiveState, PrimitiveTopology, RenderPass,
    RenderPipelineDescriptor, TextureFormat, VertexState,
};
use wgpu::{ComputePass, Device, PipelineLayoutDescriptor, RenderPipeline, include_spirv};
use winit::dpi::PhysicalSize;

use crate::wgpu_renderer::bind_groups::GlobalBindGroupLayout;
use crate::wgpu_renderer::bind_groups::constants::ConstantsBindGroups;
use crate::wgpu_renderer::bind_groups::electric::ElectricBindGroups;

pub struct ElectricPipeline {
    charge_render_pipeline: RenderPipeline,
    tracing_render_pipeline: RenderPipeline,
    plate_render_pipeline: RenderPipeline,
    compute_field_pipeline: ComputePipeline,
    compute_plates_tracing_pipeline: ComputePipeline,
    compute_charges_tracing_pipeline: ComputePipeline,
    compute_potential_pipeline: ComputePipeline,
}

impl ElectricPipeline {
    pub fn new(
        device: &Device,
        global_bind_group_layout: &GlobalBindGroupLayout,
        out_format: TextureFormat,
    ) -> anyhow::Result<Self> {
        let shader_module =
            device.create_shader_module(include_spirv!(env!("ELECTRIC_SHADER_PATH")));

        let layout_render = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("ElectricRenderPipelineLayout"),
            bind_group_layouts: &[
                Some(&global_bind_group_layout.constants.constants),
                Some(&global_bind_group_layout.electric.electric),
            ],
            immediate_size: 0,
        });

        let layout_compute = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("ElectricComputePipelineLayout"),
            bind_group_layouts: &[
                Some(&global_bind_group_layout.constants.constants),
                Some(&global_bind_group_layout.electric.electric),
            ],
            immediate_size: 0,
        });

        let compute_potential_pipeline =
            device.create_compute_pipeline(&ComputePipelineDescriptor {
                label: Some("ElectricPotentialComputePipeline"),
                layout: Some(&layout_compute),
                module: &shader_module,
                entry_point: Some("electric_potential_cs"),
                compilation_options: Default::default(),
                cache: None,
            });
        let compute_field_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("ElectricFieldComputePipeline"),
            layout: Some(&layout_compute),
            module: &shader_module,
            entry_point: Some("electric_field_cs"),
            compilation_options: Default::default(),
            cache: None,
        });

        let compute_plates_tracing_pipeline =
            device.create_compute_pipeline(&ComputePipelineDescriptor {
                label: Some("TracingPlatesComputePipeline"),
                layout: Some(&layout_compute),
                module: &shader_module,
                entry_point: Some("electric_tracing_plates_cs"),
                compilation_options: Default::default(),
                cache: None,
            });
        let compute_charges_tracing_pipeline =
            device.create_compute_pipeline(&ComputePipelineDescriptor {
                label: Some("TracingChargesComputePipeline"),
                layout: Some(&layout_compute),
                module: &shader_module,
                entry_point: Some("electric_tracing_charges_cs"),
                compilation_options: Default::default(),
                cache: None,
            });

        let plate_render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("ElectricPlatesRenderPipeline"),
            layout: Some(&layout_render),
            vertex: VertexState {
                // Pass in that shader
                module: &shader_module,
                entry_point: Some("electric_plates_vs"),
                compilation_options: Default::default(),
                buffers: &[],
            },
            // Default culling & settings.
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState::default(),
            fragment: Some(FragmentState {
                // Pass in that shader
                module: &shader_module,
                entry_point: Some("electric_fs"),
                compilation_options: Default::default(),
                targets: &[Some(ColorTargetState {
                    format: out_format,
                    blend: None,
                    write_mask: ColorWrites::ALL,
                })],
            }),
            multiview_mask: None,
            cache: None,
        });

        let charge_render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("ElectricChargeRenderPipeline"),
            layout: Some(&layout_render),
            vertex: VertexState {
                // Pass in that shader
                module: &shader_module,
                entry_point: Some("electric_vs"),
                compilation_options: Default::default(),
                buffers: &[],
            },
            // Default culling & settings.
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState::default(),
            fragment: Some(FragmentState {
                // Pass in that shader
                module: &shader_module,
                entry_point: Some("electric_fs"),
                compilation_options: Default::default(),
                targets: &[Some(ColorTargetState {
                    format: out_format,
                    blend: None,
                    write_mask: ColorWrites::ALL,
                })],
            }),
            multiview_mask: None,
            cache: None,
        });

        let tracing_render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("ElectricTracingRenderPipeline"),
            layout: Some(&layout_render),
            vertex: VertexState {
                // Pass in that shader
                module: &shader_module,
                entry_point: Some("electric_tracing_vs"),
                compilation_options: Default::default(),
                buffers: &[],
            },
            // Default culling & settings.
            primitive: PrimitiveState {
                topology: PrimitiveTopology::LineList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState::default(),
            fragment: Some(FragmentState {
                // Pass in that shader
                module: &shader_module,
                entry_point: Some("electric_tracing_fs"),
                compilation_options: Default::default(),
                targets: &[Some(ColorTargetState {
                    format: out_format,
                    blend: None,
                    write_mask: ColorWrites::ALL,
                })],
            }),
            multiview_mask: None,
            cache: None,
        });

        Ok(Self {
            compute_field_pipeline,
            compute_potential_pipeline,
            charge_render_pipeline,
            tracing_render_pipeline,
            plate_render_pipeline,
            compute_plates_tracing_pipeline,
            compute_charges_tracing_pipeline,
        })
    }

    pub fn draw_charge(
        &self,
        rpass: &mut RenderPass<'_>,
        constants_bind_groups: &ConstantsBindGroups,
        electric_bind_groups: &ElectricBindGroups,
        num_charges: u32,
        polygon_vertices: u32,
    ) {
        rpass.set_pipeline(&self.charge_render_pipeline);
        rpass.set_bind_group(0, &constants_bind_groups.constants, &[]);
        rpass.set_bind_group(1, &electric_bind_groups.electric_compute, &[]);

        rpass.draw(0..polygon_vertices, 0..num_charges);
    }

    pub fn draw_plates(
        &self,
        rpass: &mut RenderPass<'_>,
        constants_bind_groups: &ConstantsBindGroups,
        electric_bind_groups: &ElectricBindGroups,
        num_plates: u32,
    ) {
        rpass.set_pipeline(&self.plate_render_pipeline);
        rpass.set_bind_group(0, &constants_bind_groups.constants, &[]);
        rpass.set_bind_group(1, &electric_bind_groups.electric_compute, &[]);

        rpass.draw(0..6, 0..num_plates);
    }

    pub fn draw_tracing(
        &self,
        rpass: &mut RenderPass<'_>,
        constants_bind_groups: &ConstantsBindGroups,
        electric_bind_groups: &ElectricBindGroups,
        max_steps: usize,
        total_instances: u32,
    ) {
        rpass.set_pipeline(&self.tracing_render_pipeline);
        rpass.set_bind_group(0, &constants_bind_groups.constants, &[]);
        rpass.set_bind_group(1, &electric_bind_groups.electric_compute, &[]);

        rpass.draw(0..((max_steps as u32 - 1) * 2), 0..total_instances);
    }

    pub fn compute_potential(
        &mut self,
        cpass: &mut ComputePass<'_>,
        constants_bind_groups: &ConstantsBindGroups,
        electric_bind_groups: &ElectricBindGroups,
        size: PhysicalSize<u32>,
    ) {
        cpass.set_pipeline(&self.compute_potential_pipeline);
        cpass.set_bind_group(0, &constants_bind_groups.constants, &[]);
        cpass.set_bind_group(1, &electric_bind_groups.electric_compute, &[]);

        cpass.dispatch_workgroups(size.width.div_ceil(16), size.height.div_ceil(16), 1);
    }

    pub fn compute_field(
        &mut self,
        cpass: &mut ComputePass<'_>,
        constants_bind_groups: &ConstantsBindGroups,
        electric_bind_groups: &ElectricBindGroups,
        size: PhysicalSize<u32>,
    ) {
        cpass.set_pipeline(&self.compute_field_pipeline);
        cpass.set_bind_group(0, &constants_bind_groups.constants, &[]);
        cpass.set_bind_group(1, &electric_bind_groups.electric_compute, &[]);

        cpass.dispatch_workgroups(size.width.div_ceil(16), size.height.div_ceil(16), 1);
    }

    pub fn compute_tracing_plates(
        &mut self,
        cpass: &mut ComputePass<'_>,
        constants_bind_groups: &ConstantsBindGroups,
        electric_bind_groups: &ElectricBindGroups,
        num_plates: u32,
        num_segments_per_plate: u32,
    ) {
        cpass.set_pipeline(&self.compute_plates_tracing_pipeline);
        cpass.set_bind_group(0, &constants_bind_groups.constants, &[]);
        cpass.set_bind_group(1, &electric_bind_groups.electric_compute, &[]);

        cpass.dispatch_workgroups((num_plates * num_segments_per_plate).div_ceil(128), 1, 1);
    }

    pub fn compute_tracing_charges(
        &mut self,
        cpass: &mut ComputePass<'_>,
        constants_bind_groups: &ConstantsBindGroups,
        electric_bind_groups: &ElectricBindGroups,
        num_charges: u32,
        num_particles_per_charge: u32,
    ) {
        cpass.set_pipeline(&self.compute_charges_tracing_pipeline);
        cpass.set_bind_group(0, &constants_bind_groups.constants, &[]);
        cpass.set_bind_group(1, &electric_bind_groups.electric_compute, &[]);

        cpass.dispatch_workgroups((num_charges * num_particles_per_charge).div_ceil(128), 1, 1);
    }
}
