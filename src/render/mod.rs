use bevy::prelude::*;
use bevy::prelude::shape::UVSphere;
use bevy::render::extract_resource::{ExtractResource};
use bevy::render::camera::CameraOutputMode;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages};
use bytemuck_derive::{Pod, Zeroable};
use crate::render::raytracer::SIZE;
use crate::render::raytracer::types::{PBRCameraEntity, RaytracingImage, RTCameraEntity, SphereTag};

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
    Self {
      color,
      coord,
      radius,
    }
  }
}

pub fn setup(
  mut commands: Commands,
  mut images: ResMut<Assets<Image>>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  let mut image = Image::new_fill(
    Extent3d {
      width: SIZE[0],
      height: SIZE[1],
      depth_or_array_layers: 1,
    },
    TextureDimension::D2,
    &[255, 0, 0, 255],
    TextureFormat::Rgba8Unorm,
  );
  image.texture_descriptor.usage =
    TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;
  let image = images.add(image);

  commands.spawn(SpriteBundle {
    sprite: Sprite {
      custom_size: Some(Vec2::new(1024 as f32, 768 as f32)),
      ..default()
    },
    texture: image.clone(),
    ..default()
  });
  let cam_2d = commands.spawn(Camera2dBundle {
    camera: Camera {
      order: 0,
      output_mode: CameraOutputMode::Skip,
      ..default()
    },
    transform: Transform::from_xyz(0.0, 0.0, 2.0),
    .. default()
  }).id();
  let cam_3d = commands.spawn(Camera3dBundle {
    camera: Camera {
      order: 1,
      output_mode: CameraOutputMode::Skip,
      ..default()
    },
    transform: Transform::from_xyz(0.0, 0.0, 2.0),
    .. default()
  }).id();

  commands.insert_resource(RTCameraEntity(cam_2d));
  commands.insert_resource(PBRCameraEntity(cam_3d));

  commands.insert_resource(RaytracingImage(image));

  let mesh = UVSphere {
    radius: 0.5,
    sectors: 16,
    stacks: 16,
  };
  let m_id = meshes.add(mesh.into());

  let material = StandardMaterial {
    base_color: Color::BEIGE,
    metallic: 0.3,
    perceptual_roughness: 0.3,
    .. default()
  };

  let material_2 = StandardMaterial {
    base_color: Color::RED,
    metallic: 0.3,
    perceptual_roughness: 0.3,
    .. default()
  };

  let mat_id = materials.add(material);
  let mat_id_2 = materials.add(material_2);

  commands.spawn((SphereTag, PbrBundle {
    mesh: m_id.clone(),
    material: mat_id.clone(),
    .. default()
  }));
  commands.spawn((SphereTag, PbrBundle {
    mesh: m_id,
    material: mat_id_2,
    transform: Transform::from_xyz(1.0, -1.0, 0.2),
    .. default()
  }));

  commands.spawn(DirectionalLightBundle {
    directional_light: DirectionalLight {
      color: Color::WHITE,
      illuminance: 10000.0,
      shadows_enabled: true,
      .. default()
    },
    transform: Transform::default().looking_at(Vec3::new(1.0, -1.0, -1.0), Vec3::new(0.0, 1.0, 0.0)),
    ..default()
  });
}