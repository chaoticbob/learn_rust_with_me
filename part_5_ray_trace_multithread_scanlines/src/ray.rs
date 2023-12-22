#![allow(dead_code)]

use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub pos: Vec3,
    pub dir: Vec3,
}
