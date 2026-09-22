// this SUPPOSEDLY will hide terminal showing up for windows...
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// The main function will just run the wgpu renderer
pub fn main() -> anyhow::Result<()> {
    app::wgpu_renderer::main()
}
