#![allow(dead_code)]
#![allow(unused_parens)]
#![allow(non_snake_case)]

use crate::ray::Ray;
use crate::vec3::*;

pub struct Sphere {
    pub pos: Vec3,
    pub radius: f32,
    pub color: Vec3,
}

impl Sphere {
    pub fn intersect(&self, ray: &Ray, t: &mut f32) -> bool {
        let f = ray.pos - self.pos;
        let a = dot(ray.dir, ray.dir);
        let bi = dot(-f, ray.dir);
        let c = dot(f, f) - (self.radius * self.radius);
        let s = f + (bi / a) * ray.dir;
        let discr = self.radius * self.radius - dot(s, s);

        let mut t1: f32 = -1.0;
        let mut t2: f32 = -1.0;
        let mut hit = false;
        if (discr >= 0.0) {
            let q = bi + bi.signum() * (a * discr).sqrt();
            t1 = c / q;
            t2 = q / a;
            hit = true;
        }

        *t = if (t1 < t2) { t1 } else { t2 };

        return hit;
    }

    pub fn get_normal(&self, P : Vec3) -> Vec3 {
        let N = normalize(P - self.pos);
        N
    }
}
