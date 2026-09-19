#![no_std]
#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]
use core::f32::consts::PI;
use glam::{UVec3, Vec2, Vec3, Vec4};
use shaders_shared::{Charge, Field, Particle, ShaderConstants};
use spirv_std::num_traits::Float;
use spirv_std::spirv;

pub mod compute;
pub mod fragment;
pub mod vertex;
