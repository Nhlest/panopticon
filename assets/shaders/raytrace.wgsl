#import bevy_render::view

@group(0) @binding(0) var<uniform> view: View;

@group(1) @binding(0)
var texture: texture_storage_2d<rgba8unorm, write>;

struct Sphere {
  color: vec4<f32>,
  coord: vec3<f32>,
  radius: f32,
};

@group(2) @binding(0)
var<storage> spheres: array<Sphere>;
@group(2) @binding(1)
var<uniform> num_spheres: i32;

@group(3) @binding(0)
var<uniform> light_dir: vec3<f32>;

fn random( p: vec2<f32> ) -> f32 { let K1 : vec2<f32> = vec2( 23.14069263277926, 2.665144142690225 ); return fract( cos( dot(p,K1) ) * 12345.6789 ); }

@compute @workgroup_size(16, 16, 1)
fn main(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
//  let p = view.view_proj;
  let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));
  let location_normalised = vec4<f32>(f32(invocation_id.x) / 1024.0 * 2.0 - 1.0, -f32(invocation_id.y) / 768.0 * 2.0 + 1.0, 1.0, 1.0);
//  let ray_org = vec3<f32>(0.0, 0.0, -2.0);
  let ray_org = view.world_position;
//  let ray_dir = vec3<f32>(f32(invocation_id.x) / 1024.0 * 2.0 - 1.0, f32(invocation_id.y) / 1024.0 * 2.0 - 1.0, -1.0);
  let ray_target = view.inverse_projection * location_normalised;
  let ray_dir = (view.inverse_view * vec4(normalize(ray_target.xyz / ray_target.w), 0.0)).xyz;

  var color = vec4(0.0, 0.5 * f32(location.y) / 768.0, 0.0, 1.0);
  if ((location.x / 30 % 2 + location.y / 30 % 2) % 2 == 0) {
    color.g = 1.0 - color.g;
  }
  for (var i: i32 = 0; i < num_spheres; i ++) {
    let s_c = spheres[i].coord;
//    let c = dot(ray_org, ray_org) - spheres[i].radius * spheres[i].radius;
//
//    let dist = sqrt(pow(f32(s_c.x - location.x),2.0) + pow(f32(s_c.y - location.y),2.0));
//    if (dist <= spheres[i].radius) {
//      color = (1.0 - dist / spheres[i].radius) * spheres[i].color;
//      break;
//    }
    let radius = spheres[i].radius;
    let a = dot(ray_dir, ray_dir);
    let b = 2.0 * dot(ray_org - s_c, ray_dir);
    let c = dot(ray_org - s_c, ray_org - s_c) - radius * radius;
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
      let x = normalize(ray_org - s_c + ray_dir * t);
      let n = dot(x, -light_dir);
      color = vec4(spheres[i].color.rgb * n, 1.0);
    }
  }
  textureStore(texture, location, color);
}