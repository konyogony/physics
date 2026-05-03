use crate::wgpu_renderer::bind_group::{
    ConstantsBindGroups, ElectricBindGroups, GlobalBindGroupLayout,
};
use shaders_shared::ShaderConstants;
use wgpu::{ComputePass, Device, PipelineLayoutDescriptor, include_spirv};
use wgpu::{ComputePipeline, ComputePipelineDescriptor};

pub struct ElectricPipeline {
    compute_potential_pipeline: ComputePipeline,
    compute_field_pipeline: ComputePipeline,
}

impl ElectricPipeline {
    pub fn new(
        device: &Device,
        global_bind_group_layout: &GlobalBindGroupLayout,
    ) -> anyhow::Result<Self> {
        let shader_module =
            device.create_shader_module(include_spirv!(env!("ELECTRIC_SHADER_PATH")));

        let layout_compute = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("ParticleComputePipelineLayout"),
            bind_group_layouts: &[
                Some(&global_bind_group_layout.constants),
                Some(&global_bind_group_layout.electric),
            ],
            immediate_size: size_of::<ShaderConstants>() as u32,
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

        Ok(Self {
            compute_field_pipeline,
            compute_potential_pipeline,
        })
    }

    pub fn compute_potential(
        &mut self,
        cpass: &mut ComputePass<'_>,
        constants_bind_groups: &ConstantsBindGroups,
        electric_bind_groups: &ElectricBindGroups,
        (width, height): (u32, u32),
    ) {
        cpass.set_pipeline(&self.compute_potential_pipeline);
        cpass.set_bind_group(0, &constants_bind_groups.constants, &[]);
        cpass.set_bind_group(1, &electric_bind_groups.electric, &[]);

        cpass.dispatch_workgroups(width.div_ceil(16), height.div_ceil(16), 1);
    }

    pub fn compute_field(
        &mut self,
        cpass: &mut ComputePass<'_>,
        constants_bind_groups: &ConstantsBindGroups,
        electric_bind_groups: &ElectricBindGroups,
        (width, height): (u32, u32),
    ) {
        cpass.set_pipeline(&self.compute_field_pipeline);
        cpass.set_bind_group(0, &constants_bind_groups.constants, &[]);
        cpass.set_bind_group(1, &electric_bind_groups.electric, &[]);

        cpass.dispatch_workgroups(width.div_ceil(16), height.div_ceil(16), 1);
    }
}
