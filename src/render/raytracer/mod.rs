use bevy::prelude::*;
use bevy::render::extract_resource::{ExtractResourcePlugin};
use bevy::render::{RenderApp, RenderSet};
use bevy::render::render_graph::RenderGraph;
use crate::render::LightDir;
use crate::render::raytracer::node::RayTraceNode;
use crate::render::raytracer::pipeline::RaytracingPipeline;
use crate::render::raytracer::systems::{extract_spheres, queue_bind_group};
use crate::render::raytracer::types::{PBRCameraEntity, RaytracingImage};

pub mod node;
pub mod pipeline;
pub mod types;
pub mod systems;

pub const SIZE : [u32; 2] = [1024, 768];

pub struct RaytracePlugin;

impl Plugin for RaytracePlugin {
  fn build(&self, app: &mut App) {
    app.add_plugin(ExtractResourcePlugin::<RaytracingImage>::default());
    app.add_plugin(ExtractResourcePlugin::<PBRCameraEntity>::default());
    app.add_plugin(ExtractResourcePlugin::<LightDir>::default());
    let render_app = app.sub_app_mut(RenderApp);
    let query = QueryState::new(&mut render_app.world);
    render_app
      .init_resource::<RaytracingPipeline>()
      .add_system(extract_spheres.in_schedule(ExtractSchedule))
      .add_system(queue_bind_group.in_set(RenderSet::Queue));

    let mut render_graph = render_app.world.resource_mut::<RenderGraph>();
    render_graph.add_node("raytrace", RayTraceNode { query });
    render_graph.add_node_edge(
      "raytrace",
      bevy::render::main_graph::node::CAMERA_DRIVER,
    );
  }
}
