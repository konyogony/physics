use crate::Model;
use crate::TIME_SCALE;
use nannou::{App, Draw};
use nannou::{color::Srgba, geom::Point2};

pub struct Particle {
    pub radius: f32,
    pub color: Srgba,
    pub position: Point2,
    pub hidden: bool,
}

impl Particle {
    pub fn new(radius: f32, color: Srgba, position: Point2) -> Self {
        Self {
            radius,
            color,
            position,
            hidden: false,
        }
    }

    pub fn update_pos<F>(&mut self, app: &App, arrow_function: F)
    where
        // x, y, t
        F: Fn(f32, f32, f32) -> Point2,
    {
        let velocity = arrow_function(self.position.x, self.position.y, app.time);
        let dt = app.duration.since_prev_update.as_secs_f32() * TIME_SCALE;
        let displacement = velocity * dt;
        self.position += displacement;
    }

    pub fn draw(&self, _app: &App, _model: &Model, draw: &Draw) {
        if !self.hidden {
            draw.ellipse()
                .xy(self.position)
                .radius(self.radius)
                .color(self.color);
        }
    }
}
