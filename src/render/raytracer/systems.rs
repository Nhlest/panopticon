use crate::render::raytracer::pipeline::RaytracingPipeline;
use crate::render::raytracer::types::{ColorComponent, RaytracingBindGroups, RaytracingImage, RoughnessComponent, SphereTag, TextureIter};
use crate::render::{LightDir, MaterialE, Sphere};
use bevy::prelude::*;
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_resource::{
  BindGroupDescriptor, BindGroupEntry, BindingResource, BufferInitDescriptor, BufferUsages,
};
use bevy::render::renderer::RenderDevice;
use bevy::render::Extract;
use bevy::utils::HashMap;
use rand::Rng;

pub fn queue_bind_group(
  mut commands: Commands,
  pipeline: Res<RaytracingPipeline>,
  gpu_images: Res<RenderAssets<Image>>,
  image: Res<RaytracingImage>,
  render_device: Res<RenderDevice>,
  light_dir: Res<LightDir>,
  spheres_query: Query<(&Transform, &Handle<StandardMaterial>), With<SphereTag>>,
  materials_query: Query<(&Handle<StandardMaterial>, &MaterialE)>,
  texture_iter: Res<TextureIter>
) {
  let view = &gpu_images[&image.0];

  let texture_iter_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
    label: None,
    contents: bytemuck::cast_slice(&[texture_iter.0]),
    usage: BufferUsages::COPY_SRC | BufferUsages::UNIFORM,
  });

  let image_bind_group = render_device.create_bind_group(&BindGroupDescriptor {
    label: None,
    layout: &pipeline.texture_bind_group_layout,
    entries: &[
      BindGroupEntry {
        binding: 0,
        resource: BindingResource::TextureView(&view.texture_view),
      },
      BindGroupEntry {
        binding: 1,
        resource: BindingResource::Buffer(texture_iter_buffer.as_entire_buffer_binding()),
      },
    ],
  });

  let mut materials = vec![];
  let mut materials_map = HashMap::new();
  let mut id = 0;
  for (h, m) in materials_query.iter() {
    materials.push(m.clone());
    materials_map.insert(h, id);
    id+=1;
  }

  let mut spheres = vec![];
  for (i, h) in spheres_query.iter() {
    spheres.push(Sphere::new(
      [i.translation.x, i.translation.y, i.translation.z],
      i.scale.x,
      materials_map[h]
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

  let materials_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
    label: None,
    contents: bytemuck::cast_slice(materials.as_slice()),
    usage: BufferUsages::COPY_SRC | BufferUsages::STORAGE,
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
      BindGroupEntry {
        binding: 2,
        resource: BindingResource::Buffer(materials_buffer.as_entire_buffer_binding()),
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

  let seed_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
    label: None,
    contents: bytemuck::bytes_of(&rand::thread_rng().gen::<[f32; 2]>()),
    usage: BufferUsages::COPY_SRC | BufferUsages::UNIFORM,
  });

  let seed_bind_group = render_device.create_bind_group(&BindGroupDescriptor {
    label: None,
    layout: &pipeline.seed_bind_group_layout,
    entries: &[BindGroupEntry {
      binding: 0,
      resource: BindingResource::Buffer(seed_buffer.as_entire_buffer_binding()),
    }],
  });

  commands.insert_resource(RaytracingBindGroups {
    image: image_bind_group,
    spheres: spheres_bind_group,
    light_dir: light_dir_bind_group,
    seed: seed_bind_group
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
      j.clone(),
      SphereTag,
    ));
  }
  for (h, m) in materials.iter() {
    commands.spawn((materials.get_handle(h), MaterialE::new(m.base_color.as_rgba_f32(), m.perceptual_roughness, [m.emissive.r(), m.emissive.g(), m.emissive.b()])));
  }
}
