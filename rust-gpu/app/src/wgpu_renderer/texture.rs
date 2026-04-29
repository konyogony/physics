use wgpu::{
    Device, Sampler, SamplerDescriptor, TextureDescriptor, TextureFormat, TextureUsages,
    TextureView, TextureViewDescriptor,
};

#[derive(Debug, Clone)]
pub struct ElectricStorageTextures {
    pub density: Texture,
    pub potential: Texture,
    pub field: Texture,
}

#[derive(Debug, Clone)]
pub struct Texture {
    pub format: TextureFormat,
    pub view: TextureView,
    pub sampler: Sampler,
}

impl Texture {
    pub fn new(
        device: &Device,
        label: &str,
        (width, height): (u32, u32),
        format: TextureFormat,
    ) -> Self {
        let texture_size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&TextureDescriptor {
            label: Some(label),
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: TextureUsages::STORAGE_BINDING
                | TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST,
            view_formats: &[],
        });

        let view = texture.create_view(&TextureViewDescriptor::default());

        let sampler = device.create_sampler(&SamplerDescriptor {
            label: Some(label),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            ..Default::default()
        });

        Self {
            format,
            view,
            sampler,
        }
    }

    pub fn create_electric_textures(
        device: &Device,
        (width, height): (u32, u32),
    ) -> ElectricStorageTextures {
        let density = Texture::new(
            device,
            "ElectricDensityTexture",
            (width, height),
            TextureFormat::R32Float,
        );

        let potential = Texture::new(
            device,
            "ElectricPotentialTexture",
            (width, height),
            TextureFormat::R32Float,
        );

        let field = Texture::new(
            device,
            "ElectricFieldTexture",
            (width, height),
            TextureFormat::Rgba32Float,
        );

        ElectricStorageTextures {
            density,
            potential,
            field,
        }
    }
}
