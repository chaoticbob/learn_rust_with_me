#![allow(dead_code)]

use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

pub static X_AXIS: Vec2 = Vec2{x: 1.0, y: 0.0};
pub static Y_AXIS: Vec2 = Vec2{x: 0.0, y: 1.0};

pub fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2 { x, y }
}

// -Vec2
impl ops::Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Vec2 {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

// Vec2 + Vec2
impl ops::Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: (self.x + rhs.x),
            y: (self.y + rhs.y),
        }
    }
}

// Vec2 + f32
impl ops::Add<f32> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: f32) -> Vec2 {
        vec2(self.x + rhs, self.y + rhs)
    }
}

// Vec2 - Vec2
impl ops::Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: (self.x - rhs.x),
            y: (self.y - rhs.y),
        }
    }
}

// Vec2 - f32
impl ops::Sub<f32> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: f32) -> Vec2 {
        vec2(self.x - rhs, self.y - rhs)
    }
}

// Vec2 * Vec2
impl ops::Mul for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: (self.x * rhs.x),
            y: (self.y * rhs.y),
        }
    }
}

// Vec2 * f32
impl ops::Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f32) -> Vec2 {
        Vec2 {
            x: (self.x * rhs),
            y: (self.y * rhs),
        }
    }
}

// f32 * Vec2
impl ops::Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: (self * rhs.x),
            y: (self * rhs.y),
        }
    }
}

// Vec2 / Vec2
impl ops::Div for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: (self.x / rhs.x),
            y: (self.y / rhs.y),
        }
    }
}

// Vec2 / f32
impl ops::Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f32) -> Vec2 {
        Vec2 {
            x: (self.x / rhs),
            y: (self.y / rhs),
        }
    }
}

pub fn dot(a: Vec2, b: Vec2) -> f32 {
    let v = a * b;
    let s = v.x + v.y;
    s
}

pub fn length(v: Vec2) -> f32 {
    dot(v, v).sqrt()
}

pub fn length2(v: Vec2) -> f32 {
    dot(v, v)
}

pub fn normalize(v: Vec2) -> Vec2 {
    let s = length(v);
    v / s
}

pub fn reflect(i: Vec2, n: Vec2) -> Vec2 {
    i - (2.0 * n * dot(i, n))
}
