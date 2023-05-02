use bevy::prelude::*;
use bevy::render::render_resource::{
  BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BufferBindingType,
  CachedComputePipelineId, ComputePipelineDescriptor, PipelineCache, ShaderStages, ShaderType, StorageTextureAccess,
  TextureFormat, TextureViewDimension,
};
use bevy::render::renderer::RenderDevice;
use bevy::render::view::ViewUniform;
use std::borrow::Cow;

#[derive(Resource)]
pub struct RaytracingPipeline {
  pub view_bind_group_layout: BindGroupLayout,
  pub texture_bind_group_layout: BindGroupLayout,
  pub spheres_bind_group_layout: BindGroupLayout,
  pub light_dir_bind_group_layout: BindGroupLayout,
  pub pipeline: CachedComputePipelineId,
}

impl FromWorld for RaytracingPipeline {
  fn from_world(world: &mut World) -> Self {
    let view_bind_group_layout =
      world
        .resource::<RenderDevice>()
        .create_bind_group_layout(&BindGroupLayoutDescriptor {
          label: None,
          entries: &[BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::COMPUTE,
            ty: BindingType::Buffer {
              ty: BufferBindingType::Uniform,
              has_dynamic_offset: true,
              min_binding_size: Some(ViewUniform::min_size()),
            },
            count: None,
          }],
        });
    let texture_bind_group_layout =
      world
        .resource::<RenderDevice>()
        .create_bind_group_layout(&BindGroupLayoutDescriptor {
          label: None,
          entries: &[BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::COMPUTE,
            ty: BindingType::StorageTexture {
              access: StorageTextureAccess::WriteOnly,
              format: TextureFormat::Rgba8Unorm,
              view_dimension: TextureViewDimension::D2,
            },
            count: None,
          }],
        });
    let spheres_bind_group_layout =
      world
        .resource::<RenderDevice>()
        .create_bind_group_layout(&BindGroupLayoutDescriptor {
          label: None,
          entries: &[
            BindGroupLayoutEntry {
              binding: 0,
              visibility: ShaderStages::COMPUTE,
              ty: BindingType::Buffer {
                ty: BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: None,
              },
              count: None,
            },
            BindGroupLayoutEntry {
              binding: 1,
              visibility: ShaderStages::COMPUTE,
              ty: BindingType::Buffer {
                ty: BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
              },
              count: None,
            },
          ],
        });
    let light_dir_bind_group_layout =
      world
        .resource::<RenderDevice>()
        .create_bind_group_layout(&BindGroupLayoutDescriptor {
          label: None,
          entries: &[BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::COMPUTE,
            ty: BindingType::Buffer {
              ty: BufferBindingType::Uniform,
              has_dynamic_offset: false,
              min_binding_size: None,
            },
            count: None,
          }],
        });
    let pipeline_cache = world.resource::<PipelineCache>();
    let shader = world.resource::<AssetServer>().load("shaders/raytrace.wgsl");
    let pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
      label: None,
      layout: vec![
        view_bind_group_layout.clone(),
        texture_bind_group_layout.clone(),
        spheres_bind_group_layout.clone(),
        light_dir_bind_group_layout.clone(),
      ],
      push_constant_ranges: vec![],
      shader,
      shader_defs: vec![],
      entry_point: Cow::from("main"),
    });

    RaytracingPipeline {
      view_bind_group_layout,
      texture_bind_group_layout,
      spheres_bind_group_layout,
      light_dir_bind_group_layout,
      pipeline,
    }
  }
}
