use bevy::prelude::*;
use bevy::render::extract_resource::ExtractResource;
use bevy::render::render_resource::{BindGroup, Buffer};

#[derive(Resource, Clone, Deref, ExtractResource)]
pub struct RaytracingImage(pub Handle<Image>);

#[derive(Resource)]
pub struct RTCameraEntity(pub Entity);

#[derive(Resource, Clone, Deref, ExtractResource)]
pub struct PBRCameraEntity(pub Entity);

#[derive(Resource, Clone, ExtractResource)]
pub struct TextureIter(pub u32);

#[derive(Resource)]
pub struct RaytracingBindGroups {
  pub image: BindGroup,
  pub spheres: BindGroup,
  pub light_dir: BindGroup,
  pub seed: BindGroup,
}

#[derive(Component)]
pub struct ColorComponent {
  pub color: Color,
}

#[derive(Component)]
pub struct RoughnessComponent {
  pub roughness: f32,
}

#[derive(Component)]
pub struct SphereTag;
