#![allow(dead_code)]

use std::ops;
use crate::vec2::Vec2;

#[derive(Debug, Copy, Clone, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub static ONE:    Vec3 = Vec3{x: 1.0, y: 1.0, z: 1.0};
pub static ZERO:   Vec3 = Vec3{x: 0.0, y: 0.0, z: 0.0};

pub static X_AXIS: Vec3 = Vec3{x: 1.0, y: 0.0, z: 0.0};
pub static Y_AXIS: Vec3 = Vec3{x: 0.0, y: 1.0, z: 0.0};
pub static Z_AXIS: Vec3 = Vec3{x: 0.0, y: 0.0, z: 1.0};

impl Vec3 {
    pub fn xy(&self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.y
        }
    }

    pub fn xyz(&self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    pub fn yzx(&self) -> Vec3 {
        Vec3 {
            x: self.y,
            y: self.z,
            z: self.x,
        }
    }
}

pub fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3 { x, y, z }
}

pub fn as_vec3(v: Vec2, z: f32) -> Vec3 {
    vec3(v.x, v.y, z)
}

pub fn from_scalar(s : f32) -> Vec3 {
    vec3(s, s, s)
}

impl ops::Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, i: usize) -> &f32 {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!(),
        }
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f32 {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!(),
        }
    }
}

// -Vec3
impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

// Vec3 + Vec3
impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: (self.x + rhs.x),
            y: (self.y + rhs.y),
            z: (self.z + rhs.z),
        }
    }
}

//  Vec3 += Vec3
impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

// Vec3 - Vec3
impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

// Vec3 - f32
impl ops::Sub<f32> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: f32) -> Vec3 {
        Vec3 {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

// f32 - Vec3
impl ops::Sub<Vec3> for f32 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self - rhs.x,
            y: self - rhs.y,
            z: self - rhs.z,
        }
    }
}

// Vec3 * Vec3
impl ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: (self.x * rhs.x),
            y: (self.y * rhs.y),
            z: (self.z * rhs.z),
        }
    }
}

// Vec3 *= Vec3
impl ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

// Vec3 * f32
impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Vec3 {
        Vec3 {
            x: (self.x * rhs),
            y: (self.y * rhs),
            z: (self.z * rhs),
        }
    }
}

// f32 * Vec3
impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: (self * rhs.x),
            y: (self * rhs.y),
            z: (self * rhs.z),
        }
    }
}

// Vec3 / Vec3
impl ops::Div for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: (self.x / rhs.x),
            y: (self.y / rhs.y),
            z: (self.z / rhs.z),
        }
    }
}

// Vec3 / f32
impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Vec3 {
        Vec3 {
            x: (self.x / rhs),
            y: (self.y / rhs),
            z: (self.z / rhs),
        }
    }
}

impl ops::Div<Vec3> for f32 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: (self / rhs.x),
            y: (self / rhs.y),
            z: (self / rhs.z),
        }
    }
}

pub fn dot(a: Vec3, b: Vec3) -> f32 {
    let v = a * b;
    let s = v.x + v.y + v.z;
    s
}

pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
    Vec3 {
        x: (a.y * b.z) - (b.y * a.z),
        y: (a.z * b.x) - (b.z * a.x),
        z: (a.x * b.y) - (b.x * a.y),
    }
}

pub fn length(v: Vec3) -> f32 {
    dot(v, v).sqrt()
}

pub fn length2(v: Vec3) -> f32 {
    dot(v, v)
}

pub fn normalize(v: Vec3) -> Vec3 {
    let s = length(v);
    v / s
}

pub fn reflect(i: Vec3, n: Vec3) -> Vec3 {
    i - (2.0 * n * dot(i, n))
}

pub fn min(a: Vec3, b: Vec3) -> Vec3
{
    Vec3 {
        x: a.x.min(b.x),
        y: a.y.min(b.y),
        z: a.z.min(b.z),
    }
}

pub fn max(a: Vec3, b: Vec3) -> Vec3
{
    Vec3 {
        x: a.x.max(b.x),
        y: a.y.max(b.y),
        z: a.z.max(b.z),
    }
}

pub fn abs(v: Vec3) -> Vec3
{
    Vec3 {
        x: v.x.abs(),
        y: v.y.abs(),
        z: v.z.abs(),
    }
}

pub fn sign(v: Vec3) -> Vec3 {
    Vec3 {
        x: v.x.signum(),
        y: v.y.signum(),
        z: v.z.signum(),
    }
}

pub fn mix(x: Vec3, y: Vec3, a: f32) -> Vec3 {
    (x * (1.0 - a)) + (y * a)
}

pub fn step(edge: Vec3, x: Vec3) -> Vec3 {
    Vec3 {
        x : if (x.x < edge.x) { 0.0 } else { 1.0 },
        y : if (x.y < edge.y) { 0.0 } else { 1.0 },
        z : if (x.z < edge.z) { 0.0 } else { 1.0 },
    }
}