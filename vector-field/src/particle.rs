use crate::DEFAULT_LINE_DISTANCE;
use crate::FIELD_MODE;
use crate::FieldMode;
use crate::Model;
use crate::TIME_SCALE;
use nannou::{App, Draw};
use nannou::{color::Srgba, geom::Point2};

pub struct Particle {
    pub radius: f32,
    pub color: Srgba,
    pub position: Point2,
    pub hidden: bool,
    pub velocity: Point2,
}

impl Particle {
    pub fn new(radius: f32, color: Srgba, position: Point2, velocity: Point2) -> Self {
        Self {
            radius,
            color,
            position,
            // Redundant flag, which in previous version was used to not render particles once they
            // left the screen, however since I wasnt able to detect when they went back onto the
            // screen, it became useless.
            hidden: false,
            velocity,
        }
    }

    pub fn update_pos<F>(&mut self, app: &App, arrow_function: F)
    where
        // x, y, t
        F: Fn(f32, f32, f32) -> Point2,
    {
        let dt = app.duration.since_prev_update.as_secs_f32() * TIME_SCALE;
        // Added a switch between acceleration / velocity field.
        if FIELD_MODE == FieldMode::Acceleration {
            // Velocity is cummulative and differs by the acceleration
            let acceleration = arrow_function(self.position.x, self.position.y, app.time);
            self.velocity += acceleration * dt;
            self.position += self.velocity * dt;
        } else {
            // Velocity DIRECTLY correlates to the arrow function
            self.velocity = arrow_function(self.position.x, self.position.y, app.time);
            self.position += self.velocity * dt;
        }
    }

    pub fn draw(&self, _app: &App, model: &Model, draw: &Draw) {
        // Just draw a small circle.
        let scale: f32 = DEFAULT_LINE_DISTANCE as f32 / model.grid.line_distance as f32;
        let radius = self.radius / scale;
        if !self.hidden {
            draw.ellipse()
                .xy(self.position)
                .radius(radius)
                .color(self.color);
        }
    }
}
