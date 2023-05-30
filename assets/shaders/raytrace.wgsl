#import bevy_render::view

@group(0) @binding(0) var<uniform> view: View;

@group(1) @binding(0)
var texture: texture_storage_2d<rgba8unorm, read_write>;
@group(1) @binding(1)
var<uniform> iter: u32;

struct Vertex {
  coord: vec3<f32>,
  normal: vec3<f32>,
};

struct Mesh {
  transform: mat4x4<f32>,
  start_index: u32,
  len_index: u32,
  material: u32
}

struct Material {
  color: vec4<f32>,
  emissive: vec4<f32>,
  roughness: f32,
  metallic: f32,
  specular: f32,
}

struct HitInfo {
  hit_point: vec3<f32>,
  distance: f32,
  normal: vec3<f32>,
  material:  u32
}

@group(2) @binding(0)
var<storage> verticies: array<Vertex>;
@group(2) @binding(1)
var<storage> indicies: array<u32>;
@group(2) @binding(2)
var<storage> meshes: array<Mesh>;
@group(2) @binding(3)
var<uniform> num_meshes: u32;

@group(3) @binding(0)
var<storage> materials: array<Material>;

@group(4) @binding(0)
var<uniform> light_dir: vec3<f32>;

@group(5) @binding(0)
var<uniform> seed: vec2<f32>;

struct Ray {
  org: vec3<f32>,
  dir: vec3<f32>
}

fn hit(hit_info: ptr<function, HitInfo>, ray: Ray) -> bool {
  var hit_flag = false;
  for (var mid: u32 = u32(0); mid < num_meshes; mid++) {
    let transform = meshes[mid].transform;
    let start_index = meshes[mid].start_index;
    let len_index = meshes[mid].len_index;
    let material = meshes[mid].material;
    for (var vid: u32 = start_index; vid < start_index + len_index; vid+=u32(3)) {
      let v0v = verticies[indicies[vid]];
      let v1v = verticies[indicies[vid+u32(1)]];
      let v2v = verticies[indicies[vid+u32(2)]];
      let v0 = (transform * vec4(v0v.coord, 1.0)).xyz;
      let v1 = (transform * vec4(v1v.coord, 1.0)).xyz;
      let v2 = (transform * vec4(v2v.coord, 1.0)).xyz;
      let v0v1 = v1 - v0;
      let v0v2 = v2 - v0;
      let n = cross(v0v1, v0v2);

      let ndotdir = dot(n, ray.dir);
      if (abs(ndotdir) < 0.00000001) {
        continue;
      }

      let d = -dot(n, v0);
      let t = -(dot(n, ray.org) + d) / ndotdir;
      if (t < 0.0) {
        continue;
      }

      let p = ray.org + t * ray.dir;
      let e0 = v1-v0;
      let vp0 = p - v0;
      if (dot(n, cross(e0, vp0)) < 0.0) { continue; }

      let e1 = v2-v1;
      let vp1 = p - v1;
      if (dot(n, cross(e1, vp1)) < 0.0) { continue; }

      let e2 = v0-v2;
      let vp2 = p - v2;
      if (dot(n, cross(e2, vp2)) < 0.0) { continue; }

      if ((!hit_flag || (t < (*hit_info).distance)) && dot(n, ray.dir) < 0.0) {
        hit_flag = true;
        (*hit_info).hit_point = p;

        let f0 = v0 - p;
        let f1 = v1 - p;
        let f2 = v2 - p;
        let a = length(cross(v0-v1, v0-v2));
        let a0 = length(cross(f1, f2)) / a;
        let a1 = length(cross(f2, f0)) / a;
        let a2 = length(cross(f0, f1)) / a;
        let normal = v0v.normal * a0 + v1v.normal * a1 + v2v.normal * a2;

        (*hit_info).normal = normal;
        (*hit_info).distance = t;
        (*hit_info).material = material;
      }
    }
  }
  return hit_flag;
}

fn miss(ray: Ray) -> vec4<f32> {
  var color = vec4(0.0, 0.5 * f32(ray.dir.y), 0.0, 1.0);
//  if ((ray.dir.x / 30.0 % 2 + ray.dir.y / 30.0 % 2.0) % 2 == 0) {
//    color.g = 1.0 - color.g;
//  }
  return color;
}

