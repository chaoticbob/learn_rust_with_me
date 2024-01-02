#![allow(unused_parens)]
#![allow(unused_imports)]

use crate::primitives::{Cylinder, RoundedBox};
use crate::primitives::AABox;
use std::f32::consts::PI;
use crate::bitmap::Bitmap;
use crate::primitives::{Ellipsoid, Goursat, Plane, Sphere, Torus};
use crate::vec2::vec2;
use crate::vec3::{normalize, vec3};
use crate::vec3::Y_AXIS;
use crate::vec4::vec4;
use crate::scene::Scene;

mod bitmap;
mod mat4;
mod quat;
mod ray;
mod primitives;
mod sphere_flake;
mod vec3;
mod vec4;
mod camera;
mod scene;
mod vec2;
mod transform;

const WINDOW_WIDTH : u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;

fn main() {
    let mut window = minifb::Window::new(
        file!(),
        WINDOW_WIDTH as usize,
        WINDOW_HEIGHT as usize,
        minifb::WindowOptions::default(),
    ).unwrap();

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    // -------------------------------------------------------------------------

    let image = Bitmap::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    let aspect_ratio = (image.width as f32) / (image.height as f32);

    let mut scene = Scene::default();
    scene.camera.look_at(vec3(-2.0, 7.0, -5.0), vec3(-0.5, 1.0, 0.5), Y_AXIS);
    scene.camera.perspective(60.0, aspect_ratio, 1.0, 10000.0);

    scene.primitives.push(Box::new(Sphere {
        transform: transform::from_position(vec3(-2.0, 1.0, -1.0)),
        color: vec3(0.3, 0.7, 0.9),
    }));

    scene.primitives.push(Box::new(Ellipsoid {
        transform: transform::from_position(vec3(1.5, 1.0, -1.0)),
        radii: vec3(1.0, 0.5, 1.5),
        color: vec3(0.9, 0.7, 0.3),
    }));

    scene.primitives.push(Box::new(Goursat {
        transform: transform::from_position(vec3(2.0, 1.0, 3.0)),
        ka: 0.3,
        kb: 0.9,
        color: vec3(0.7, 0.9, 0.3),
    }));

    scene.primitives.push(Box::new(Torus {
        transform: transform::transform(vec3(-2.0, 0.5, 3.0), vec3(PI/2.0, 0.0, 0.0), vec3::ONE),
        major_radius: 1.0,
        minor_radius: 0.5,
        color: vec3(0.9, 0.3, 0.3),
    }));

    scene.primitives.push(Box::new(AABox {
        transform: transform::transform(vec3(5.0, 1.0, 1.0), vec3(0.0, 0.0, 0.0), vec3::ONE),
        size: vec3::vec3(0.8, 1.0, 1.5),
        color: vec3(0.9, 0.3, 0.83),
    }));

    scene.primitives.push(Box::new(Cylinder {
        transform: transform::transform(vec3(-5.0, 0.0, 6.0), vec3(0.0, 0.0, 0.0), vec3::ONE),
        start: vec3::vec3(0.0, 3.0, 0.0),
        end: vec3::vec3(0.0, 0.0, 0.0),
        radius: 1.0,
        color: vec3(0.5, 0.5, 0.5),
    }));

    scene.primitives.push(Box::new(RoundedBox {
        transform: transform::transform(vec3(-5.0, 1.2, 1.0), vec3(0.0, 0.0, 0.0), vec3::ONE),
        size: vec3(0.8, 0.8, 0.8),
        radius: 0.4,
        color: vec3(0.1, 0.1, 0.1),
    }));

    scene.primitives.push(Box::new(Plane {
        transform: transform::from_position(vec3(0.0, 0.0, 0.0)),
        color: 1.5 * vec3(0.430, 0.430, 0.440),
    }));

    scene.light = vec3(-3.0, 10.0, -5.0);

    let timer = std::time::Instant::now();

    // Queue of scanlines
    let scanline_queue = std::sync::Arc::new(std::sync::Mutex::new(std::collections::VecDeque::new()));
    for y in 0..image.height {
        scanline_queue.lock().unwrap().push_back(y);
    }

    // Save these so we lock less often
    let image_width = image.width;
    let image_height = image.height;

    // Make these accessible across threads
    let shared_image = std::sync::Arc::new(std::sync::Mutex::new(image));
    let shared_scene = std::sync::Arc::new(scene);
    let shared_scanline_rendered = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let shared_stop_render = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));

    // Spawn threads to ray trace each scanline
    let num_cores = num_cpus::get() - 1;
    let mut threads = Vec::new();
    for _i in 0..num_cores {
        let local_scanlines = scanline_queue.clone();
        let local_image = shared_image.clone();
        let local_scene = shared_scene.clone();
        let local_scanline_rendered = shared_scanline_rendered.clone();
        let local_stop_render = shared_stop_render.clone();
        let thread = std::thread::spawn(move || {
            loop {
                let scanline = local_scanlines.lock().unwrap().pop_front();
                match scanline {
                    Some(y) => {
                        // Get pointer to scanline
                        let mut ptr = local_image.lock().unwrap().get_scanline(y);

                        let mut stop_render = false;
                        for x in 0..image_width {
                            stop_render = local_stop_render.load(std::sync::atomic::Ordering::Relaxed);
                            if (stop_render) {
                                break;
                            }

                            let u = (x as f32) / (image_width as f32);
                            let v = (y as f32) / (image_height as f32);

                            let ray = local_scene.camera.generate_ray(vec2(u, v));
                            let color = local_scene.trace_recursive(ray, 0, 3);

                            // Swap R and B due to minifb's pixel format
                            let r: u8 = (color.z * 255.0) as u8;
                            let g: u8 = (color.y * 255.0) as u8;
                            let b: u8 = (color.x * 255.0) as u8;

                            // Write pixel - doesn't look like you can index using [] as you would in C/C++.
                            unsafe {
                                *ptr = r;
                                ptr = ptr.offset(1);

                                *ptr = g;
                                ptr = ptr.offset(1);

                                *ptr = b;
                                ptr = ptr.offset(1);

                                *ptr = 255;
                                ptr = ptr.offset(1);
                            }
                        }
                        if (stop_render) {
                            break;
                        }

                        local_scanline_rendered.store(true, std::sync::atomic::Ordering::Relaxed);
                        println!("Traced scanline {}", y);
                    },
                    None => break,
                }
            }
        });
        threads.push(thread);
    }

    // -------------------------------------------------------------------------

    let mut write_file = true;
    let mut wrote_time = false;

    // Loop while the window is open
    while (window.is_open()) {
        if (window.is_key_down(minifb::Key::Escape)) {
            shared_stop_render.store(true, std::sync::atomic::Ordering::Relaxed);
            write_file = false;
            break;
        }

        let has_new_scanline = shared_scanline_rendered.load(std::sync::atomic::Ordering::Relaxed);
        if (has_new_scanline) {
            // Lock image
            let locked_image = shared_image.lock().unwrap();
            // Get pixels
            let pixels = locked_image.get_pixels();
            // Cast from &Vec<u8> to &[u32]
            let u32_pixels = unsafe { std::slice::from_raw_parts(pixels.as_ptr() as *const u32, (WINDOW_WIDTH * WINDOW_HEIGHT * 4) as usize) };

            window.update_with_buffer(u32_pixels, WINDOW_WIDTH as usize, WINDOW_HEIGHT as usize).unwrap();

            shared_scanline_rendered.store(false, std::sync::atomic::Ordering::Relaxed);
        }
        else {
            window.update();
        }

        if (scanline_queue.lock().unwrap().is_empty()) {
            if (!wrote_time) {
                println!("Ray trace took: {} seconds", timer.elapsed().as_secs_f32());
                wrote_time = true;
            }
        }
    }

    // -------------------------------------------------------------------------

    for thread in threads {
        thread.join().unwrap();
    }

    if (write_file) {
        shared_image.lock().unwrap().swap_red_and_blue();
        shared_image.lock().unwrap().write_png("part_7_ray_trace_primitives.png");
    }
}
