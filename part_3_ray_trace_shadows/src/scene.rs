#![allow(dead_code)]

use crate::camera::Camera;
use crate::sphere::Sphere;
use crate::ray::Ray;

pub struct Scene {
    pub camera  : Camera,
    pub spheres : Vec<Sphere>,
}

impl Scene {
    pub fn trace_closest_hit(ray: Ray, t: &f32, sphere_index: &usize) -> bool {
        return false;
    }

    pub fn trace_any_hit(ray: Ray) -> bool {
        return false;
    }
}
