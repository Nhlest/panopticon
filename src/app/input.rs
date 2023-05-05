use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use crate::render::raytracer::types::PBRCameraEntity;

pub fn camera(
  mut commands: Commands,
  camera: Res<PBRCameraEntity>,
  mut transform: Query<&mut Transform>,
  mut mouse_input: EventReader<MouseMotion>,
  key: Res<Input<KeyCode>>,
  mouse_button: Res<Input<MouseButton>>,
  time: Res<Time>
) {
  // if !mouse_button.pressed(MouseButton::Left) { return; }
  // let mut transform = transform.get_mut(camera.0).unwrap();
  // for m in mouse_input.iter() {
  //   transform.rotate_y(m.delta.x * 0.01);
  //   transform.rotate_x(m.delta.y * 0.01);
  // }
  // let mut d = Vec3::ZERO;
  // if key.pressed(KeyCode::A) {
  //   d += transform.left() * time.delta_seconds();
  // }
  // if key.pressed(KeyCode::D) {
  //   d += transform.right() * time.delta_seconds();
  // }
  // if key.pressed(KeyCode::W) {
  //   d += transform.forward() * time.delta_seconds();
  // }
  // if key.pressed(KeyCode::S) {
  //   d += transform.back() * time.delta_seconds();
  // }
  // transform.translation += d;
}