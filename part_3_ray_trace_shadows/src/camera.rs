#![allow(dead_code)]

use crate::{mat4, vec4};
use crate::vec2::*;
use crate::vec3::*;
use crate::mat4::*;
use crate::ray::Ray;

pub struct Camera {
    eye:             Vec3,
    center:          Vec3,
    up:              Vec3,
    fovy:            f32, // Degrees
    aspect_ratio:    f32,
    near_clip:       f32,
    far_clip:        f32,
    view_matrix:     Mat4,
    proj_matrix:     Mat4,
    inv_view_matrix: Mat4,
    inv_proj_matrix: Mat4,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            eye:             vec3(0.0, 0.0, -1.0),
            center:          vec3(0.0, 0.0, 0.0),
            up:              vec3(0.0, 1.0, 0.0),
            fovy:            60.0,
            aspect_ratio:    1.0,
            near_clip:       1.0,
            far_clip:        10000.0,
            view_matrix:     mat4::identity(),
            proj_matrix:     mat4::identity(),
            inv_view_matrix: mat4::identity(),
            inv_proj_matrix: mat4::identity(),
        }
    }
}

impl Camera {
    pub fn new() -> Camera {
        Camera::default()
    }

    pub fn get_eye(&self) -> Vec3 {
        self.eye
    }

    pub fn look_at(&mut self, eye: Vec3, center: Vec3, up: Vec3) {
        self.eye = eye;
        self.center = center;
        self.up = up;
        self.view_matrix = mat4::look_at_LH(self.eye, self.center, self.up);
        self.inv_view_matrix = mat4::inverse(self.view_matrix);
    }

    pub fn perspective(&mut self, fovy: f32, aspect_ratio: f32, near_clip: f32, far_clip: f32) {
        self.fovy = fovy;
        self.aspect_ratio = aspect_ratio;
        self.near_clip = near_clip;
        self.far_clip = far_clip;
        self.proj_matrix = mat4::perspective_LH(self.fovy.to_radians(), self.aspect_ratio, self.near_clip, self.far_clip);
        self.inv_proj_matrix = mat4::inverse(self.proj_matrix);
    }

    // Expected range of uv is [0, 1)
    pub fn generate_ray(&self, uv: Vec2) -> Ray {
        let mut d = (uv * 2.0) - 1.0;
        d.y = -d.y;

        let origin    = (self.inv_view_matrix * vec4(0.0, 0.0, 0.0, 1.0)).as_vec3();
        let target    = (self.inv_proj_matrix * vec4(d.x, d.y, 1.0, 1.0)).as_vec3();
        let direction = (self.inv_view_matrix * vec4::as_vec4(target, 0.0)).as_vec3();

        Ray {
            pos: origin,
            dir: direction,
        }
    }
}