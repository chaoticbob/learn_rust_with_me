#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::mat4;
use crate::mat4::Mat4;
use crate::vec3::*;

static EPSILON: f32 = 1.19209e-07;

pub struct Quat {
    w: f32,
    x: f32,
    y: f32,
    z: f32,
}

pub fn quat(w: f32, x: f32, y: f32, z: f32) -> Quat {
    Quat { w, x, y, z }
}

pub fn as_quat(w: f32, v: Vec3) -> Quat {
    Quat {
        w: w,
        x: v.x,
        y: v.y,
        z: v.z,
    }
}

pub fn identity() -> Quat {
    quat(1.0, 0.0, 0.0, 0.0)
}

pub fn axis_angle(angle: f32, v: Vec3) -> Quat {
    let a = angle;
    let s = a.sin() * 0.5;

    as_quat(a.cos() * 0.5, v * s)
}

// GLM's quat rotation
pub fn rotation(orig: Vec3, dest: Vec3) -> Quat {
    let cosTheta = dot(orig, dest);

    if (cosTheta >= (1.0 - EPSILON)) {
        // orig and dest point in the same direction

        return identity();
    }

    if (cosTheta < (-1.0 + EPSILON)) {
        // special case when vectors in opposite directions :
        // there is no "ideal" rotation axis
        // So guess one; any will do as long as it's perpendicular to start
        // This implementation favors a rotation around the Up axis (Y),
        // since it's often what you want to do.
        let mut rotationAxis = cross(vec3(0.0, 0.0, 1.0), orig);
        if (length2(rotationAxis) < EPSILON) {
            // bad luck, they were parallel, try again!
            rotationAxis = cross(vec3(1.0, 0.0, 0.0), orig);
        }

        rotationAxis = normalize(rotationAxis);

        return axis_angle(std::f32::consts::PI, rotationAxis);
    }

    // Implementation from Stan Melax's Game Programming Gems 1 article
    let rotationAxis = cross(orig, dest);

    let s = ((1.0 + cosTheta) * 2.0).sqrt();
    let invs = 1.0 / s;

    return quat(
        s * 0.5,
        rotationAxis.x * invs,
        rotationAxis.y * invs,
        rotationAxis.z * invs,
    );
}

// GLM's quat to mat4 cast
pub fn to_mat4(q: Quat) -> Mat4 {
    let qxx = (q.x * q.x);
    let qyy = (q.y * q.y);
    let qzz = (q.z * q.z);
    let qxz = (q.x * q.z);
    let qxy = (q.x * q.y);
    let qyz = (q.y * q.z);
    let qwx = (q.w * q.x);
    let qwy = (q.w * q.y);
    let qwz = (q.w * q.z);

    let mut Result = mat4::identity();
    Result[0][0] = 1.0 - 2.0 * (qyy + qzz);
    Result[0][1] = 2.0 * (qxy + qwz);
    Result[0][2] = 2.0 * (qxz - qwy);

    Result[1][0] = 2.0 * (qxy - qwz);
    Result[1][1] = 1.0 - 2.0 * (qxx + qzz);
    Result[1][2] = 2.0 * (qyz + qwx);

    Result[2][0] = 2.0 * (qxz + qwy);
    Result[2][1] = 2.0 * (qyz - qwx);
    Result[2][2] = 1.0 - 2.0 * (qxx + qyy);

    Result
}
