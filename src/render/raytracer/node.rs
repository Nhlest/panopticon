use crate::render::raytracer::pipeline::RaytracingPipeline;
use crate::render::raytracer::types::{PBRCameraEntity, RaytracingBindGroups, TextureIter};
use crate::render::raytracer::SIZE;
use bevy::prelude::*;
use bevy::render::render_graph;
use bevy::render::render_resource::{BindGroupDescriptor, BindGroupEntry, ComputePassDescriptor, PipelineCache};
use bevy::render::renderer::RenderContext;
use bevy::render::view::{ExtractedView, ViewUniformOffset, ViewUniforms};

pub struct RayTraceNode {
  pub view: Option<u32>,
}

impl render_graph::Node for RayTraceNode {
  fn update(&mut self, world: &mut World) {
    let entity = world.resource::<PBRCameraEntity>().0;
    let view = world
      .query_filtered::<&ViewUniformOffset, With<ExtractedView>>()
      .get(world, entity)
      .ok()
      .map(|x| x.offset);
    self.view = view.or(self.view);
  }

  fn run(
    &self,
    _graph: &mut render_graph::RenderGraphContext,
    render_context: &mut RenderContext,
    world: &World,
  ) -> Result<(), render_graph::NodeRunError> {
    if world.resource::<TextureIter>().0 > 100 {
      return Ok(());
    }
    let bind_groups = &world.resource::<RaytracingBindGroups>();
    let pipeline_cache = world.resource::<PipelineCache>();
    let pipeline = world.resource::<RaytracingPipeline>();

    let view_uniforms_resource = world.resource::<ViewUniforms>();
    let view_uniforms = &view_uniforms_resource.uniforms;

    let entries = vec![BindGroupEntry {
      binding: 0,
      resource: view_uniforms.binding().unwrap(),
    }];
    let bind_group = render_context.render_device().create_bind_group(&BindGroupDescriptor {
      label: None,
      layout: &pipeline.view_bind_group_layout,
      entries: &entries,
    });

    let mut pass = render_context
      .command_encoder()
      .begin_compute_pass(&ComputePassDescriptor::default());

    pass.set_bind_group(0, &bind_group, &[self.view.unwrap()]);
    pass.set_bind_group(1, &bind_groups.image, &[]);
    pass.set_bind_group(2, &bind_groups.meshes, &[]);
    pass.set_bind_group(3, &bind_groups.materials, &[]);
    pass.set_bind_group(4, &bind_groups.light_dir, &[]);
    pass.set_bind_group(5, &bind_groups.seed, &[]);

    if let Some(pipeline) = pipeline_cache.get_compute_pipeline(pipeline.pipeline) {
      pass.set_pipeline(pipeline);
      pass.dispatch_workgroups(SIZE[0] / 32, SIZE[1] / 32, 1);
    }
    Ok(())
  }
}
