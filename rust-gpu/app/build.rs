use cargo_gpu_install::install::Install;
use cargo_gpu_install::spirv_builder::{ShaderPanicStrategy, SpirvMetadata};
use std::path::PathBuf;

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

// This file just builds the shader defined in the shaders/src/lib.rs
pub fn main() -> anyhow::Result<()> {
    // We fetch the vertex path using env variable
    let crate_path = [MANIFEST_DIR, "..", "shaders"]
        .iter()
        .copied()
        .collect::<PathBuf>();

    // Installed need packages...?
    let install = Install::from_shader_crate(crate_path.clone())
        .within_build_script()
        .run()?;
    // Create the builder itself, which will use spirv and vulkan 1.3
    let mut builder = install.to_spirv_builder(crate_path, "spirv-unknown-vulkan1.3");
    // Some env variables
    builder.build_script.defaults = true;
    builder.shader_panic_strategy = ShaderPanicStrategy::SilentExit;
    builder.spirv_metadata = SpirvMetadata::Full;

    // Build the shader & get its result
    let compile_result = builder.build()?;
    // Get where the spv is stored
    let spv_path = compile_result.module.unwrap_single();
    // Set an env variables so the app can fetch from it instead.
    println!("cargo::rustc-env=SHADER_SPV_PATH={}", spv_path.display());
    // Exit succesfully.
    Ok(())
}
