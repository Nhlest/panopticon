#import bevy_render::view

@group(0) @binding(0) var<uniform> view: View;

@group(1) @binding(0)
var texture: texture_storage_2d<rgba8unorm, read_write>;
@group(1) @binding(1)
var<uniform> iter: u32;

struct Sphere {
  coord: vec3<f32>,
  radius: f32,
  material: u32
};

struct Material {
  color: vec4<f32>,
  roughness: f32
}

struct HitInfo {
  hit_point: vec3<f32>,
  distance: f32,
  normal: vec3<f32>,
  material:  u32
}

@group(2) @binding(0)
var<storage> spheres: array<Sphere>;
@group(2) @binding(1)
var<uniform> num_spheres: i32;
@group(2) @binding(2)
var<storage> materials: array<Material>;

@group(3) @binding(0)
var<uniform> light_dir: vec3<f32>;

@group(4) @binding(0)
var<uniform> seed: vec2<f32>;

struct Ray {
  org: vec3<f32>,
  dir: vec3<f32>
}

fn hit(hit_info: ptr<function, HitInfo>, ray: Ray) -> bool {
  var hit_flag = false;
  for (var i: i32 = 0; i < num_spheres; i ++) {
    let s_c = spheres[i].coord;
    let radius = spheres[i].radius;
    let a = dot(ray.dir, ray.dir);
    let b = 2.0 * dot(ray.org - s_c, ray.dir);
    let c = dot(ray.org - s_c, ray.org - s_c) - radius * radius;
    let d = b * b - 4.0 * a * c;
    let light_dir = normalize(light_dir);
    if (d >= 0.0) {
      var t = (-b - sqrt(d)) / (2.0 * a);
      if (t < 0.0) {
        t = (-b + sqrt(d)) / (2.0 * a);
        if (t < 0.0) {
          continue;
        }
      }
      if (!hit_flag || (t < (*hit_info).distance)) {
        hit_flag = true;
        let x = normalize(ray.org - s_c + ray.dir * t);
        let n = max(dot(x, -light_dir), 0.0);
        (*hit_info).hit_point = ray.org + ray.dir * t;
        (*hit_info).normal = x;
        (*hit_info).distance = t;
        (*hit_info).material = spheres[i].material;
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

@compute @workgroup_size(16, 16, 1)
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
  var hit_info: HitInfo;
  // ---
  var ray_count = 0;
  while (true) {
    if (ray_count > 5) {
      break;
    }
    ray_count = ray_count + 1;
    if (hit(&hit_info, ray)) {
      color = vec4(color.rgb * materials[hit_info.material].color.rgb, 1.0);
      ray.org = hit_info.hit_point + hit_info.normal * 0.0001;
      ray.dir = reflect(ray.dir, hit_info.normal + (materials[hit_info.material].roughness - 0.089) * vec3(random(&seed) - 0.5, random(&seed) - 0.5, random(&seed) - 0.5));
    } else {
      if (ray_count == 1) {
        color = miss(ray);
      } else {
        color = vec4((color.rgb * light(ray)), 1.0);
      }
      break;
    }
  }
  let old_color = vec4(textureLoad(texture, location).rgb * f32(iter), 1.0);
  let new_color = vec4((old_color + color).rgb / f32(iter + u32(1)), 1.0);
  textureStore(texture, location, new_color);
}