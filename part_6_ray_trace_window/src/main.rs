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

const WINDOW_WIDTH : u32 = 854;
const WINDOW_HEIGHT: u32 = 480;

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

    let timer = std::time::Instant::now();

    // Queue of scanlines
    let scanline_queue = std::sync::Arc::new(std::sync::Mutex::new(std::collections::VecDeque::new()));
    for y in 0..image.height {
        scanline_queue.lock().unwrap().push_back(y);
    }

    // Save these so we lock less often
    let image_width = image.width;
    let image_height = image.height;

    // Make these accessible across thread
    let shared_image = std::sync::Arc::new(std::sync::Mutex::new(image));
    let shared_scene = std::sync::Arc::new(scene);
    let shared_scanline_rendered = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let shared_stop_render = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));

    let num_cores = 8;
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

                            // Totally not efficient but is okay for this example
                            local_image.lock().unwrap().set_pixel(x, y, r, g, b);
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
    while (window.is_open()) {
        if (window.is_key_down(minifb::Key::Escape)) {
            shared_stop_render.store(true, std::sync::atomic::Ordering::Relaxed);
            write_file = false;
            break;
        }

        let has_new_scanline = shared_scanline_rendered.load(std::sync::atomic::Ordering::Relaxed);
        if (has_new_scanline) {
            let locked_image = shared_image.lock().unwrap();
            let pixels = locked_image.get_pixels();
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
        shared_image.lock().unwrap().write_ppm("window.ppm");
    }
}
