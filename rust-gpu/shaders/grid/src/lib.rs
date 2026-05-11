#![no_std]
// Seperate shader for the particles
#![allow(clippy::too_many_arguments)]

// So this is the shader code itself, the fragment and vertex shaders are stored here.
// They can recieve inputs and give outputs by making them mutable and using pointers.
// No std librarires are allowed here.

use glam::{Vec2, Vec4, Vec4Swizzles};
use shaders_shared::{
    ARROW_HEAD_HEIGHT_PX, ARROW_HEAD_WIDTH_PX, ARROW_SCALE, ARROW_THICKNESS_PX, AXIS_COLOR,
    BG_COLOR, Field, GRID_COLOR, GRID_SPACING_PX, GRID_THICKNESS_PX, HIGHLIGHT_COLOR,
    HIGHLIGHT_SQUARES, MIN_ARROW_SCALE, SDF, ShaderConstants, antialias, antialias_no_fwidth, hsv,
    map_range, smoothstep,
};
#[allow(unused_imports)]
use spirv_std::num_traits::Float;
use spirv_std::spirv;

#[spirv(vertex(entry_point_name = "grid_vs"))]
pub fn grid_vs(#[spirv(vertex_index)] vert_id: i32, #[spirv(position)] vtx_pos: &mut Vec4) {
    // fancy bitwise manipulations
    let uv = Vec2::new(((vert_id << 1) & 2) as f32, (vert_id & 2) as f32);
    // Mapping to the correct range
    let pos = Vec2::new(uv.x * 2.0 - 1.0, uv.y * 2.0 - 1.0);
    // Basically, we are covering the entire screen here.
    *vtx_pos = pos.extend(0.0).extend(1.0);
}

#[spirv(fragment(entry_point_name = "grid_fs"))]
pub fn grid_fs(
    #[spirv(descriptor_set = 0, binding = 0, storage_buffer)] constants: &ShaderConstants,
    #[spirv(descriptor_set = 1, binding = 1, storage_buffer)] potential_field: &mut [f32],
    #[spirv(descriptor_set = 1, binding = 2, storage_buffer)] electric_field: &mut [Field],
    #[spirv(frag_coord)] frag_coords: Vec4,
    output: &mut Vec4,
) {
    // Extract raw pixel values.
    let px_x = frag_coords.x;
    let px_y = frag_coords.y;

    let center_x = constants.width as f32 / 2.0;
    let center_y = constants.height as f32 / 2.0;

    // We will use centered coordinates for grid & vecs.
    let centered_x = px_x - center_x;
    let centered_y = px_y - center_y;

    if constants.draw_options.draw_grid == 1 {
        let output_color = BG_COLOR.xyz();

        // Get how many times spacing wraps.
        let grid_distance_x = (centered_x % GRID_SPACING_PX).abs();
        let grid_distance_y = (centered_y % GRID_SPACING_PX).abs();
        // Get closest one
        let grid_distance = grid_distance_x.min(grid_distance_y);
        // Make sure lines dont look ugly and appear on all screen sizes.
        let grid_alpha = antialias(grid_distance, GRID_THICKNESS_PX);

        // Same for highlights, but different scale
        let highlight_distance_x = (centered_x % (GRID_SPACING_PX * HIGHLIGHT_SQUARES)).abs();
        let highlight_distance_y = (centered_y % (GRID_SPACING_PX * HIGHLIGHT_SQUARES)).abs();
        let highlight_distance = highlight_distance_x.min(highlight_distance_y);
        let highlight_alpha = antialias(highlight_distance, GRID_THICKNESS_PX);

        let axis_distance = centered_x.abs().min(centered_y.abs());
        let axis_alpha = antialias(axis_distance, GRID_THICKNESS_PX);

        // Now the alpha channels are applied SEPERATLY to preserve the original alpha
        // Lerp allows us to apply a mask with specific colors.
        *output = output_color
            .lerp(GRID_COLOR.xyz(), grid_alpha * GRID_COLOR.w)
            .lerp(HIGHLIGHT_COLOR.xyz(), highlight_alpha * HIGHLIGHT_COLOR.w)
            .lerp(AXIS_COLOR.xyz(), axis_alpha * AXIS_COLOR.w)
            .extend(1.0);
    }

    if constants.draw_options.draw_vec == 1 {
        let current_pos = Vec2::new(centered_x, centered_y);
        // Drawing the vectors
        let index_x = (current_pos.x / GRID_SPACING_PX).floor();
        let index_y = (current_pos.y / GRID_SPACING_PX).floor();

        // Basically, now instead of just getting the closest point (index * GRID_SPACING), which
        // will cut off the lines, we will loop through the neughboring points aswell, by adding or
        // subtracting the GRID_SPACING
        for i in -1..=1 {
            for j in -1..=1 {
                let start_point = Vec2::new(
                    index_x * GRID_SPACING_PX + GRID_SPACING_PX * i as f32,
                    index_y * GRID_SPACING_PX + GRID_SPACING_PX * j as f32,
                );

                // Evaluate the ELECTRIC FIELD from the starting point to acquire final pos
                // (relative to the start pos)
                // Also convert back to space coordinates, yes looks UGLY i know.
                let x = ((start_point.x + constants.width as f32 / 2.0) as i32)
                    .min(constants.width as i32 - 1_i32);
                let y = ((start_point.y + constants.height as f32 / 2.0) as i32)
                    .min(constants.height as i32 - 1_i32);

                if x < 0 || y < 0 || x >= constants.width as i32 || y >= constants.height as i32 {
                    continue;
                }
                let index = x + y * constants.width as i32;
                let field_reading = electric_field[index as usize].field;
                let vec = Vec2::new(field_reading[0], field_reading[1]);
                let len = vec.length().max(0.001);

                // Get the unit vector of the vector
                let dir = vec / len;
                // Get the the perpendicular direction. (I actually used the 2D rotation matrix to
                // acquire the coordinates for fun)
                let perp_dir = Vec2::new(dir.y, -dir.x);

                // Now actually bring this vec to the correct position in space
                // Make sure its normalized and the correct scaling is applied
                let relative_vec = start_point + dir * ARROW_SCALE;

                // Same logic as in nannou version,
                // we map and scale and do stuff to the magnitude to acquire a color value.
                let strength = len / (len + constants.color_value);
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

    if constants.draw_options.draw_potential == 1 {
        let index = px_x.floor() as i32 + px_y.floor() as i32 * constants.width as i32;

        // Safety check
        if (index as usize) < electric_field.len() && (index as usize) < potential_field.len() {
            let field_reading = electric_field[index as usize].field;

            // We can use vec and its strength to keep consistant thickness.
            let vec = Vec2::new(field_reading[0], field_reading[1]);
            let vec_strength = vec.length().max(0.0001);

            let current_potential = potential_field[index as usize];
            let mut alpha = 0.0;
            let mut target_potential = -1000.0;

            while target_potential <= 1000.0 {
                let potential_difference = (target_potential - current_potential).abs();
                alpha += antialias(potential_difference / vec_strength, 1.0);
                target_potential += 250.0;
            }
            *output = output.lerp(AXIS_COLOR, alpha);
        }
    }
}
