#![allow(non_snake_case)]

use crate::{quat, vec3};
use crate::vec3::Vec3;
use crate::vec4;
use crate::sphere::Sphere;

pub fn generate_sphere_flake(
    level             : u32,
    maxLevels         : u32,
    childRadius       : f32,
    parentRadius      : f32,
    parentCenter      : Vec3,
    parentOrientation : Vec3,
    spheres           : &mut Vec::<Sphere>
)
{
    if (level >= maxLevels) {
        return;
    }

    let kSphereFlakeVectors = [
        vec3( 0.408248290,  0.408248290, 0.816496581),
        vec3( 0.965925826,  0.258819045, 0.000000000),
        vec3( 0.258819045,  0.965925826, 0.000000000),
        vec3(-0.557677536,  0.149429245, 0.816496581),
        vec3(-0.707106781,  0.707106781, 0.000000000),
        vec3(-0.965925826, -0.258819045, 0.000000000),
        vec3( 0.149429245, -0.557677536, 0.816496581),
        vec3(-0.258819045, -0.965925826, 0.000000000),
        vec3( 0.707106781, -0.707106781, 0.000000000),
    ];

    let kSphereOrienation = vec3(0.0, 0.0, 1.0);

    let rotQuat = quat::rotation(kSphereOrienation, parentOrientation);
    let rotMat  = quat::to_mat4(rotQuat);

    let dist = parentRadius + childRadius;
    for i in 0..9 {
        let mut dir = vec3::normalize(kSphereFlakeVectors[i]);
        dir         = (rotMat * vec4::as_vec4(dir, 0.0)).as_vec3();
        let offset  = parentCenter + (dist * dir);

        spheres.push(Sphere{pos: offset, radius: childRadius, color: vec3(0.7, 0.7, 0.85)});

        let center = offset;
        generate_sphere_flake(level + 1, maxLevels, childRadius / 3.0, childRadius, center, dir, spheres);
    }
}
