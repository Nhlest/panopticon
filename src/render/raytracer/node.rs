use bevy::prelude::*;
use bevy::render::{render_graph};
use bevy::render::render_resource::{BindGroupDescriptor, BindGroupEntry, ComputePassDescriptor, PipelineCache};
use bevy::render::renderer::RenderContext;
use bevy::render::view::{ExtractedView, ViewTarget, ViewUniformOffset, ViewUniforms};
use crate::render::raytracer::pipeline::RaytracingPipeline;
use crate::render::raytracer::SIZE;
use crate::render::raytracer::types::{PBRCameraEntity, RaytracingBindGroups};

pub struct RayTraceNode {
  pub query: QueryState<
    (
      &'static ViewUniformOffset,
      &'static ViewTarget,
    ),
    With<ExtractedView>,
  >
}

impl render_graph::Node for RayTraceNode {
  fn update(&mut self, world: &mut World) {
    self.query.update_archetypes(world);
  }

  fn run(
    &self,
    _graph: &mut render_graph::RenderGraphContext,
    render_context: &mut RenderContext,
    world: &World,
  ) -> Result<(), render_graph::NodeRunError> {
    let bind_groups = &world.resource::<RaytracingBindGroups>();
    let pipeline_cache = world.resource::<PipelineCache>();
    let pipeline = world.resource::<RaytracingPipeline>();

    let view_uniforms_resource = world.resource::<ViewUniforms>();
    let view_uniforms = &view_uniforms_resource.uniforms;

    let entries = vec![
      BindGroupEntry {
        binding: 0,
        resource: view_uniforms.binding().unwrap(),
      },
    ];
    let bind_group =
      render_context
        .render_device()
        .create_bind_group(&BindGroupDescriptor {
          label: None,
          layout: &pipeline.view_bind_group_layout,
          entries: &entries,
        });

    let mut pass = render_context
      .command_encoder()
      .begin_compute_pass(&ComputePassDescriptor::default());

    let (a, _) = self.query.get_manual(world, world.resource::<PBRCameraEntity>().0).unwrap();

    pass.set_bind_group(0, &bind_group, &[a.offset]);
    pass.set_bind_group(1, &bind_groups.image, &[]);
    pass.set_bind_group(2, &bind_groups.spheres, &[]);
    pass.set_bind_group(3, &bind_groups.light_dir, &[]);

    if let Some(pipeline) = pipeline_cache.get_compute_pipeline(pipeline.pipeline) {
      pass.set_pipeline(pipeline);
      pass.dispatch_workgroups(SIZE[0] / 16, SIZE[1] / 16, 1);
    }
    Ok(())
  }
}
