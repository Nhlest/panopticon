use crate::app::AppState;
use crate::ui::main_menu::main_menu;
use bevy::prelude::*;
use bevy::window::{ExitCondition, WindowResolution};
use bevy_editor_pls::EditorPlugin;
use crate::render::{LightDir, setup};
use crate::render::raytracer::RaytracePlugin;
use crate::ui::debug_ui::debug_ui;

pub mod app;
pub mod ui;
pub mod util;
pub mod render;

fn main() {
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
    .init_resource::<LightDir>()
    .add_state::<AppState>()
    .add_startup_system(setup)
    // .add_plugin(EguiPlugin)
    .add_plugin(RaytracePlugin)
    .add_system(main_menu.in_set(OnUpdate(AppState::MainMenu)))
    .add_system(debug_ui.in_set(OnUpdate(AppState::Render)))
    .add_system(rotate_light.in_set(OnUpdate(AppState::Render)))
    .insert_resource(ClearColor(Color::BLACK))
    .run();
}

fn rotate_light(
  time: Res<Time>,
  mut light_dir: ResMut<LightDir>
) {
  let t = time.elapsed().as_secs_f32();
  light_dir.dir[0] = t.sin();
  light_dir.dir[2] = 0.2;
  light_dir.dir[1] = t.cos();
}