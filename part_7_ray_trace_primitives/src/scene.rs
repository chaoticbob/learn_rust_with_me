#![allow(dead_code)]
#![allow(non_snake_case)]

use crate::camera::Camera;
use crate::primitives::Primitive;
use crate::ray::Ray;
use crate::vec3;
use crate::vec3::*;

#[derive(Default)]
pub struct Scene {
    pub camera     : Camera,
    pub primitives : Vec<Box<dyn Primitive + Sync + Send>>, // Sync + Send required sharing between threads
    pub light      : Vec3,
}

impl Scene {
    pub fn trace_closest_hit(&self, ray: Ray, hit_index: &mut usize, closest_t: &mut f32, closest_P: &mut Vec3, closest_N: &mut Vec3) -> bool {
        *closest_t = f32::MAX;
        *hit_index = usize::MAX;
        for (i, prim) in self.primitives.iter().enumerate() {
            let mut t = f32::MAX;
            let mut P = vec3::ZERO;
            let mut N = vec3::ZERO;
            let hit = prim.intersect_illum(&ray, &mut t, &mut P, &mut N);
            if (hit && (t > 0.0) && (t < *closest_t)) {
                *hit_index = i;
                *closest_t = t;
                *closest_P = P;
                *closest_N = N;
            }
        }

        if (*hit_index != usize::MAX) {
            return true;
        }

        return false;
    }

    pub fn trace_any_hit(&self, ray: Ray) -> bool {
        for prim in self.primitives.iter() {
            let hit = prim.intersect_shadow(&ray);
            if hit {
                return true;
            }
        }
        return false;
    }

    fn phong(&self, P: Vec3, N: Vec3, V: Vec3) -> f32 {
        let L = normalize(self.light - P);
        let R = reflect(-L, N);
        let d = dot(N, L).max(0.0);
        let s = dot(R, V).max(0.0).powf(30.0);
        let a = 0.2;
        let c = a + (0.7 * d) + (0.3 * s);
        return c;
    }

    pub fn shade(&self, hit_index: usize, P: Vec3, N: Vec3) -> Vec3 {
        // Light
        let V = normalize(self.camera.get_eye() - P);
        let c = self.phong(P, N, V);
        let color = c * *self.primitives[hit_index].get_color();

        // Shadow
        let mut shadow: f32 = 0.0;
        let shadow_pos = P + (0.01 * N);
        let shadow_dir = vec3::normalize(self.light - P);
        let shadow_ray = Ray { pos: shadow_pos, dir: shadow_dir };
        let hit = self.trace_any_hit(shadow_ray);
        if (hit) {
            shadow = 0.7;
        }

        return color * (1.0 - shadow);
    }

    pub fn trace_recursive(&self, ray: Ray, depth: u32, max_depth: u32) -> Vec3 {
        if (depth >= max_depth) {
            return vec3(0.0, 0.0, 0.0);
        }

        let mut hit_index = usize::MAX;
        let mut t = f32::MAX;
        let mut P = vec3::ZERO;
        let mut N = vec3::ZERO;
        let hit = self.trace_closest_hit(ray, &mut hit_index, &mut t, &mut P, &mut N);
        if (!hit) {
            // Sky color
            return 0.8 * get_sky_color(ray.dir, normalize(self.light));
        }

        let reflection_pos = P + (0.01 * N);
        let reflection_dir = normalize(vec3::reflect(ray.dir, N));
        let reflectionRay = Ray{ pos: reflection_pos, dir: reflection_dir };
        let reflection = self.trace_recursive(reflectionRay, depth + 1, max_depth);

        let color = self.shade(hit_index, P, N);

        return color + 0.5 * reflection;
    }
}

// https://www.shadertoy.com/view/tl23Rm
fn get_sky_color(dir: Vec3, L: Vec3) -> Vec3 {
    let mut color = mix(vec3(1.0, 1.0, 1.0), vec3(0.5, 0.7, 1.0), 0.5 + 0.5*dir.y);
    let sun = dot(L, dir).clamp(0.0, 1.0);
    color += (vec3(1.0, 0.6, 0.1) * sun.powf(4.0)) + vec3::from_scalar(10.0 * sun.powf(32.0));
    return color;
}