#![allow(dead_code)]
#![allow(non_snake_case)]

use crate::{mat4, vec3};
use crate::vec3::*;
use crate::vec4;
use crate::vec4::*;
use std::ops;

pub enum RotationOrder {
    XYZ,
    XZY,
    YZX,
    YXZ,
    ZXY,
    ZYX,
}

#[derive(Debug, Copy, Clone)]
pub struct Mat4 {
    pub value: [Vec4; 4], // Column vectors
}

impl Mat4 {
    pub fn row(&self, i: usize) -> Vec4 {
        match i {
            0 => vec4(self[0][0], self[1][0], self[2][0], self[3][0]),
            1 => vec4(self[0][1], self[1][1], self[2][1], self[3][1]),
            2 => vec4(self[0][2], self[1][2], self[2][2], self[3][2]),
            3 => vec4(self[0][3], self[1][3], self[2][3], self[3][3]),
            _ => panic!(),
        }
    }
}

pub fn mat4(col0: Vec4, col1: Vec4, col2: Vec4, col3: Vec4) -> Mat4 {
    Mat4 {
        value: [col0, col1, col2, col3],
    }
}

pub fn as_mat4(
    x0: f32,
    y0: f32,
    z0: f32,
    w0: f32, // value[0]
    x1: f32,
    y1: f32,
    z1: f32,
    w1: f32, // value[1]
    x2: f32,
    y2: f32,
    z2: f32,
    w2: f32, // value[2]
    x3: f32,
    y3: f32,
    z3: f32,
    w3: f32, // value[3]
) -> Mat4 {
    Mat4 {
        value: [
            vec4(x0, y0, z0, w0),
            vec4(x1, y1, z1, w1),
            vec4(x2, y2, z2, w2),
            vec4(x3, y3, z3, w3),
        ],
    }
}

impl ops::Index<usize> for Mat4 {
    type Output = Vec4;
    fn index(&self, i: usize) -> &Vec4 {
        &self.value[i]
    }
}

impl ops::IndexMut<usize> for Mat4 {
    fn index_mut(&mut self, i: usize) -> &mut Vec4 {
        &mut self.value[i]
    }
}

// -Mat4
impl ops::Neg for Mat4 {
    type Output = Mat4;

    fn neg(self) -> Mat4 {
        Mat4 {
            value: [
                -self.value[0],
                -self.value[1],
                -self.value[2],
                -self.value[3],
            ],
        }
    }
}

// Mat4 + Mat4
impl ops::Add for Mat4 {
    type Output = Mat4;

    fn add(self, rhs: Mat4) -> Mat4 {
        Mat4 {
            value: [
                self.value[0] + rhs.value[0],
                self.value[1] + rhs.value[1],
                self.value[2] + rhs.value[2],
                self.value[3] + rhs.value[3],
            ],
        }
    }
}

// Mat4 - Mat4
impl ops::Sub for Mat4 {
    type Output = Mat4;

    fn sub(self, rhs: Mat4) -> Mat4 {
        Mat4 {
            value: [
                self.value[0] - rhs.value[0],
                self.value[1] - rhs.value[1],
                self.value[2] - rhs.value[2],
                self.value[3] - rhs.value[3],
            ],
        }
    }
}

// Mat4 * Mat4
impl ops::Mul for Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: Mat4) -> Mat4 {
        let row0 = vec4(self[0].x, self[1].x, self[2].x, self[3].x);
        let row1 = vec4(self[0].y, self[1].y, self[2].y, self[3].y);
        let row2 = vec4(self[0].z, self[1].z, self[2].z, self[3].z);
        let row3 = vec4(self[0].w, self[1].w, self[2].w, self[3].w);

        let col0 = vec4(
            vec4::dot(row0, rhs[0]),
            vec4::dot(row1, rhs[0]),
            vec4::dot(row2, rhs[0]),
            vec4::dot(row3, rhs[0]),
        );
        let col1 = vec4(
            vec4::dot(row0, rhs[1]),
            vec4::dot(row1, rhs[1]),
            vec4::dot(row2, rhs[1]),
            vec4::dot(row3, rhs[1]),
        );
        let col2 = vec4(
            vec4::dot(row0, rhs[2]),
            vec4::dot(row1, rhs[2]),
            vec4::dot(row2, rhs[2]),
            vec4::dot(row3, rhs[2]),
        );
        let col3 = vec4(
            vec4::dot(row0, rhs[3]),
            vec4::dot(row1, rhs[3]),
            vec4::dot(row2, rhs[3]),
            vec4::dot(row3, rhs[3]),
        );

