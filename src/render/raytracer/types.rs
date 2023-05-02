use bevy::prelude::*;
use bevy::render::extract_resource::ExtractResource;
use bevy::render::render_resource::BindGroup;

#[derive(Resource, Clone, Deref, ExtractResource)]
pub struct RaytracingImage(pub Handle<Image>);

#[derive(Resource)]
pub struct RTCameraEntity(pub Entity);

#[derive(Resource, Clone, Deref, ExtractResource)]
pub struct PBRCameraEntity(pub Entity);

#[derive(Resource)]
pub struct RaytracingBindGroups {
  pub image: BindGroup,
  pub spheres: BindGroup,
  pub light_dir: BindGroup,
}

#[derive(Component)]
pub struct ColorComponent {
  pub color: Color,
}

#[derive(Component)]
pub struct SphereTag;
