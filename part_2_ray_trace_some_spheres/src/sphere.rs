#![allow(dead_code)]
#![allow(unused_parens)]
#![allow(non_snake_case)]

use crate::vec3::*;
use crate::ray::Ray;

pub struct Sphere {
    pub pos   : Vec3,
    pub radius: f32,
    pub color : Vec3,
}

impl Sphere {
    pub fn intersect(&self, ray : &Ray, t: &mut f32) -> bool {
        let f     = ray.pos - self.pos;
        let a     = dot(ray.dir, ray.dir);
        let bi    = dot(-f, ray.dir);
        let c     = dot(f, f) - (self.radius * self.radius);
        let s     = f + (bi / a) * ray.dir;
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

        hit
    }

    pub fn shade(&self, ray: &Ray, t: f32, eye_pos: Vec3, light_pos: Vec3) -> Vec3 {
        let P = ray.pos + t * ray.dir;
        let N = normalize(P - self.pos);
        let L = normalize(light_pos - P);
        let V = normalize(eye_pos - P);
        let R = reflect(-L, N);
        let d = dot(N, L).max(0.0);
        let s = dot(R, V).max(0.0).powf(30.0);
        let a = 0.2;
        let c = a + d + s;
        Vec3{x: c, y: c, z: c} * self.color
    }
}