        Mat4 {
            value: [col0, col1, col2, col3],
        }
    }
}

// Mat4 * Vec4
impl ops::Mul<Vec4> for Mat4 {
    type Output = Vec4;

    fn mul(self, v: Vec4) -> Vec4 {
        vec4(
            vec4::dot(self.row(0), v),
            vec4::dot(self.row(1), v),
            vec4::dot(self.row(2), v),
            vec4::dot(self.row(3), v),
        )
    }
}

// Mat4 * f32
impl ops::Mul<f32> for Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: f32) -> Mat4 {
        Mat4 {
            value: [
                self.value[0] * rhs,
                self.value[1] * rhs,
                self.value[2] * rhs,
                self.value[3] * rhs,
            ],
        }
    }
}

// f32 * Nat4
impl ops::Mul<Mat4> for f32 {
    type Output = Mat4;

    fn mul(self, rhs: Mat4) -> Mat4 {
        Mat4 {
            value: [
                self * rhs.value[0],
                self * rhs.value[1],
                self * rhs.value[2],
                self * rhs.value[3],
            ],
        }
    }
}

pub fn identity() -> Mat4 {
    Mat4 {
        value: [
            vec4(1.0, 0.0, 0.0, 0.0),
            vec4(0.0, 1.0, 0.0, 0.0),
            vec4(0.0, 0.0, 1.0, 0.0),
            vec4(0.0, 0.0, 0.0, 1.0),
        ],
    }
}

pub fn zero() -> Mat4 {
    Mat4 {
        value: [
            vec4(0.0, 0.0, 0.0, 0.0),
            vec4(0.0, 0.0, 0.0, 0.0),
            vec4(0.0, 0.0, 0.0, 0.0),
            vec4(0.0, 0.0, 0.0, 0.0),
        ],
    }
}

pub fn translate(position: Vec3) -> Mat4 {
    let tx = position.x;
    let ty = position.y;
    let tz = position.z;
    Mat4 {
        value: [
            vec4(1.0, 0.0, 0.0, 0.0),
            vec4(0.0, 1.0, 0.0, 0.0),
            vec4(0.0, 0.0, 1.0, 0.0),
            vec4( tx,  ty,  tz, 1.0),
        ],
    }
}

pub fn euler_angle_x(angle: f32) -> Mat4 {
    let cs = angle.cos();
    let sn = angle.sin();
    Mat4 {
        value: [
            vec4(1.0, 0.0, 0.0, 0.0),
            vec4(0.0,  cs,  sn, 0.0),
            vec4(0.0, -sn,  cs, 0.0),
            vec4(0.0, 0.0, 0.0, 1.0),
        ],
    }
}

pub fn euler_angle_y(angle: f32) -> Mat4 {
    let cs = angle.cos();
    let sn = angle.sin();
    Mat4 {
        value: [
            vec4( cs, 0.0, -sn, 0.0),
            vec4(0.0, 1.0, 0.0, 0.0),
            vec4( sn, 0.0,  cs, 0.0),
            vec4(0.0, 0.0, 0.0, 1.0),
        ],
    }
}

pub fn euler_angle_z(angle: f32) -> Mat4 {
    let cs = angle.cos();
    let sn = angle.sin();
    Mat4 {
        value: [
            vec4( cs,  sn, 0.0, 0.0),
            vec4(-sn,  cs, 0.0, 0.0),
            vec4(0.0, 0.0, 1.0, 0.0),
            vec4(0.0, 0.0, 0.0, 1.0),
        ],
    }
}

