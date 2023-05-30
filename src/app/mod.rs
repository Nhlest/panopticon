use crate::render::raytracer::types::{PBRCameraEntity, RTCameraEntity, RaytracingImage, TextureIter};
use crate::render::raytracer::SIZE;
use crate::render::LightDir;
use bevy::prelude::*;
use bevy::render::camera::CameraOutputMode;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages};
use bevy_egui::EguiContexts;

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum AppState {
  #[default]
  MainMenu,
  Render,
}

pub fn setup(
  mut commands: Commands,
  mut images: ResMut<Assets<Image>>,
  mut egui_contexts: EguiContexts,
  asset_server: Res<AssetServer>,
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
  let cam_2d = commands
    .spawn(Camera2dBundle {
      camera: Camera {
        order: 0,
        output_mode: CameraOutputMode::default(),
        ..default()
      },
      transform: Transform::from_xyz(0.0, 0.0, 2.0),
      ..default()
    })
    .id();
  let cam_3d = commands
    .spawn(Camera3dBundle {
      camera: Camera {
        order: 1,
        output_mode: CameraOutputMode::Skip,
        ..default()
      },
      transform: Transform::from_xyz(0.0, 1.2, 4.0),
      ..default()
    })
    .id();

  commands.insert_resource(RTCameraEntity(cam_2d));
  commands.insert_resource(PBRCameraEntity(cam_3d));

  egui_contexts.add_image(image.clone());

  commands.insert_resource(RaytracingImage(image));

  let scene = asset_server.load("boxge.glb#Scene0");

  commands.spawn(SceneBundle {
    scene: scene,
    transform: Transform::from_xyz(0.0, 0.0, 0.0),
    ..Default::default()
  });

  // let mesh = Cube {
  //   size: 1.0,
  // };
  // let m_id = meshes.add(mesh.into());
  //
  // let material = StandardMaterial {
  //   base_color: Color::Rgba {
  //     red: 0.0,
  //     green: 0.0,
  //     blue: 0.5,
  //     alpha: 1.0,
  //   },
  //   emissive: Color::BLACK,
  //   metallic: 0.3,
  //   perceptual_roughness: 0.3,
  //   ..default()
  // };
  //
  // let material_2 = StandardMaterial {
  //   base_color: Color::Rgba {
  //     red: 0.5,
  //     green: 1.0,
  //     blue: 1.0,
  //     alpha: 1.0,
  //   },
  //   metallic: 0.3,
  //   perceptual_roughness: 0.3,
  //   emissive: Color::BLACK,
  //   ..default()
  // };
  //
  // let material_3 = StandardMaterial {
  //   base_color: Color::BLACK,
  //   emissive: Color::WHITE,
  //   ..default()
  // };
  //
  // let mat_id = materials.add(material);
  // let mat_id_2 = materials.add(material_2);
  // let mat_id_3 = materials.add(material_3);
  //
  // commands.spawn((
  //   Raytraced,
  //   PbrBundle {
  //     mesh: m_id.clone(),
  //     material: mat_id.clone(),
  //     ..default()
  //   },
  // ));
  // commands.spawn((
  //   Raytraced,
  //   PbrBundle {
  //     mesh: m_id.clone(),
  //     material: mat_id_2,
  //     transform: Transform::from_xyz(0.0, -100.0, 0.0).with_scale(Vec3::splat(99.0)),
  //     ..default()
  //   },
  // ));
  // commands.spawn((
  //   Raytraced,
  //   PbrBundle {
  //     mesh: m_id,
  //     material: mat_id_3,
  //     transform: Transform::from_xyz(2.0, 2.0, -1.0).with_scale(Vec3::splat(2.0)),
  //     ..default()
  //   },
  // ));

  // commands.spawn(DirectionalLightBundle {
  //   directional_light: DirectionalLight {
  //     color: Color::WHITE,
  //     illuminance: 10000.0,
  //     shadows_enabled: true,
  //     ..default()
  //   },
  //   transform: Transform::default().looking_at(Vec3::new(1.0, -1.0, -1.0), Vec3::new(0.0, 1.0, 0.0)),
  //   ..default()
  // });
}

pub fn rotate_light(_time: Res<Time>, mut light_dir: ResMut<LightDir>) {
  light_dir.dir[0] = 0.2;
  light_dir.dir[1] = -1.0;
  light_dir.dir[2] = -0.2;
}

pub fn reset_iter(
  q: Query<Entity, Changed<Transform>>,
  m: EventReader<AssetEvent<StandardMaterial>>,
  mut iter: ResMut<TextureIter>,
) {
  iter.0 += 1;
  if !q.is_empty() || !m.is_empty() {
    iter.0 = 0;
  }
}
