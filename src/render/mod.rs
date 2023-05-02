use bevy::prelude::*;
use bevy::render::extract_resource::ExtractResource;
use bytemuck_derive::{Pod, Zeroable};

pub mod raytracer;

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
struct Sphere {
  color: [f32; 4],
  coord: [f32; 3],
  radius: f32,
}

#[derive(Copy, Clone, Pod, Zeroable, Resource, ExtractResource, Default)]
#[repr(C)]
pub struct LightDir {
  pub dir: [f32; 3],
}

impl Sphere {
  pub fn new(color: [f32; 4], coord: [f32; 3], radius: f32) -> Self {
    Self { color, coord, radius }
  }
}