pub fn rotate(euler_angles: Vec3, rotation_order: RotationOrder) -> Mat4 {
    let mx = euler_angle_x(euler_angles.x);
    let my = euler_angle_y(euler_angles.y);
    let mz = euler_angle_z(euler_angles.z);
    //let mx = rotate_axis_angle(euler_angles.x, vec3::X_AXIS);
    //let my = rotate_axis_angle(euler_angles.y, vec3::Y_AXIS);
    //let mz = rotate_axis_angle(euler_angles.z, vec3::Z_AXIS);
    match rotation_order {
        RotationOrder::XYZ => (mx * my * mz),
        RotationOrder::XZY => (mx * mz * my),
        RotationOrder::YZX => (my * mz * mx),
        RotationOrder::YXZ => (my * mx * mz),
        RotationOrder::ZXY => (mz * mx * my),
        RotationOrder::ZYX => (mz * my * mx),
    }
}

// GLM: rotate
pub fn rotate_axis_angle(angle: f32, v: Vec3) -> Mat4 {
    let a = angle;
    let c = a.cos();
    let s = a.sin();

    let axis = normalize(v);
    let temp = (1.0 - c) * axis;

    let mut Result = identity();
    Result[0][0] = c + temp[0] * axis[0];
    Result[0][1] = temp[0] * axis[1] + s * axis[2];
    Result[0][2] = temp[0] * axis[2] - s * axis[1];

    Result[1][0] = temp[1] * axis[0] - s * axis[2];
    Result[1][1] = c + temp[1] * axis[1];
    Result[1][2] = temp[1] * axis[2] + s * axis[0];

    Result[2][0] = temp[2] * axis[0] + s * axis[1];
    Result[2][1] = temp[2] * axis[1] - s * axis[0];
    Result[2][2] = c + temp[2] * axis[2];

    Result
}

pub fn scale(scale_factor: Vec3) -> Mat4 {
    let sx = scale_factor.x;
    let sy = scale_factor.y;
    let sz = scale_factor.z;
    Mat4 {
        value: [
            vec4( sx, 0.0, 0.0, 0.0),
            vec4(0.0,  sy, 0.0, 0.0),
            vec4(0.0, 0.0,  sz, 0.0),
            vec4(0.0, 0.0, 0.0, 1.0),
        ],
    }
}

// GLM: inverse
pub fn inverse(m: Mat4) -> Mat4 {
    let Coef00 = m[2][2] * m[3][3] - m[3][2] * m[2][3];
    let Coef02 = m[1][2] * m[3][3] - m[3][2] * m[1][3];
    let Coef03 = m[1][2] * m[2][3] - m[2][2] * m[1][3];

    let Coef04 = m[2][1] * m[3][3] - m[3][1] * m[2][3];
    let Coef06 = m[1][1] * m[3][3] - m[3][1] * m[1][3];
    let Coef07 = m[1][1] * m[2][3] - m[2][1] * m[1][3];

    let Coef08 = m[2][1] * m[3][2] - m[3][1] * m[2][2];
    let Coef10 = m[1][1] * m[3][2] - m[3][1] * m[1][2];
    let Coef11 = m[1][1] * m[2][2] - m[2][1] * m[1][2];

    let Coef12 = m[2][0] * m[3][3] - m[3][0] * m[2][3];
    let Coef14 = m[1][0] * m[3][3] - m[3][0] * m[1][3];
    let Coef15 = m[1][0] * m[2][3] - m[2][0] * m[1][3];

    let Coef16 = m[2][0] * m[3][2] - m[3][0] * m[2][2];
    let Coef18 = m[1][0] * m[3][2] - m[3][0] * m[1][2];
    let Coef19 = m[1][0] * m[2][2] - m[2][0] * m[1][2];

    let Coef20 = m[2][0] * m[3][1] - m[3][0] * m[2][1];
    let Coef22 = m[1][0] * m[3][1] - m[3][0] * m[1][1];
    let Coef23 = m[1][0] * m[2][1] - m[2][0] * m[1][1];

    let Fac0 = vec4(Coef00, Coef00, Coef02, Coef03);
    let Fac1 = vec4(Coef04, Coef04, Coef06, Coef07);
    let Fac2 = vec4(Coef08, Coef08, Coef10, Coef11);
    let Fac3 = vec4(Coef12, Coef12, Coef14, Coef15);
    let Fac4 = vec4(Coef16, Coef16, Coef18, Coef19);
    let Fac5 = vec4(Coef20, Coef20, Coef22, Coef23);

    let Vec0 = vec4(m[1][0], m[0][0], m[0][0], m[0][0]);
    let Vec1 = vec4(m[1][1], m[0][1], m[0][1], m[0][1]);
    let Vec2 = vec4(m[1][2], m[0][2], m[0][2], m[0][2]);
    let Vec3 = vec4(m[1][3], m[0][3], m[0][3], m[0][3]);

    let Inv0 = (Vec1 * Fac0 - Vec2 * Fac1 + Vec3 * Fac2);
    let Inv1 = (Vec0 * Fac0 - Vec2 * Fac3 + Vec3 * Fac4);
    let Inv2 = (Vec0 * Fac1 - Vec1 * Fac3 + Vec3 * Fac5);
    let Inv3 = (Vec0 * Fac2 - Vec1 * Fac4 + Vec2 * Fac5);

    let SignA = vec4(1.0, -1.0, 1.0, -1.0);
    let SignB = vec4(-1.0, 1.0, -1.0, 1.0);
    let Inverse = mat4(Inv0 * SignA, Inv1 * SignB, Inv2 * SignA, Inv3 * SignB);

    let Row0 = vec4(Inverse[0][0], Inverse[1][0], Inverse[2][0], Inverse[3][0]);

    let Dot0 = m[0] * Row0;
    let Dot1 = (Dot0.x + Dot0.y) + (Dot0.z + Dot0.w);

    let OneOverDeterminant = 1.0 / Dot1;

    return Inverse * OneOverDeterminant;
}

