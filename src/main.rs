use cgmath::{Angle, Deg};
use nannou::{
    App, Frame,
    event::{Key, Update},
    geom::pt2,
    prelude::Point2,
    window,
};

use crate::{grid::Grid, particle::Particle};

const DEFAULT_LINE_DISTANCE: usize = 30;
const DEFAULT_RADIUS: f32 = 5.0;
const HIHGLIGHT_DISTANCE: usize = 3;
const ARROW_SCALING: f32 = 30.0;
const MIN_ARROW_SCLAE: f32 = 0.7;
const TIME_SCALE: f32 = 1.0;

pub mod grid;
pub mod particle;
pub mod utils;

fn main() {
    nannou::app(model_fn).update(update_fn).run();
}

pub struct Model {
    _window: window::Id,
    color_value: f32,
    time_scale: f32,
    grid: Grid,
    last_key_press_pts: Option<f32>,
    particles: Vec<Particle>,
}

// Setup the window & particles
fn model_fn(app: &App) -> Model {
    let _window = app.new_window().view(view_fn).build().unwrap();

    let color_axis = nannou::color::srgba(1.0, 1.0, 1.0, 0.8);
    let color_grid = nannou::color::srgba(0.5, 0.5, 0.5, 0.15);
    let color_highlight = nannou::color::srgba(0.0, 1.0, 1.0, 0.4);

    let grid = Grid::new(
        DEFAULT_LINE_DISTANCE,
        color_axis,
        color_grid,
        color_highlight,
        HIHGLIGHT_DISTANCE,
        MIN_ARROW_SCLAE,
        ARROW_SCALING,
    );

    Model {
        _window,
        color_value: 200.0,
        time_scale: 1.0,
        last_key_press_pts: None,
        grid,
        particles: Vec::new(),
    }
}

fn update_fn(app: &App, model: &mut Model, _update: Update) {
    // if app.keys.down.contains(&Key::Equals) {
    //     model.time_scale += 0.1;
    //     model.last_key_press_pts = Some(app.time);
    // }
    // if app.keys.down.contains(&Key::Minus) {
    //     model.time_scale -= 0.1;
    //     model.last_key_press_pts = Some(app.time);
    // }

    if app.keys.down.contains(&Key::Equals) {
        model.grid.line_distance += 1;
        model.last_key_press_pts = Some(app.time)
    }

    if app.keys.down.contains(&Key::Minus) {
        model.grid.line_distance -= 1;
        model.last_key_press_pts = Some(app.time)
    }

    if app.keys.down.contains(&Key::Period) && model.color_value < 1000.0 {
        model.color_value += 1.0;
        model.last_key_press_pts = Some(app.time);
    }
    if app.keys.down.contains(&Key::Comma) && model.color_value > 1.0 {
        model.color_value -= 1.0;
        model.last_key_press_pts = Some(app.time);
    }

    if app.mouse.buttons.left().is_down() {
        let mouse_pos = pt2(app.mouse.x, app.mouse.y);
        let particle = Particle::new(
            DEFAULT_RADIUS,
            nannou::color::srgba(1.0, 0.3, 0.1, 0.6),
            mouse_pos,
            Point2::ZERO,
        );
        model.particles.push(particle);
    }

    if app.keys.down.contains(&Key::P) {
        let mut particles = model.grid.init_grid_particles(app);
        model.particles.append(&mut particles);
    }

    if app.keys.down.contains(&Key::C) {
        model.particles.clear();
    }

    for particle in &mut model.particles {
        particle.update_pos(app, arrow_function);
    }
}

// Arrow function responsible for the vectors themselves at each point in space
// Basically governs the velocity of the particle at any particular point in space-time
fn arrow_function(x: f32, y: f32, t: f32) -> Point2 {
    let r = (x * x + y * y).sqrt() + 0.001;

    let angle = t * 0.5;

    let x_output = -y / r + 0.3 * angle.cos() * x;
    let y_output = x / r + 0.3 * angle.sin() * y;

    pt2(x_output, y_output)
}

// Responsible solely for rendering all the parts
fn view_fn(app: &App, model: &Model, frame: Frame) {
    // Get window information & draw
    let draw = app.draw();

    model.grid.draw_grid(app, model, &draw);
    model.grid.draw_vectors(app, model, &draw, arrow_function);

    let alpha = if let Some(last_press) = model.last_key_press_pts
        && (app.time - last_press) < 3.0
    {
        1.0
    } else {
        0.0
    };

    draw.text(
        format!(
            "FPS: {:.2}, Color Value: {}, Time Scale: {}",
            app.fps(),
            model.color_value,
            model.time_scale
        )
        .as_str(),
    )
    .xy(pt2(
        app.window_rect().right() - 250.0,
        app.window_rect().top() - 10.0,
    ))
    .rgba(1.0, 1.0, 1.0, alpha);

    for particle in &model.particles {
        particle.draw(app, model, &draw);
    }

    draw.to_frame(app, &frame).unwrap();
}
