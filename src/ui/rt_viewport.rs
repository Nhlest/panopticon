use crate::render::raytracer::types::RaytracingImage;
use bevy::prelude::*;
use bevy_editor_pls::editor::EditorInternalState;
use bevy_editor_pls::editor_window::{EditorWindow, EditorWindowContext};
use bevy_editor_pls::egui::Ui;
use bevy_editor_pls::egui_dock::NodeIndex;
use bevy_egui::{egui, EguiUserTextures};

pub struct RTViewportWindow;

impl EditorWindow for RTViewportWindow {
  type State = ();
  const NAME: &'static str = "RTX Viewport";

  fn ui(world: &mut World, _cx: EditorWindowContext, ui: &mut egui::Ui) {
    let viewport_image = &world.resource::<RaytracingImage>().0;
    let egui = world.resource::<EguiUserTextures>();
    let id = egui.image_id(viewport_image);
    ui.image(id.unwrap(), ui.available_size());
  }
}
