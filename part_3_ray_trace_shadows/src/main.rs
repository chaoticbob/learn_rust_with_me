#![allow(unused_parens)]

use crate::bitmap::Bitmap;
use crate::sphere::Sphere;
use crate::sphere_flake::generate_sphere_flake;
use crate::vec2::vec2;
use crate::vec3::vec3;
use crate::vec3::Y_AXIS;
use crate::vec4::vec4;
use crate::scene::Scene;

mod bitmap;
mod mat4;
mod quat;
mod ray;
mod sphere;
mod sphere_flake;
mod vec3;
mod vec4;
mod camera;
mod scene;
mod vec2;

fn main() {
    let mut image = Bitmap::new(854, 480);
    let aspect_ratio = (image.width as f32) / (image.height as f32);

    let mut scene = Scene::default();
    scene.camera.look_at(vec3(0.0, 4.0, -3.0), vec3(0.0, 1.0, 0.0), Y_AXIS);
    scene.camera.perspective(60.0, aspect_ratio, 1.0, 10000.0);

    scene.spheres.push(Sphere {
        pos: vec3(0.0, 1.0, 0.0),
        radius: 1.0,
        color: vec3(0.3, 0.7, 0.9),
    });
    scene.spheres.push(Sphere {
        pos: vec3(0.0, -1000.0, 0.0),
        radius: 1000.0,
        color: vec3(0.980, 0.863, 0.596),
    });
    generate_sphere_flake(
        0,
        3,
        1.0 / 3.0,
        1.0,
        vec3(0.0, 1.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        &mut scene.spheres,
    );

    scene.light = vec3(2.0, 25.0, -5.0);

    for y in 0..image.height {
        for x in 0..image.width {
            let u = (x as f32) / (image.width as f32);
            let v = (y as f32) / (image.height as f32);

            let ray = scene.camera.generate_ray(vec2(u, v));

            let mut closest_t = f32::MAX;
            let mut hit_index = usize::MAX;
            let hit = scene.trace_closest_hit(ray, &mut hit_index, &mut closest_t);

            let mut color = vec3(0.0, 0.0, 0.0);
            if (hit) {
                color = scene.shade(&ray, hit_index, closest_t);
            }

            let r: u8 = (color.x * 255.0) as u8;
            let g: u8 = (color.y * 255.0) as u8;
            let b: u8 = (color.z * 255.0) as u8;
            image.set_pixel(x, y, r, g, b);
        }
        println!("Traced scanline {}", y);
    }

    image.write_ppm("shadows.ppm");
}