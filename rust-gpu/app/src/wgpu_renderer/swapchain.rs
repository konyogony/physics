use anyhow::Context;
use std::sync::Arc;
use wgpu::{
    Adapter, CurrentSurfaceTexture, Device, Instance, Surface, SurfaceConfiguration, TextureFormat,
    TextureView,
};
use winit::dpi::PhysicalSize;
use winit::window::Window;

// So a swap chain manager basically manages all the boring stuff
pub struct SwapchainManager<'a> {
    instance: Instance,
    adapter: Adapter,
    device: Device,
    window: Arc<Window>,
    surface: Surface<'a>,
    format: TextureFormat,

    // state below
    active: Option<ActiveConfiguration>,
    should_recreate: bool,
}

pub struct ActiveConfiguration {
    size: PhysicalSize<u32>,
    surface_config: SurfaceConfiguration,
}

impl<'a> SwapchainManager<'a> {
    // Creating a new swap chain manager instance
    pub fn new(
        instance: Instance,
        adapter: Adapter,
        device: Device,
        window: Arc<Window>,
        surface: Surface<'a>,
    ) -> Self {
        // We just pass in all the stuff we created in the state & get the caps here
        let caps = surface.get_capabilities(&adapter);
        let mut manager = SwapchainManager {
            instance,
            adapter,
            device,
            window,
            surface,
            format: caps.formats[0],
            active: None,
            should_recreate: false,
        };
        let size = manager.get_size();
        manager
            .configure_surface(size)
            .expect("Initial configuration failed");
        manager
    }

    // Method to recerate self
    pub fn recreate(&mut self) -> anyhow::Result<()> {
        let size = self.get_size();
        // Reset flag
        self.should_recreate = false;
        // Re-configure the surface with new size
        self.configure_surface(size)?;
        Ok(())
    }

    // Setter
    pub fn set_should_recreate_true(&mut self) {
        self.should_recreate = true;
    }

    // Getters (which i hate but this came as default so im not changing it..)
    pub fn get_format(&self) -> TextureFormat {
        self.format
    }
    pub fn get_size(&self) -> PhysicalSize<u32> {
        self.window.inner_size()
    }
    pub fn get_config(&self) -> Option<SurfaceConfiguration> {
        if let Some(active) = &self.active {
            return Some(active.surface_config.clone());
        }
        None
    }

    // Takes in a function that it can only be called once,
    // takes in a texture view (as a storage texture im guessing)
    pub fn render<F>(&mut self, render_function: F) -> anyhow::Result<()>
    where
        F: FnOnce(TextureView) -> anyhow::Result<()>,
    {
        // Get the size
        let size = self.get_size();

        match &self.active {
            // If active, but wrong size -> recreate
            Some(active) if active.size != size => {
                self.set_should_recreate_true();
            }
            // If not active -> recreate
            None => self.set_should_recreate_true(),
            _ => {}
        }

        // If flag is active -> recreate
        if self.should_recreate {
            self.recreate()?;
        }

        match self.surface.get_current_texture() {
            // If able to acquire texture
            CurrentSurfaceTexture::Success(surface_texture) => {
                // Get the view of this texture & feed it to the render function passed in.
                let output_view =
                    surface_texture
                        .texture
                        .create_view(&wgpu::TextureViewDescriptor {
                            format: Some(self.format),
                            ..wgpu::TextureViewDescriptor::default()
                        });
                let result = render_function(output_view);
                surface_texture.present();
                result?;
            }
            CurrentSurfaceTexture::Occluded | CurrentSurfaceTexture::Timeout => (),
            CurrentSurfaceTexture::Suboptimal(_) | CurrentSurfaceTexture::Outdated => {
                // If something is wrong, recreate
                self.set_should_recreate_true();
            }
            CurrentSurfaceTexture::Validation => {
                // Panic
                anyhow::bail!("Validation error during surface texture acquisition")
            }
            CurrentSurfaceTexture::Lost => {
                // If we are lost???
                // Then just recreate the surface entirely
                self.surface = self.instance.create_surface(self.window.clone())?;
                self.set_should_recreate_true();
            }
        };

        Ok(())
    }

    // Configure the surface.. duh. with correct size
    fn configure_surface(&mut self, size: PhysicalSize<u32>) -> anyhow::Result<()> {
        // Generate a new configuration
        let mut surface_config = self
            .surface
            .get_default_config(&self.adapter, size.width, size.height)
            .with_context(|| {
                format!(
                    "Incompatible adapter for surface, returned capabilities: {:?}",
                    self.surface.get_capabilities(&self.adapter)
                )
            })?;

        // force srgb surface format
        surface_config.view_formats.push(self.format);
        // limit framerate to vsync (Dont like it, might change to Mailbox)
        surface_config.present_mode = wgpu::PresentMode::AutoVsync;
        self.surface.configure(&self.device, &surface_config);

        // A configuration was made, it can now be used.
        self.active = Some(ActiveConfiguration {
            surface_config,
            size,
        });
        Ok(())
    }
}
