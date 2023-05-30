use crate::app::{reset_iter, rotate_light, AppState};
use crate::render::raytracer::types::{ShaderMaterial, ShaderMesh, ShaderVertex};
use crate::render::raytracer::RaytracePlugin;
use crate::render::LightDir;
use crate::ui::main_menu::main_menu;
use crate::ui::rt_viewport::RTViewportWindow;
use app::setup;
use bevy::prelude::*;
use bevy::window::{ExitCondition, WindowResolution};
use bevy_editor_pls::{AddEditorWindow, EditorPlugin};

pub mod app;
pub mod render;
pub mod ui;
pub mod util;

fn main() {
  assert_eq!(std::mem::size_of::<ShaderMaterial>() % 16, 0);
  assert_eq!(std::mem::size_of::<ShaderMesh>() % 16, 0);
  assert_eq!(std::mem::size_of::<ShaderVertex>() % 16, 0);
  App::new()
    .add_plugins(
      DefaultPlugins
        .set(WindowPlugin {
          primary_window: Some(Window {
            resolution: WindowResolution::new(1024.0, 768.0),
            title: "Lode Runner".to_string(),
            resizable: false,
            ..default()
          }),
          exit_condition: ExitCondition::OnAllClosed,
          close_when_requested: true,
        })
        .set(ImagePlugin::default_nearest()),
    )
    .add_plugin(EditorPlugin::default())
    .add_editor_window::<RTViewportWindow>()
    .init_resource::<LightDir>()
    .add_state::<AppState>()
    .add_startup_system(setup)
    // .add_plugin(EguiPlugin)
    .add_plugin(RaytracePlugin)
    .add_system(main_menu.in_set(OnUpdate(AppState::MainMenu)))
    // .add_system(debug_ui.in_set(OnUpdate(AppState::Render)))
    .add_system(rotate_light.in_set(OnUpdate(AppState::Render)))
    .add_system(reset_iter.in_set(OnUpdate(AppState::Render)))
    .insert_resource(ClearColor(Color::BLACK))
    .run();
}
