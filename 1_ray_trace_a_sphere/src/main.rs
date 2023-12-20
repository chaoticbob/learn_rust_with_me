#![allow(unused_parens)]

use crate::vec3::*;
use crate::sphere::Sphere;
use crate::bitmap::Bitmap;
use crate::ray::Ray;

mod vec3;
mod ray;
mod bitmap;
mod sphere;

pub fn generate_ray(eye_pos : Vec3, u: f32, v: f32, fov: f32, aspect_ratio: f32) -> Ray {
    let theta = fov.to_radians() / 2.0;
    let h = theta.tan();
    let w = h * aspect_ratio;
    let s = u * w;
    let t = v * h;
    let dir = vec3::normalize(vec3(s, t, 1.0) - eye_pos);
    Ray{pos: eye_pos, dir: dir}
}

fn main() {
    let mut image = Bitmap::new(640, 480);
    let aspect_ratio = (image.width as f32) / (image.height as f32);
    let sphere = Sphere{pos : vec3(0.0, 0.0, 1.0), radius: 0.25};

    let eye_pos = vec3(0.0, 0.0, 0.0);

    for y in 0..image.height {
        for x in 0..image.width {
            let u = (x as f32) / (image.width as f32);
            let v = (y as f32) / (image.height as f32);
            let du =  (2.0 * u - 1.0);
            let dv = -(2.0 * v - 1.0);
            let ray = generate_ray(eye_pos, du, dv, 60.0, aspect_ratio);

            let mut r: u8 = 0;
            let mut g: u8 = 0;
            let mut b: u8 = 0;

            let mut t = 0.0;
            let hit = sphere.intersect(&ray, &mut t);
            if (hit) {
                let color = sphere.shade(&ray, t, eye_pos);
                r = (color.x * 255.0) as u8;
                g = (color.y * 255.0) as u8;
                b = (color.z * 255.0) as u8;
            }

            image.set_pixel(x, y, r, g, b);
        }
    }

    image.write_ppm("sphere.ppm");
}
