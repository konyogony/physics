use crate::*;

#[spirv(fragment(entry_point_name = "electric_fs"))]
pub fn electric_fs(#[spirv(location = 0)] vtx_color: Vec3, output: &mut Vec4) {
    *output = vtx_color.extend(1.0);
}

#[spirv(fragment(entry_point_name = "electric_tracing_fs"))]
pub fn electric_tracing_fs(output: &mut Vec4) {
    *output = Vec4::new(1.0, 1.0, 1.0, 1.0);
}
