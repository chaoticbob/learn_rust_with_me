#![allow(dead_code)]

use crate::{mat4, vec3, vec4};
use crate::vec3::*;
use crate::mat4::*;
use crate::vec4::as_vec4;

pub struct Transform {
    translation          : Vec3,
    rotation             : Vec3,
    scale_factor         : Vec3,
    translation_matrix   : Mat4,
    rotation_matrix      : Mat4,
    scale_matrix         : Mat4,
    transform_matrix     : Mat4,
    inv_transform_matrix : Mat4,
}

impl Default for Transform {
    fn default() -> Self {
        Transform {
            translation          : vec3(0.0, 0.0, 0.0),
            rotation             : vec3(0.0, 0.0, 0.0),
            scale_factor         : vec3(1.0, 1.0, 1.0),
            translation_matrix   : mat4::identity(),
            rotation_matrix      : mat4::identity(),
            scale_matrix         : mat4::identity(),
            transform_matrix     : mat4::identity(),
            inv_transform_matrix : mat4::identity(),
        }
    }
}

impl Transform {
    pub fn new() -> Transform {
        Transform::default()
    }

    pub fn translate(&mut self, position: Vec3) {
        self.translation = position;
        self.translation_matrix = mat4::translate(self.translation);
        self.update_transform();
    }

    pub fn rotate(&mut self, euler_angles: Vec3) {
        self.rotation = euler_angles;
        self.rotation_matrix = mat4::rotate(self.rotation, RotationOrder::XYZ);
        self.update_transform();
    }

    pub fn scale(&mut self, scale_factor: Vec3) {
        self.scale_factor = scale_factor;
        self.scale_matrix = mat4::scale(self.scale_factor);
        self.update_transform();
    }

    fn update_transform(&mut self) {
        self.transform_matrix = self.translation_matrix * self.rotation_matrix * self.scale_matrix;
        self.inv_transform_matrix = mat4::inverse(self.transform_matrix)
    }

    pub fn local_to_world_point(&self, point: Vec3) -> Vec3 {
        let v = self.transform_matrix * as_vec4(point, 1.0);
        vec3(v.x, v.y, v.z)
    }

    pub fn local_to_world_vector(&self, vector: Vec3) -> Vec3 {
        let v = self.transform_matrix * as_vec4(vector, 0.0);
        vec3(v.x, v.y, v.z)
    }

    pub fn world_to_local_point(&self, point: Vec3) -> Vec3 {
        let v = self.inv_transform_matrix * as_vec4(point, 1.0);
        vec3(v.x, v.y, v.z)
    }

    pub fn world_to_local_vector(&self, vector: Vec3) -> Vec3 {
        let v = self.inv_transform_matrix * as_vec4(vector, 0.0);
        vec3(v.x, v.y, v.z)
    }
}

pub fn transform(position: Vec3, rotation: Vec3, scale_factor: Vec3) -> Transform
{
    let mut xform = Transform::new();
    xform.scale(scale_factor);
    xform.rotate(rotation);
    xform.translate(position);
    xform
}

pub fn from_position(position: Vec3) -> Transform {
    let mut xform = Transform::new();
    xform.translate(position);
    xform
}