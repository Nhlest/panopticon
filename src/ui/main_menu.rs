use crate::app::AppState;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_egui::egui::Layout;
use bevy_egui::*;

pub fn main_menu(
  mut egui: EguiContexts,
  mut next_state: ResMut<NextState<AppState>>,
  mut exit_event: EventWriter<AppExit>,
  windows: Query<&Window>,
) {
  let x = (windows.single().width() - 300.0) / 2.0;
  egui::Window::new("Main Menu")
    .fixed_size([300.0, 600.0])
    .fixed_pos([x, 50.0])
    .show(egui.ctx_mut(), |ui| {
      ui.with_layout(Layout::top_down(egui::Align::Center), |ui| {
        if ui.button("Render").clicked() {
          next_state.set(AppState::Render);
        }
        if ui.button("Exit").clicked() {
          exit_event.send(AppExit);
        }
      });
      ui.allocate_space(ui.available_size());
    });
}
