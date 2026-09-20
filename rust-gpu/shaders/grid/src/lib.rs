#![no_std]
#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]
use glam::{Vec2, Vec4, Vec4Swizzles};
use shaders_shared::{
    ARROW_HEAD_HEIGHT_PX, ARROW_HEAD_WIDTH_PX, ARROW_SCALE, ARROW_THICKNESS_PX, AXIS_COLOR,
    BG_COLOR, Field, GRID_COLOR, GRID_SPACING_PX, GRID_THICKNESS_PX, HIGHLIGHT_COLOR,
    HIGHLIGHT_SQUARES, MIN_ARROW_SCALE, ShaderConstants,
    sdf::SDF,
    util::{antialias, antialias_no_fwidth, hsv, map_range, smoothstep},
};
use spirv_std::num_traits::Float;
use spirv_std::spirv;

pub const MULTIPLIER: f32 = 1.5;

pub mod fragment;
pub mod vertex;