fn light(ray: Ray) -> f32 {
  let n = max(dot(normalize(ray.dir), -normalize(light_dir)), 0.0);
  return n;
}


fn random( p: ptr<function, vec2<f32>> ) -> f32 {
  let v = *p;
  let K1 : vec2<f32> = vec2( 23.14069263277926, 2.665144142690225 );
  let fx = fract( cos( dot(v,K1) ) * 12345.6789 );
  let fy = fract( cos( dot(v,K1) ) * 6789.12345 );
  *p = vec2(fract(cos(v.x * fx)), fract(cos(v.y * fy)));
  return fx;
}

@compute @workgroup_size(32, 32, 1)
fn main(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
  var seed: vec2<f32> = seed + vec2(f32(invocation_id.x), f32(invocation_id.y));
//  let p = view.view_proj;
  let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));
  let location_normalised = vec4<f32>(f32(invocation_id.x) / 1024.0 * 2.0 - 1.0, -f32(invocation_id.y) / 768.0 * 2.0 + 1.0, 1.0, 1.0);
//  let ray_org = vec3<f32>(0.0, 0.0, -2.0);
  var ray : Ray;
  ray.org = view.world_position;
//  let ray_dir = vec3<f32>(f32(invocation_id.x) / 1024.0 * 2.0 - 1.0, f32(invocation_id.y) / 1024.0 * 2.0 - 1.0, -1.0);
  let ray_target = view.inverse_projection * location_normalised;
  ray.dir = (view.inverse_view * vec4(normalize(ray_target.xyz / ray_target.w), 0.0)).xyz;

  var color = vec4(1.0, 1.0, 1.0, 1.0);
  var light = vec4(0.0, 0.0, 0.0, 0.0);
  var hit_info: HitInfo;
  // ---
  var ray_count = 0;
  while (true) {
    if (ray_count > 5) {
      break;
    }
    ray_count = ray_count + 1;
    if (hit(&hit_info, ray)) {
//      color = vec4(materials[hit_info.material].color.rgb, 1.0);
//      break;
      if (length(materials[hit_info.material].emissive.rgb) > 0.1) {
        light += vec4(color.rgb * materials[hit_info.material].emissive.rgb, 1.0);
        break;
      }
      let diffuseDir = reflect(ray.dir, hit_info.normal + (materials[hit_info.material].roughness - 0.089) * vec3(random(&seed) - 0.5, random(&seed) - 0.5, random(&seed) - 0.5));
      let specularDir = reflect(ray.dir, hit_info.normal);
      if (random(&seed) > materials[hit_info.material].metallic) {
        color = vec4(color.rgb * materials[hit_info.material].color.rgb, 1.0);
        ray.org = hit_info.hit_point + hit_info.normal * 0.0001;
        let s = materials[hit_info.material].specular;
        ray.dir = vec3((specularDir.x - diffuseDir.x) * s + diffuseDir.x, (specularDir.y - diffuseDir.y) * s + diffuseDir.y, (specularDir.z - diffuseDir.z) * s + diffuseDir.z);
      } else {
        ray.org = hit_info.hit_point + hit_info.normal * 0.0001;
        ray.dir = specularDir;
      }
    } else {
      if (ray_count == 1) {
        light = miss(ray);
//          color = light;
      } else {
        light += vec4(color.rgb * vec3(0.1, 0.1, 0.1), 1.0);
//        color = vec4((color.rgb * light(ray)), 1.0);
//        color = color * 0.0001;
      }
      break;
    }
  }
  color = light;
  let weight = 1.0 / (f32(iter) + 1.0);
  let new_color = vec4(textureLoad(texture, location).rgb * (1.0 - weight) + color.rgb * weight, 1.0);
//  let new_color = color;
//  let old_color = vec4(textureLoad(texture, location).rgb * f32(iter), 1.0);
//  let new_color = vec4((old_color + color).rgb / f32(iter + u32(1)), 1.0);
  textureStore(texture, location, new_color);
}