// GLM: lookAtRH
pub fn look_at_RH(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    let f = normalize(center - eye);
    let s = normalize(cross(f, up));
    let u = cross(s, f);

    let mut Result = identity();
    Result[0][0] = s.x;
    Result[1][0] = s.y;
    Result[2][0] = s.z;
    Result[0][1] = u.x;
    Result[1][1] = u.y;
    Result[2][1] = u.z;
    Result[0][2] = -f.x;
    Result[1][2] = -f.y;
    Result[2][2] = -f.z;
    Result[3][0] = -vec3::dot(s, eye);
    Result[3][1] = -vec3::dot(u, eye);
    Result[3][2] = vec3::dot(f, eye);

    Result
}

// GLM: lookAtLH
pub fn look_at_LH(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    let f = normalize(center - eye);
    let s = normalize(cross(up, f));
    let u = cross(f, s);

    let mut Result = identity();
    Result[0][0] = s.x;
    Result[1][0] = s.y;
    Result[2][0] = s.z;
    Result[0][1] = u.x;
    Result[1][1] = u.y;
    Result[2][1] = u.z;
    Result[0][2] = f.x;
    Result[1][2] = f.y;
    Result[2][2] = f.z;
    Result[3][0] = -vec3::dot(s, eye);
    Result[3][1] = -vec3::dot(u, eye);
    Result[3][2] = -vec3::dot(f, eye);

    Result
}

// GLM: perspectiveRH_ZO
pub fn perspective_RH(fovy: f32, aspect: f32, zNear: f32, zFar: f32) -> Mat4 {
    let tanHalfFovy = (fovy / 2.0).tan();

    let mut Result = zero();
    Result[0][0] = 1.0 / (aspect * tanHalfFovy);
    Result[1][1] = 1.0 / (tanHalfFovy);
    Result[2][2] = zFar / (zNear - zFar);
    Result[2][3] = -1.0;
    Result[3][2] = -(zFar * zNear) / (zFar - zNear);

    Result
}

// GLM: perspectiveLH_ZO
pub fn perspective_LH(fovy: f32, aspect: f32, zNear: f32, zFar: f32) -> Mat4 {
    let tanHalfFovy = (fovy / 2.0).tan();

    let mut Result = zero();
    Result[0][0] = 1.0 / (aspect * tanHalfFovy);
    Result[1][1] = 1.0 / (tanHalfFovy);
    Result[2][2] = zFar / (zFar - zNear);
    Result[2][3] = 1.0;
    Result[3][2] = -(zFar * zNear) / (zFar - zNear);

    Result
}
