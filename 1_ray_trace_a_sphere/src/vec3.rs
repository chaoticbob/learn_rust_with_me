#![allow(dead_code)]

use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x : f32,
    pub y : f32,
    pub z : f32,
}

// -Vec3
impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3{
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

// Vec3 + Vec3
impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3{
            x: (self.x + rhs.x),
            y: (self.y + rhs.y),
            z: (self.z + rhs.z)
        }
    }
}

// Vec3 - Vec3
impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3{
            x: (self.x - rhs.x),
            y: (self.y - rhs.y),
            z: (self.z - rhs.z)
        }
    }
}

// Vec3 * Vec3
impl ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3{
            x: (self.x * rhs.x),
            y: (self.y * rhs.y),
            z: (self.z * rhs.z)
        }
    }
}

// Vec3 * f32
impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Vec3 {
        Vec3{
            x: (self.x * rhs),
            y: (self.y * rhs),
            z: (self.z * rhs)
        }
    }
}

// f32 * Vec3
impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3{
            x: (self * rhs.x),
            y: (self * rhs.y),
            z: (self * rhs.z)
        }
    }
}

// Vec3 / f32
impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Vec3 {
        Vec3{
            x: (self.x / rhs),
            y: (self.y / rhs),
            z: (self.z / rhs),
        }
    }
}

pub fn vec3(x: f32, y: f32, z:f32) -> Vec3
{
    return Vec3{x, y, z};
}

pub fn dot(a: Vec3, b: Vec3) -> f32 {
    let x = a.x * b.x;
    let y = a.y * b.y;
    let z = a.z * b.z;
    let s = x + y + z;
    s
}

pub fn length(v: Vec3) -> f32 {
    dot(v, v).sqrt()
}

pub fn normalize(v: Vec3) -> Vec3 {
    let s = length(v);
    v / s
}

pub fn reflect(i: Vec3, n: Vec3) -> Vec3 {
    i - (2.0 * n * dot(i, n))
}