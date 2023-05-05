use crate::render::raytracer::node::RayTraceNode;
use crate::render::raytracer::pipeline::RaytracingPipeline;
use crate::render::raytracer::systems::{extract_spheres, queue_bind_group};
use crate::render::raytracer::types::{PBRCameraEntity, RaytracingImage, TextureIter};
use crate::render::LightDir;
use bevy::prelude::*;
use bevy::render::extract_resource::ExtractResourcePlugin;
use bevy::render::render_graph::RenderGraph;
use bevy::render::{RenderApp, RenderSet};

pub mod node;
pub mod pipeline;
pub mod systems;
pub mod types;

pub const SIZE: [u32; 2] = [1024, 768];

pub struct RaytracePlugin;

impl Plugin for RaytracePlugin {
  fn build(&self, app: &mut App) {
    app.insert_resource(TextureIter(0));
    app.add_plugin(ExtractResourcePlugin::<TextureIter>::default());
    app.add_plugin(ExtractResourcePlugin::<RaytracingImage>::default());
    app.add_plugin(ExtractResourcePlugin::<PBRCameraEntity>::default());
    app.add_plugin(ExtractResourcePlugin::<LightDir>::default());
    let render_app = app.sub_app_mut(RenderApp);
    render_app
      .init_resource::<RaytracingPipeline>()
      .add_system(extract_spheres.in_schedule(ExtractSchedule))
      .add_system(queue_bind_group.in_set(RenderSet::Queue));

    let mut render_graph = render_app.world.resource_mut::<RenderGraph>();
    render_graph.add_node("raytrace", RayTraceNode { view: None });
    render_graph.add_node_edge("raytrace", bevy::render::main_graph::node::CAMERA_DRIVER);
  }
}
