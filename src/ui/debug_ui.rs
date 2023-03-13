use bevy::prelude::*;
use bevy::render::camera::CameraOutputMode;
use bevy_egui::egui::{Layout, Widget};
use bevy_egui::*;
use crate::render::raytracer::types::{PBRCameraEntity, RTCameraEntity};

pub fn debug_ui(
  mut egui: EguiContexts,
  mut transforms: Query<&mut Transform>,
  mut camera: Query<&mut Camera>,
  pbr_camera_entity: Res<PBRCameraEntity>,
  rt_camera_entity: Res<RTCameraEntity>,
  mut sprite: Query<&mut Visibility, With<Sprite>>,
) {
  let mut transform = transforms.get_mut(pbr_camera_entity.0).unwrap();
  let mut visibility = sprite.single_mut();
  egui::Window::new("Debug UI")
    .fixed_size([300.0, 600.0])
    .show(egui.ctx_mut(), |ui| {
      ui.with_layout(Layout::top_down(egui::Align::Center), |ui| {
        egui::Slider::new(&mut transform.translation.x, -30.0..=30.0).ui(ui);
        egui::Slider::new(&mut transform.translation.y, -30.0..=30.0).ui(ui);
        egui::Slider::new(&mut transform.translation.z, -30.0..=30.0).ui(ui);
        let mut visible = *visibility != Visibility::Hidden;
        let t = if visible { "RTX ON" } else { "RTX OFF" };
        ui.checkbox(&mut visible, t);
        *visibility = if visible { Visibility::Visible } else { Visibility::Hidden };
        if visible {
          camera.get_mut(rt_camera_entity.0).unwrap().output_mode = CameraOutputMode::default();
          camera.get_mut(pbr_camera_entity.0).unwrap().output_mode = CameraOutputMode::Skip;
        } else {
          camera.get_mut(pbr_camera_entity.0).unwrap().output_mode = CameraOutputMode::default();
          camera.get_mut(rt_camera_entity.0).unwrap().output_mode = CameraOutputMode::Skip;
        }
      });
      ui.allocate_space(ui.available_size());
    });
}
