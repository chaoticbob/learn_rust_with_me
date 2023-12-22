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

    pub fn shade(&self, ray: &Ray, hit_index: usize, closest_t: f32) -> Vec3 {
        let color = self.spheres[hit_index].shade(&ray, closest_t, self.camera.get_eye(), self.light);

        let mut shadow : f32 = 0.0;
        let P = ray.pos + (closest_t * 0.9999) * ray.dir;
        let shadowRay = Ray { pos: P, dir: vec3::normalize(self.light - P)};
        let hit = self.trace_any_hit(shadowRay);
        if (hit) {
            shadow = 0.7;
        }

        return color * (1.0 - shadow);
    }
}
