use crate::render::raytracer::pipeline::RaytracingPipeline;
use crate::render::raytracer::types::{
  ExtractedMesh, MaterialBuffer, MaterialStorage, MeshBuffer, MeshStorage, RaytracingBindGroups, RaytracingImage,
  ShaderMaterial, ShaderMesh, ShaderVertex, TextureIter, VertexBuffer, VertexStorage,
};
use crate::render::LightDir;
use bevy::prelude::*;
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_resource::{
  BindGroupDescriptor, BindGroupEntry, BindingResource, BufferInitDescriptor, BufferUsages,
};
use bevy::render::renderer::RenderDevice;
use bevy::render::Extract;
use bevy::utils::HashMap;
use bevy_editor_pls::prelude::NotInScene;
use itertools::Itertools;
use rand::Rng;

pub fn queue_bind_group(
  mut commands: Commands,
  pipeline: Res<RaytracingPipeline>,
  gpu_images: Res<RenderAssets<Image>>,
  image: Res<RaytracingImage>,
  render_device: Res<RenderDevice>,
  light_dir: Res<LightDir>,
  texture_iter: Res<TextureIter>,
  mesh_storage: Res<MeshStorage>,
  vertex_buffer: Res<VertexBuffer>,
  mesh_buffer: Res<MeshBuffer>,
  material_buffer: Res<MaterialBuffer>,
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

  let num_of_meshes_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
    label: None,
    contents: (mesh_storage.meshes.len() as u32).to_le_bytes().as_slice(),
    usage: BufferUsages::COPY_SRC | BufferUsages::UNIFORM,
  });

  let materials_bind_group = render_device.create_bind_group(&BindGroupDescriptor {
    label: None,
    layout: &pipeline.materials_bind_group_layout,
    entries: &[BindGroupEntry {
      binding: 0,
      resource: BindingResource::Buffer(material_buffer.buffer.as_entire_buffer_binding()),
    }],
  });

  let meshes_bind_group = render_device.create_bind_group(&BindGroupDescriptor {
    label: None,
    layout: &pipeline.meshes_bind_group_layout,
    entries: &[
      BindGroupEntry {
        binding: 0,
        resource: BindingResource::Buffer(vertex_buffer.vertex_buffer.as_entire_buffer_binding()),
      },
      BindGroupEntry {
        binding: 1,
        resource: BindingResource::Buffer(vertex_buffer.index_buffer.as_entire_buffer_binding()),
      },
      BindGroupEntry {
        binding: 2,
        resource: BindingResource::Buffer(mesh_buffer.buffer.as_entire_buffer_binding()),
      },
      BindGroupEntry {
        binding: 3,
        resource: BindingResource::Buffer(num_of_meshes_buffer.as_entire_buffer_binding()),
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
    meshes: meshes_bind_group,
    materials: materials_bind_group,
    light_dir: light_dir_bind_group,
    seed: seed_bind_group,
  });
}

pub fn extract_meshes(
  mesh_assets: Extract<Res<Assets<Mesh>>>,
  material_assets: Extract<Res<Assets<StandardMaterial>>>,
  meshes_changed: Extract<
    Query<
      Entity,
      (
        Without<NotInScene>,
        Or<(
          Changed<Transform>,
          Changed<Handle<Mesh>>,
          Changed<Handle<StandardMaterial>>,
        )>,
      ),
    >,
  >,
  meshes: Extract<Query<(&Transform, &Handle<Mesh>, &Handle<StandardMaterial>), Without<NotInScene>>>,
  mesh_events: Extract<EventReader<AssetEvent<Mesh>>>,
  material_events: Extract<EventReader<AssetEvent<StandardMaterial>>>,
  mut vertex_storage: ResMut<VertexStorage>,
  mut mesh_storage: ResMut<MeshStorage>,
  mut material_storage: ResMut<MaterialStorage>,
) {
  if !mesh_events.is_empty() {
    let meshes_unique = meshes.iter().map(|(_, m, _)| m.clone()).unique().collect::<Vec<_>>();
    let mut vertex_buffer = vec![];
    let mut index_buffer = vec![];
    let mut mesh_map = HashMap::new();
    for handle in &meshes_unique {
      let mesh = mesh_assets.get(handle).unwrap();
      let position = mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap();
      let normal = mesh.attribute(Mesh::ATTRIBUTE_NORMAL).unwrap();
      let index = mesh.indices().unwrap();

      let vertex_base = vertex_buffer.len();
      vertex_buffer.extend(
        position
          .as_float3()
          .unwrap()
          .iter()
          .zip(normal.as_float3().unwrap().iter())
          .map(|(p, n)| ShaderVertex {
            position: *p,
            normal: *n,
            p1: [0; 4],
            p2: [0; 4],
          }),
      );
      let index_first = index_buffer.len();
      index_buffer.extend(index.iter().map(|x| (vertex_base + x) as u32).collect::<Vec<_>>());
      let index_len = index_buffer.len() - index_first;
      mesh_map.insert(handle.id(), (index_first, index_len));
    }
    *vertex_storage = VertexStorage {
      mesh_map,
      verticies: vertex_buffer,
      indicies: index_buffer,
    };
  }
  if !material_events.is_empty() {
    let mut material_map = HashMap::new();
    let mut material_vec = vec![];
    let materials_unique = meshes.iter().map(|(_, _, m)| m.clone()).unique().collect::<Vec<_>>();
    for handle in &materials_unique {
      let material = material_assets.get(handle).unwrap();
      material_vec.push(ShaderMaterial {
        color: material.base_color.as_rgba_f32(),
        emissive: material.emissive.as_rgba_f32(),
        roughness: material.perceptual_roughness,
        metallic: material.metallic,
        specular: material.reflectance,
        pad: [0; 4],
      });
      material_map.insert(handle.id(), material_vec.len() - 1);
    }
    *material_storage = MaterialStorage {
      material_vec,
      material_map,
    };
  }
  if meshes_changed.is_empty() {
    return;
  }
  let mut extracted_meshes = vec![];
  for (transform, mesh, mat) in meshes.iter() {
    extracted_meshes.push(ExtractedMesh {
      transform: transform.clone(),
      material: mat.id(),
      mesh: mesh.id(),
    });
  }
  *mesh_storage = MeshStorage {
    meshes: extracted_meshes,
  };
}

pub fn prepare_meshes(
  mut commands: Commands,
  render_device: ResMut<RenderDevice>,
  vertex_storage: Res<VertexStorage>,
  mesh_storage: Res<MeshStorage>,
  material_storage: Res<MaterialStorage>,
) {
  if vertex_storage.is_changed() {
    let vertex_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
      label: None,
      contents: bytemuck::cast_slice(vertex_storage.verticies.as_slice()),
      usage: BufferUsages::COPY_SRC | BufferUsages::STORAGE,
    });
    let index_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
      label: None,
      contents: bytemuck::cast_slice(vertex_storage.indicies.as_slice()),
      usage: BufferUsages::COPY_SRC | BufferUsages::STORAGE,
    });
    commands.insert_resource(VertexBuffer {
      vertex_buffer,
      index_buffer,
    });
  }
  if mesh_storage.is_changed() {
    let meshes = mesh_storage
      .meshes
      .iter()
      .map(
        |ExtractedMesh {
           transform,
           material,
           mesh,
         }| {
          ShaderMesh {
            transform: transform.compute_matrix(),
            start_index: vertex_storage.mesh_map.get(mesh).unwrap().0 as u32,
            len_index: vertex_storage.mesh_map.get(mesh).unwrap().1 as u32,
            material: *material_storage.material_map.get(material).unwrap() as u32,
            pad: [0; 4],
          }
        },
      )
      .collect::<Vec<_>>();
    let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
      label: None,
      contents: bytemuck::cast_slice(meshes.as_slice()),
      usage: BufferUsages::COPY_SRC | BufferUsages::STORAGE,
    });
    commands.insert_resource(MeshBuffer { buffer });
  }
  if material_storage.is_changed() {
    let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
      label: None,
      contents: bytemuck::cast_slice(material_storage.material_vec.as_slice()),
      usage: BufferUsages::COPY_SRC | BufferUsages::STORAGE,
    });
    commands.insert_resource(MaterialBuffer { buffer });
  }
}
