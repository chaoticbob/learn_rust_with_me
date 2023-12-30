#![allow(unused_parens)]

use crate::ray::Ray;
use crate::vec3::vec3;
use crate::vec4::vec4;
use crate::sphere::Sphere;
use crate::bitmap::Bitmap;
use crate::sphere_flake::generate_sphere_flake;

mod vec3;
mod ray;
mod bitmap;
mod sphere;
mod vec4;
mod mat4;
mod sphere_flake;
mod quat;

fn main() {
    let mut image = Bitmap::new(854, 480);
    let aspect_ratio = (image.width as f32) / (image.height as f32);
    let mut spheres = Vec::<Sphere>::new();
    spheres.push(Sphere { pos: vec3(0.0, 1.0, 0.0), radius: 1.0, color: vec3(0.3, 0.7, 0.9) });
    spheres.push(Sphere { pos: vec3(0.0, -1000.0, 0.0), radius: 1000.0, color: 1.5 * vec3(0.490, 0.430, 0.295) });

    generate_sphere_flake(0, 3, 1.0 / 3.0, 1.0, vec3(0.0, 1.0, 0.0), vec3(0.0, 1.0, 0.0), &mut spheres);

    let eye_pos = vec3(0.0, 4.0, -3.0);
    let center = vec3(0.0, 1.0, 0.0);
    let view_matrix = mat4::look_at_LH(eye_pos, center, vec3(0.0, 1.0, 0.0));
    let proj_matrix = mat4::perspective_LH((60 as f32).to_radians(), aspect_ratio, 1.0, 10000.0);
    let inv_view_matrix = mat4::inverse(view_matrix);
    let inv_proj_matrix = mat4::inverse(proj_matrix);

    let light_pos = vec3(2.0, 25.0, -5.0);

    for y in 0..image.height {
        for x in 0..image.width {
            let u = (x as f32) / (image.width as f32);
            let v = (y as f32) / (image.height as f32);
            let du = (u * 2.0 - 1.0);
            let dv = -(v * 2.0 - 1.0);

            let origin = (inv_view_matrix * vec4(0.0, 0.0, 0.0, 1.0)).as_vec3();
            let target = (inv_proj_matrix * vec4(du, dv, 1.0, 1.0)).as_vec3();
            let direction = (inv_view_matrix * vec4::as_vec4(target, 0.0)).as_vec3();
            let ray = Ray { pos: origin, dir: direction };

            let mut hit_index = usize::MAX;
            let mut closest_t = f32::MAX;
            for (i, sphere) in spheres.iter().enumerate() {
                let mut t = f32::MAX;
                let hit = sphere.intersect(&ray, &mut t);
                if (hit && (t > 0.0) && (t < closest_t)) {
                    hit_index = i;
                    closest_t = t;
                }
            }

            let mut color = vec3(0.0, 0.0, 0.0);
            if (hit_index != usize::MAX) {
                color = spheres[hit_index].shade(&ray, closest_t, eye_pos, light_pos);
            }

            let r: u8 = (color.x * 255.0) as u8;
            let g: u8 = (color.y * 255.0) as u8;
            let b: u8 = (color.z * 255.0) as u8;

            image.set_pixel(x, y, r, g, b);
        }
        println!("Traced scanline {}", y);
    }

    image.write_ppm("part_2_ray_trace_some_spheres.ppm");
}
