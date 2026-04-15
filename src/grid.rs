use crate::Model;
use crate::utils::smoothstep;
use nannou::{
    App, Draw,
    color::{self, Srgba, hsv},
    geom::{Point2, pt2},
    math::map_range,
};

// Grid is only responsible for creating & displaying the grid + vectors
pub struct Grid {
    pub line_distance: usize,
    pub color_axis: Srgba,
    pub color_grid: Srgba,
    pub color_highlight: Srgba,
    pub highlight_distnace: usize,
    pub min_arrow_scale: f32,
    pub arrow_scaling: f32,
}

impl Grid {
    pub fn new(
        line_distance: usize,
        color_axis: Srgba,
        color_grid: Srgba,
        color_highlight: Srgba,
        highlight_distnace: usize,
        min_arrow_scale: f32,
        arrow_scaling: f32,
    ) -> Self {
        Self {
            line_distance,
            color_highlight,
            color_axis,
            color_grid,
            highlight_distnace,
            min_arrow_scale,
            arrow_scaling,
        }
    }

    pub fn draw_grid(&self, app: &App, _model: &Model, draw: &Draw) {
        // Get window information
        let window_rect = app.window_rect();
        let (min_x, max_x, min_y, max_y) = (
            window_rect.left(),
            window_rect.right(),
            window_rect.bottom(),
            window_rect.top(),
        );

        draw.background().color(color::BLACK);

        // y-axis
        draw.line()
            .start(pt2(0.0, min_y))
            .end(pt2(0.0, max_y))
            .color(self.color_axis)
            .weight(1.5);
        // x-axis
        draw.line()
            .start(pt2(min_x, 0.0))
            .end(pt2(max_x, 0.0))
            .color(self.color_axis)
            .weight(1.5);

        // Vertical lines
        for i in (self.line_distance..(max_x as usize)).step_by(self.line_distance) {
            // Get pure index
            let index = i / self.line_distance;
            // Color appropriately
            let color = if index.is_multiple_of(self.highlight_distnace) {
                self.color_highlight
            } else {
                self.color_grid
            };

            let x_coord = i as f32;
            draw.line()
                .start(pt2(x_coord, min_y))
                .end(pt2(x_coord, max_y))
                .color(color)
                .weight(1.0);
            draw.line()
                .start(pt2(-x_coord, min_y))
                .end(pt2(-x_coord, max_y))
                .color(color)
                .weight(1.0);
        }

        // Horizontal lines
        for i in (self.line_distance..(max_y as usize)).step_by(self.line_distance) {
            let index = i / self.line_distance;
            let color = if index.is_multiple_of(self.highlight_distnace) {
                self.color_highlight
            } else {
                self.color_grid
            };

            let y_coord = i as f32;
            draw.line()
                .start(pt2(min_x, y_coord))
                .end(pt2(max_x, y_coord))
                .color(color)
                .weight(1.0);
            draw.line()
                .start(pt2(min_x, -y_coord))
                .end(pt2(max_x, -y_coord))
                .color(color)
                .weight(1.0);
        }
    }

    // Use generics to pass in a function
    pub fn draw_vectors<F>(&self, app: &App, model: &Model, draw: &Draw, arrow_function: F)
    where
        F: Fn(f32, f32) -> Point2,
    {
        // Get window information
        let window_rect = app.window_rect();
        let (min_x, max_x, min_y, max_y) = (
            window_rect.left(),
            window_rect.right(),
            window_rect.bottom(),
            window_rect.top(),
        );

        // Have to do this, or otherwise vectors wont align with intersections
        let start_x = (min_x / self.line_distance as f32).ceil() as i32 * self.line_distance as i32;
        let start_y = (min_y / self.line_distance as f32).ceil() as i32 * self.line_distance as i32;

        for x in (start_x..max_x as i32).step_by(self.line_distance) {
            for y in (start_y..max_y as i32).step_by(self.line_distance) {
                let x_f32 = x as f32;
                let y_f32 = y as f32;

                let relative_origin = pt2(x_f32, y_f32);
                let vec = arrow_function(x_f32, y_f32);

                let len = vec.length();
                let strength = len / (len + model.color_value);

                let t = smoothstep(0.0, 1.0, strength);
                let t_clamped = t.clamp(self.min_arrow_scale, 1.0);
                let hue = map_range(t, 0.0, 1.0, 0.6, 0.0);
                let color = hsv(hue, 0.8, 0.9);

                let vec_scaled = vec.normalize_or_zero() * self.arrow_scaling;
                let end = relative_origin + vec_scaled;

                draw.arrow()
                    .start(relative_origin)
                    .end(end)
                    .head_width(t_clamped * 5.0)
                    .stroke_weight(t_clamped * 3.0)
                    .color(color);
            }
        }
    }
}
