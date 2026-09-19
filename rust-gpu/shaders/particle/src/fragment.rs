use crate::*;

#[spirv(fragment(entry_point_name = "particle_fs"))]
pub fn particle_fs(#[spirv(location = 0)] vtx_color: Vec3, output: &mut Vec4) {
    *output = vtx_color.extend(1.0);
}
