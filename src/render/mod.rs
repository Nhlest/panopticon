use bevy::prelude::*;
use bevy::render::extract_resource::ExtractResource;
use bytemuck_derive::{Pod, Zeroable};

pub mod raytracer;

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
struct Sphere {
  coord: [f32; 3],
  radius: f32,
  material: u32,
  pad: [f32; 3],
}

#[derive(Copy, Clone, Pod, Zeroable, Component)]
#[repr(C)]
pub struct MaterialE {
  pub color: [f32; 4],
  pub emissive: [f32; 3],
  pub roughness: f32,
}

#[derive(Copy, Clone, Pod, Zeroable, Resource, ExtractResource, Default)]
#[repr(C)]
pub struct LightDir {
  pub dir: [f32; 3],
}

impl MaterialE {
  pub fn new(color: [f32; 4], roughness: f32, emissive: [f32; 3]) -> Self {
    Self {
      color,
      roughness,
      emissive,
    }
  }
}
