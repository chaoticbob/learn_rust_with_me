#![allow(dead_code)]

use std::ops;
use crate::vec3::*;


#[derive(Debug, Copy, Clone)]
pub struct Vec4 {
    pub x : f32,
    pub y : f32,
    pub z : f32,
    pub w : f32,
}

impl Vec4 {
    pub fn as_vec3(&self) -> Vec3 {
        vec3(self.x, self.y, self.z)
    }
}

pub fn vec4(x: f32, y: f32, z: f32, w: f32) -> Vec4
{
    Vec4{x, y, z, w}
}

pub fn as_vec4(v: Vec3, w: f32) -> Vec4
{
    vec4(v.x, v.y, v.z, w)
}

impl ops::Index<usize> for Vec4 {
    type Output = f32;
    fn index(&self, i: usize) -> &f32 {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!(),
        }
    }
}

impl ops::IndexMut<usize> for Vec4 {
    fn index_mut(&mut self, i: usize) -> &mut f32 {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!(),
        }
    }
}

// -Vec4
impl ops::Neg for Vec4 {
    type Output = Vec4;

    fn neg(self) -> Vec4 {
        Vec4{
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

// Vec4 + Vec4
impl ops::Add for Vec4 {
    type Output = Vec4;

    fn add(self, rhs: Vec4) -> Vec4 {
        Vec4{
            x: (self.x + rhs.x),
            y: (self.y + rhs.y),
            z: (self.z + rhs.z),
            w: (self.w + rhs.w),
        }
    }
}

// Vec4 - Vec4
impl ops::Sub for Vec4 {
    type Output = Vec4;

    fn sub(self, rhs: Vec4) -> Vec4 {
        Vec4{
            x: (self.x - rhs.x),
            y: (self.y - rhs.y),
            z: (self.z - rhs.z),
            w: (self.w - rhs.w),
        }
    }
}

// Vec4 * Vec4
impl ops::Mul for Vec4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Vec4 {
        Vec4{
            x: (self.x * rhs.x),
            y: (self.y * rhs.y),
            z: (self.z * rhs.z),
            w: (self.w * rhs.w),
        }
    }
}

// Vec4 * f32
impl ops::Mul<f32> for Vec4 {
    type Output = Vec4;

    fn mul(self, rhs: f32) -> Vec4 {
        Vec4{
            x: (self.x * rhs),
            y: (self.y * rhs),
            z: (self.z * rhs),
            w: (self.w * rhs),
        }
    }
}

// f32 * Vec4
impl ops::Mul<Vec4> for f32 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Vec4 {
        Vec4{
            x: (self * rhs.x),
            y: (self * rhs.y),
            z: (self * rhs.z),
            w: (self * rhs.w),
        }
    }
}

pub fn dot(a: Vec4, b: Vec4) -> f32
{
    let v = a * b;
    let s = v.x + v.y + v.z + v.w;
    s
}
