use crate::render::raytracer::pipeline::RaytracingPipeline;
use crate::render::raytracer::types::{ColorComponent, RaytracingBindGroups, RaytracingImage, SphereTag};
use crate::render::{LightDir, Sphere};
use bevy::prelude::*;
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_resource::{
  BindGroupDescriptor, BindGroupEntry, BindingResource, BufferInitDescriptor, BufferUsages,
};
use bevy::render::renderer::RenderDevice;
use bevy::render::Extract;

pub fn queue_bind_group(
  mut commands: Commands,
  pipeline: Res<RaytracingPipeline>,
  gpu_images: Res<RenderAssets<Image>>,
  image: Res<RaytracingImage>,
  render_device: Res<RenderDevice>,
  light_dir: Res<LightDir>,
  query: Query<(&Transform, &ColorComponent), With<SphereTag>>,
) {
  let view = &gpu_images[&image.0];
  let image_bind_group = render_device.create_bind_group(&BindGroupDescriptor {
    label: None,
    layout: &pipeline.texture_bind_group_layout,
    entries: &[BindGroupEntry {
      binding: 0,
      resource: BindingResource::TextureView(&view.texture_view),
    }],
  });

  let mut spheres = vec![];
  for (i, j) in query.iter() {
    spheres.push(Sphere::new(
      j.color.as_rgba_f32(),
      [i.translation.x, i.translation.y, i.translation.z],
      1.0,
    ));
  }

  let sphere_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
    label: None,
    contents: bytemuck::cast_slice(spheres.as_slice()),
    usage: BufferUsages::COPY_SRC | BufferUsages::STORAGE,
  });

  let num_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
    label: None,
    contents: bytemuck::bytes_of(&(spheres.len() as i32)),
    usage: BufferUsages::COPY_SRC | BufferUsages::UNIFORM,
  });

  let spheres_bind_group = render_device.create_bind_group(&BindGroupDescriptor {
    label: None,
    layout: &pipeline.spheres_bind_group_layout,
    entries: &[
      BindGroupEntry {
        binding: 0,
        resource: BindingResource::Buffer(sphere_buffer.as_entire_buffer_binding()),
      },
      BindGroupEntry {
        binding: 1,
        resource: BindingResource::Buffer(num_buffer.as_entire_buffer_binding()),
      },
    ],
  });

  let light_dir_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
    label: None,
    contents: bytemuck::bytes_of(light_dir.as_ref()),
    usage: BufferUsages::COPY_SRC | BufferUsages::UNIFORM,
  });

  let light_dir_bind_group = render_device.create_bind_group(&BindGroupDescriptor {
    label: None,
    layout: &pipeline.light_dir_bind_group_layout,
    entries: &[BindGroupEntry {
      binding: 0,
      resource: BindingResource::Buffer(light_dir_buffer.as_entire_buffer_binding()),
    }],
  });

  commands.insert_resource(RaytracingBindGroups {
    image: image_bind_group,
    spheres: spheres_bind_group,
    light_dir: light_dir_bind_group,
  });
}

pub fn extract_spheres(
  mut commands: Commands,
  materials: Extract<Res<Assets<StandardMaterial>>>,
  spheres: Extract<Query<(&Transform, &Handle<StandardMaterial>), With<SphereTag>>>,
) {
  for (i, j) in spheres.iter() {
    commands.spawn((
      i.clone(),
      ColorComponent {
        color: materials.get(j).unwrap().base_color,
      },
      SphereTag,
    ));
  }
}
