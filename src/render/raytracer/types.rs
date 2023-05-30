use bevy::asset::HandleId;
use bevy::prelude::*;
use bevy::render::extract_resource::ExtractResource;
use bevy::render::render_resource::{BindGroup, Buffer};
use bevy::utils::hashbrown::HashMap;
use bytemuck::{Pod, Zeroable};

#[derive(Resource, Clone, Deref, ExtractResource)]
pub struct RaytracingImage(pub Handle<Image>);

#[derive(Resource)]
pub struct RTCameraEntity(pub Entity);

#[derive(Resource, Clone, Deref, ExtractResource)]
pub struct PBRCameraEntity(pub Entity);

#[derive(Resource, Clone, ExtractResource)]
pub struct TextureIter(pub u32);

#[derive(Resource)]
pub struct RaytracingBindGroups {
  pub image: BindGroup,
  pub meshes: BindGroup,
  pub materials: BindGroup,
  pub light_dir: BindGroup,
  pub seed: BindGroup,
}

#[derive(Component)]
pub struct ColorComponent {
  pub color: Color,
}

#[derive(Component)]
pub struct RoughnessComponent {
  pub roughness: f32,
}

#[repr(C)]
#[derive(Zeroable, Pod, Copy, Clone)]
pub struct ShaderVertex {
  pub position: [f32; 3],
  pub p1: [u8; 4],
  pub normal: [f32; 3],
  pub p2: [u8; 4],
}

#[derive(Resource)]
pub struct VertexStorage {
  pub mesh_map: HashMap<HandleId, (usize, usize)>,
  pub verticies: Vec<ShaderVertex>,
  pub indicies: Vec<u32>,
}

impl Default for VertexStorage {
  fn default() -> Self {
    Self {
      mesh_map: Default::default(),
      verticies: vec![],
      indicies: vec![],
    }
  }
}

#[repr(C)]
#[derive(Zeroable, Pod, Copy, Clone)]
pub struct ShaderMaterial {
  pub color: [f32; 4],
  pub emissive: [f32; 4],
  pub roughness: f32,
  pub metallic: f32,
  pub specular: f32,
  pub pad: [u8; 4],
}

#[derive(Resource, Default)]
pub struct MaterialStorage {
  pub material_vec: Vec<ShaderMaterial>,
  pub material_map: HashMap<HandleId, usize>,
}

pub struct ExtractedMesh {
  pub transform: Transform,
  pub material: HandleId,
  pub mesh: HandleId,
}

#[repr(C)]
#[derive(Zeroable, Pod, Copy, Clone)]
pub struct ShaderMesh {
  pub transform: Mat4,
  pub start_index: u32,
  pub len_index: u32,
  pub material: u32,
  pub pad: [u8; 4],
}

#[derive(Resource, Default)]
pub struct MeshStorage {
  pub meshes: Vec<ExtractedMesh>,
}

#[derive(Resource)]
pub struct VertexBuffer {
  pub vertex_buffer: Buffer,
  pub index_buffer: Buffer,
}

#[derive(Resource)]
pub struct MeshBuffer {
  pub buffer: Buffer,
}

#[derive(Resource)]
pub struct MaterialBuffer {
  pub buffer: Buffer,
}
