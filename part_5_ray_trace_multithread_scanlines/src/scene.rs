#![allow(dead_code)]
#![allow(non_snake_case)]

use crate::camera::Camera;
use crate::sphere::Sphere;
use crate::ray::Ray;
use crate::vec3;
use crate::vec3::*;

#[derive(Default)]
pub struct Scene {
    pub camera  : Camera,
    pub spheres : Vec<Sphere>,
    pub light   : Vec3,
}

impl Scene {
    pub fn trace_closest_hit(&self, ray: Ray, hit_index: &mut usize, closest_t: &mut f32) -> bool {
        *closest_t = f32::MAX;
        *hit_index = usize::MAX;
        for (i, sphere) in self.spheres.iter().enumerate() {
            let mut t = f32::MAX;
            let hit = sphere.intersect(&ray, &mut t);
            if (hit && (t > 0.0) && (t < *closest_t)) {
                *hit_index = i;
                *closest_t = t;
            }
        }

        if (*hit_index != usize::MAX) {
            return true;
        }

        return false;
    }

    pub fn trace_any_hit(&self, ray: Ray) -> bool {
        for sphere in self.spheres.iter() {
            let mut t = f32::MAX;
            let hit = sphere.intersect(&ray, &mut t);
            if (hit && (t > 0.0)) {
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
        let c = a + d + s;
        return c;
    }

    pub fn shade(&self, ray: &Ray, hit_index: usize, t: f32) -> Vec3 {
        // Light
        let P = ray.pos + t * ray.dir;
        let N = self.spheres[hit_index].get_normal(P);
        let V = normalize(self.camera.get_eye() - P);
        let c = self.phong(P, N, V);
        let color = c *self.spheres[hit_index].color;

        // Shadow
        let mut shadow : f32 = 0.0;
        let P = ray.pos + (t * 0.9999) * ray.dir;
        let shadowRay = Ray { pos: P, dir: vec3::normalize(self.light - P)};
        let hit = self.trace_any_hit(shadowRay);
        if (hit) {
            shadow = 0.7;
        }

        return color * (1.0 - shadow);
    }

    pub fn trace_recursive(&self, ray: Ray, depth: u32, max_depth: u32) -> Vec3 {
        if (depth >= max_depth) {
            return vec3(0.0, 0.0, 0.0);
        }

        let mut closest_t = f32::MAX;
        let mut hit_index = usize::MAX;
        let hit = self.trace_closest_hit(ray, &mut hit_index, &mut closest_t);
        if (!hit) {
            // Sky color
            return 0.2 * vec3(0.88, 0.90, 0.94);
        }

        let mut P = ray.pos + closest_t * ray.dir;
        let N = self.spheres[hit_index].get_normal(P);
        let R = vec3::reflect(ray.dir, N);

        P = P - (0.001 * ray.dir);
        let reflectionRay = Ray{ pos: P, dir: R };

        let reflection = self.trace_recursive(reflectionRay, depth + 1, max_depth);
        let color = self.shade(&ray, hit_index, closest_t);

        return color + 0.5 * reflection;
    }
}
