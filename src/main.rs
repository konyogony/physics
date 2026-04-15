use nannou::{
    App, Frame,
    event::{Key, Update},
    geom::pt2,
    prelude::Point2,
    window,
};

use crate::{grid::Grid, particle::Particle};

const LINE_DISTANCE: usize = 50;
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
    last_key_press_pts: Option<f32>,
    particles: Vec<Particle>,
}

// Setup the window & particles
fn model_fn(app: &App) -> Model {
    let _window = app.new_window().view(view_fn).build().unwrap();

    let particle1 = Particle::new(5.0, nannou::color::srgba(1.0, 0.3, 0.1, 0.6), pt2(0.0, 1.0));
    Model {
        _window,
        color_value: 200.0,
        time_scale: 1.0,
        last_key_press_pts: None,
        particles: vec![particle1],
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
        let particle = Particle::new(5.0, nannou::color::srgba(1.0, 0.3, 0.1, 0.6), mouse_pos);
        model.particles.push(particle);
    }

    for particle in &mut model.particles {
        particle.update_pos(app, arrow_function);
    }
}

// Arrow function responsible for the vectors themselves at each point in space
fn arrow_function(x: f32, y: f32) -> Point2 {
    let x_output = y;
    let y_output = x;
    pt2(x_output, y_output)
}

// Responsible solely for rendering all the parts
fn view_fn(app: &App, model: &Model, frame: Frame) {
    let color_axis = nannou::color::srgba(1.0, 1.0, 1.0, 0.8);
    let color_grid = nannou::color::srgba(0.5, 0.5, 0.5, 0.15);
    let color_highlight = nannou::color::srgba(0.0, 1.0, 1.0, 0.4);

    // Get window information & draw
    let draw = app.draw();

    let grid = Grid::new(
        LINE_DISTANCE,
        color_axis,
        color_grid,
        color_highlight,
        HIHGLIGHT_DISTANCE,
        MIN_ARROW_SCLAE,
        ARROW_SCALING,
    );
    grid.draw_grid(app, model, &draw);
    grid.draw_vectors(app, model, &draw, arrow_function);

    let alpha = if let Some(last_press) = model.last_key_press_pts
        && (app.time - last_press) < 3.0
    {
        1.0
    } else {
        0.0
    };

    draw.text(
        format!(
            "Color Value: {}, Time Scale: {}",
            model.color_value, model.time_scale
        )
        .as_str(),
    )
    .xy(pt2(
        app.window_rect().right() - 100.0,
        app.window_rect().top() - 10.0,
    ))
    .rgba(1.0, 1.0, 1.0, alpha);

    for particle in &model.particles {
        particle.draw(app, model, &draw);
    }

    draw.to_frame(app, &frame).unwrap();
}
