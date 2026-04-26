// So this is the shader code itself, the fragment and vertex shaders are stored here.
// They can recieve inputs and give outputs by making them mutable and using pointers.
// No std librarires are allowed here.

use crate::shared::{
    SDF, ShaderConstants, antialias, antialias_no_fwidth, hsv, map_range, smoothstep,
};
use glam::{Vec2, Vec4, Vec4Swizzles};
#[allow(unused_imports)]
use spirv_std::num_traits::Float;
use spirv_std::spirv;

// colors RGBA
const GRID_COLOR: Vec4 = Vec4::new(0.3, 0.3, 0.3, 0.05);
const AXIS_COLOR: Vec4 = Vec4::new(1.0, 1.0, 1.0, 0.8);
const BG_COLOR: Vec4 = Vec4::new(0.0, 0.0, 0.0, 1.0);
const HIGHLIGHT_COLOR: Vec4 = Vec4::new(0.0, 1.0, 1.0, 0.4);
const GRID_THICKNESS_PX: f32 = 1.0;
const GRID_SPACING_PX: f32 = 30.0;
const ARROW_THICKNESS_PX: f32 = 1.0;
const ARROW_HEAD_WIDTH_PX: f32 = 4.0;
const ARROW_HEAD_HEIGHT_PX: f32 = 10.0;
const HIGHLIGHT_SQUARES: f32 = 3.0;
const ARROW_SCALE: f32 = 25.0;
const MIN_ARROW_SCALE: f32 = 0.7;
const COLOR_VALUE: f32 = 2.5;

#[spirv(vertex(entry_point_name = "grid_vs"))]
pub fn grid_vs(#[spirv(vertex_index)] vert_id: i32, #[spirv(position)] vtx_pos: &mut Vec4) {
    // fancy bitwise manipulations
    let uv = Vec2::new(((vert_id << 1) & 2) as f32, (vert_id & 2) as f32);
    // Mapping to the correct range
    let pos = Vec2::new(uv.x * 2.0 - 1.0, uv.y * 2.0 - 1.0);
    // Basically, we are covering the entire screen here.
    *vtx_pos = pos.extend(0.0).extend(1.0);
}

// AI Generated function for testing
pub fn arrow_fn(x: f32, y: f32, t: f32) -> Vec2 {
    let p = Vec2::new(x, y) * 0.006;
    let vx = (p.y * 4.0 + t).cos() + (p.x + p.y + t * 0.5).sin();
    let vy = (p.x * 4.0 - t).sin() + (p.x - p.y - t * 0.8).cos();
    Vec2::new(vx, vy)
}

#[spirv(fragment(entry_point_name = "grid_fs"))]
pub fn grid_fs(
    #[spirv(descriptor_set = 0, binding = 0, storage_buffer)] constants: &ShaderConstants,
    #[spirv(frag_coord)] frag_coords: Vec4,
    output: &mut Vec4,
) {
    // Center first
    let uv = frag_coords / Vec4::new(constants.width as f32, constants.height as f32, 0.0, 0.0);
    let centered_uv = uv - 0.5;

    // Convert to pixels
    let px_x = centered_uv.x * constants.width as f32;
    let px_y = -centered_uv.y * constants.height as f32;

    // Draw the grid
    Grid::draw_grid(px_x, px_y, output);

    // Drawing the vectors
    Grid::draw_vectors(px_x, px_y, constants.time, output);
}

pub struct Grid;

