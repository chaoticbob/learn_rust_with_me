#![allow(dead_code)]

use crate::vec3::*;

//#[derive(Default)]
pub struct Camera {
    eye:          Vec3,
    center:       Vec3,
    up:           Vec3,
    fovy:         f32, // Degrees
    aspect_ratio: f32,
    near_clip:    f32,
    far_clip:     f32,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            eye:          vec3(0.0, 0.0, -1.0),
            center:       vec3(0.0, 0.0, 0.0),
            up:           vec3(0.0, 1.0, 0.0),
            fovy:         60.0,
            aspect_ratio: 1.0,
            near_clip:    1.0,
            far_clip:     10000.0
        }
    }
}

impl Camera {
    pub fn new() -> Camera {
        Camera::default()
    }

    pub fn look_at(&mut self, eye: Vec3, center: Vec3, up: Vec3) {
        self.eye = eye;
        self.center = center;
        self.up = up;
    }

    pub fn perspective(&mut self, fovy: f32, aspect_ratio: f32, near_clip: f32, far_clip: f32) {
        self.fovy = fovy;
        self.aspect_ratio = aspect_ratio;
        self.near_clip = near_clip;
        self.far_clip = far_clip;
    }
}