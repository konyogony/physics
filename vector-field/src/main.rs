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
const TIME_SCALE: f32 = 20.0;
const FIELD_MODE: FieldMode = FieldMode::Velocity;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FieldMode {
    Acceleration,
    Velocity,
}

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
    // +/= increases the scale
    if app.keys.down.contains(&Key::Equals) {
        model.grid.line_distance += 1;
        model.last_key_press_pts = Some(app.time)
    }

    // - increases the scale
    if app.keys.down.contains(&Key::Minus) {
        model.grid.line_distance -= 1;
        model.last_key_press_pts = Some(app.time)
    }

    // </, increases the color value, which affects the color distribution shown in the arrows.
    if app.keys.down.contains(&Key::Period) && model.color_value < 1000.0 {
        model.color_value += 1.0;
        model.last_key_press_pts = Some(app.time);
    }

    // >/. decreases the color value
    if app.keys.down.contains(&Key::Comma) && model.color_value > 1.0 {
        model.color_value -= 1.0;
        model.last_key_press_pts = Some(app.time);
    }

    // Create a new particle with no veclocity at the mouse location when left clicked.
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

    // Spawn a grid of particles with 'P', however this really degrades the performance
    if app.keys.down.contains(&Key::P) {
        let mut particles = model.grid.init_grid_particles(app);
        model.particles.append(&mut particles);
    }

    // Delete all the particles currently active.
    if app.keys.down.contains(&Key::C) {
        model.particles.clear();
    }

    for particle in &mut model.particles {
        particle.update_pos(app, arrow_function);
    }
}

// Arrow function responsible for the vectors themselves at each point in space
// Basically governs the velocity/accelearation of the particle at any particular point in space-time
// __only__ This function is usually AI generated.
fn arrow_function(x: f32, y: f32, t: f32) -> Point2 {
    // Adjust scale: lower = zoomed out (more detail), higher = zoomed in
    let p = Point2::new(x, y) * 0.005;
    let time = t * 0.4;

    // Layer 1: Large scale swirls
    let mut vx = (p.y + time).cos() + (p.y * 0.5 + time * 0.6).cos();
    let mut vy = (p.x - time).sin() + (p.x * 0.4 - time * 0.8).sin();

    // Layer 2: Medium scale turbulence (The "Curl")
    // We add a rotation based on the sine of the opposite axis
    let angle = (p.x * 1.2 + time).sin() * (p.y * 1.2 - time).cos();
    vx += angle.cos() * 0.5;
    vy += angle.sin() * 0.5;

    // Layer 3: Small scale jitter
    vx += (p.x * 3.0 + p.y * 2.0 + time * 2.0).sin() * 0.2;
    vy += (p.y * 3.0 - p.x * 2.0 - time * 2.0).cos() * 0.2;

    Point2::new(vx, vy)
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