impl Grid {
    pub fn draw_grid(px_x: f32, px_y: f32, output: &mut Vec4) {
        let output_color = BG_COLOR.xyz();

        // Get how many times spacing wraps.
        let grid_distance_x = (px_x % GRID_SPACING_PX).abs();
        let grid_distance_y = (px_y % GRID_SPACING_PX).abs();
        // Get closest one
        let grid_distance = grid_distance_x.min(grid_distance_y);
        // Make sure lines dont look ugly and appear on all screen sizes.
        let grid_alpha = antialias(grid_distance, GRID_THICKNESS_PX);

        // Same for highlights, but different scale
        let highlight_distance_x = (px_x % (GRID_SPACING_PX * HIGHLIGHT_SQUARES)).abs();
        let highlight_distance_y = (px_y % (GRID_SPACING_PX * HIGHLIGHT_SQUARES)).abs();
        let highlight_distance = highlight_distance_x.min(highlight_distance_y);
        let highlight_alpha = antialias(highlight_distance, GRID_THICKNESS_PX);

        let axis_distance = px_x.abs().min(px_y.abs());
        let axis_alpha = antialias(axis_distance, GRID_THICKNESS_PX);

        // Now the alpha channels are applied SEPERATLY to preserve the original alpha
        // Lerp allows us to apply a mask with specific colors.
        *output = output_color
            .lerp(GRID_COLOR.xyz(), grid_alpha * GRID_COLOR.w)
            .lerp(HIGHLIGHT_COLOR.xyz(), highlight_alpha * HIGHLIGHT_COLOR.w)
            .lerp(AXIS_COLOR.xyz(), axis_alpha * AXIS_COLOR.w)
            .extend(1.0);
    }

    pub fn draw_vectors(px_x: f32, px_y: f32, time: f32, output: &mut Vec4) {
        let current_pos = Vec2::new(px_x, px_y);

        let index_x = (px_x / GRID_SPACING_PX).floor();
        let index_y = (px_y / GRID_SPACING_PX).floor();

        // Basically, now instead of just getting the closest point (index * GRID_SPACING), which
        // will cut off the lines, we will loop through the neughboring points aswell, by adding or
        // subtracting the GRID_SPACING
        for i in -1..=1 {
            for j in -1..=1 {
                let start_point = Vec2::new(
                    index_x * GRID_SPACING_PX + GRID_SPACING_PX * i as f32,
                    index_y * GRID_SPACING_PX + GRID_SPACING_PX * j as f32,
                );

                // Evaluate the arrow function from the starting point to acquire final pos
                // (relative to the start pos)
                let vec = arrow_fn(start_point.x, start_point.y, time);
                let len = vec.length();

                // Get the unit vector of the vector
                let dir = vec.normalize();
                // Get the the perpendicular direction. (I actually used the 2D rotation matrix to
                // acquire the coordinates for fun)
                let perp_dir = Vec2::new(dir.y, -dir.x);

                // Now actually bring this vec to the correct position in space
                // Make sure its normalized and the correct scaling is applied
                let relative_vec = start_point + dir * ARROW_SCALE;

                // Same logic as in nannou version,
                // we map and scale and do stuff to the magnitude to acquire a color value.
                let strength = len / (len + COLOR_VALUE);
                let t = smoothstep(0.0, 1.0, strength);
                let t_clamped = t.clamp(MIN_ARROW_SCALE, 1.0);
                let hue = map_range(t, 0.0, 1.0, 0.6, 0.0);
                let color = hsv(hue, 0.8, 0.9);

                // Get the rectange sdf between start pos and the end point, and current pixel
                let line_sdf = SDF::sdf_rectangle(start_point, relative_vec, current_pos);
                let line_alpha = antialias_no_fwidth(line_sdf, ARROW_THICKNESS_PX);

                // Get the triangle sdf.
                // A -> From the tip and to the left
                // B -> From the tip and to the right
                // We use the perp_dir to get both of those
                // C -> From the tip and a bit further
                let triangle_sdf = SDF::sdf_triangle(
                    relative_vec + perp_dir * ARROW_HEAD_WIDTH_PX * t_clamped,
                    relative_vec - perp_dir * ARROW_HEAD_WIDTH_PX * t_clamped,
                    relative_vec + dir * ARROW_HEAD_HEIGHT_PX * t_clamped,
                    current_pos,
                );
                let triangle_alpha = antialias_no_fwidth(triangle_sdf, ARROW_THICKNESS_PX);

                *output = output.lerp(color, line_alpha);
                *output = output.lerp(color, triangle_alpha)
            }
        }
    }
}
