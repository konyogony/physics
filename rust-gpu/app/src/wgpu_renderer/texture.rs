//use shaders_shared::{Charge, Field};
//use wgpu::{
//    Buffer, BufferDescriptor, BufferUsages, Device, Sampler, SamplerDescriptor, TextureDescriptor,
//    TextureFormat, TextureUsages, TextureView, TextureViewDescriptor,
//    util::{BufferInitDescriptor, DeviceExt},
//};
//
//#[derive(Debug, Clone)]
//pub struct Texture {
//    pub format: TextureFormat,
//    pub view: TextureView,
//    pub sampler: Sampler,
//}
//
//impl Texture {
//    pub fn new(
//        device: &Device,
//        label: &str,
//        (width, height): (u32, u32),
//        format: TextureFormat,
//    ) -> Self {
//        let texture_size = wgpu::Extent3d {
//            width,
//            height,
//            depth_or_array_layers: 1,
//        };
//
//        let texture = device.create_texture(&TextureDescriptor {
//            label: Some(label),
//            size: texture_size,
//            mip_level_count: 1,
//            sample_count: 1,
//            dimension: wgpu::TextureDimension::D2,
//            format,
//            usage: TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING,
//            view_formats: &[],
//        });
//
//        let view = texture.create_view(&TextureViewDescriptor::default());
//
//        let sampler = device.create_sampler(&SamplerDescriptor {
//            label: Some(label),
//            address_mode_u: wgpu::AddressMode::ClampToEdge,
//            address_mode_v: wgpu::AddressMode::ClampToEdge,
//            address_mode_w: wgpu::AddressMode::ClampToEdge,
//            mag_filter: wgpu::FilterMode::Nearest,
//            min_filter: wgpu::FilterMode::Nearest,
//            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
//            ..Default::default()
//        });
//
//        Self {
//            format,
//            view,
//            sampler,
//        }
//    }
//}
