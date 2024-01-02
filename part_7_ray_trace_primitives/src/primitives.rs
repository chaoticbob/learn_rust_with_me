#![allow(dead_code)]
#![allow(unused_parens)]
#![allow(non_snake_case)]

use crate::ray::Ray;
use crate::transform::Transform;
use crate::vec2::*;
use crate::{vec2, vec3};
use crate::vec3::*;

pub trait Primitive {
    fn intersect_illum(&self, ray: &Ray, out_t: &mut f32, out_P: &mut Vec3, out_N: &mut Vec3) -> bool;
    fn intersect_shadow(&self, ray: &Ray) -> bool;
    fn get_color(&self) -> &Vec3;
    fn get_transform(&self) -> &Transform;
}

// Utility functions to make porting easier
fn min(a: f32, b: f32) -> f32 {
    a.min(b)
}

fn max(a: f32, b:f32) -> f32 {
    a.max(b)
}

fn abs(value: f32) -> f32 {
    value.abs()
}

fn sqrt(value: f32) -> f32 {
    value.sqrt()
}

fn sign(value: f32) -> f32 {
    value.signum()
}

fn cos(value: f32) -> f32 {
    value.cos()
}

fn acos(value: f32) -> f32 {
    value.acos()
}

fn sin(value: f32) -> f32 {
    value.sin()
}

fn atan(value: f32) -> f32 {
    value.atan()
}

fn atan2(y: f32, x: f32) -> f32 {
    y.atan2(x)
}

fn pow(value: f32, y: f32) -> f32 {
    value.powf(y)
}

// =====================================================================================================================
// Sphere
// =====================================================================================================================
pub struct Sphere {
    pub transform : Transform,
    pub color: Vec3,
}

impl Sphere {
    fn get_normal(&self, P: Vec3) -> Vec3 {
        let pos = self.transform.world_to_local_point(P);
        let N = vec3::normalize(pos);
        self.transform.local_to_world_vector(N)
    }
}

impl Primitive for Sphere {
    fn intersect_illum(&self, ray: &Ray, out_t: &mut f32, out_P: &mut Vec3, out_N: &mut Vec3) -> bool {
        let local_ray = Ray {
            pos: self.transform.world_to_local_point(ray.pos),
            dir: self.transform.world_to_local_vector(ray.dir),
        };

        let radius = 1.0;
        let f  = local_ray.pos;
        let a  = vec3::dot(local_ray.dir, local_ray.dir);
        let bi = vec3::dot(-f, local_ray.dir);
        let c  = vec3::dot(f, f) - (radius * radius);
        let s  = f + (bi / a) * local_ray.dir;
        let discr = radius * radius - vec3::dot(s, s);

        let mut t1: f32 = -1.0;
        let mut t2: f32 = -1.0;
        let mut hit = false;
        if (discr >= 0.0) {
            let q = bi + bi.signum() * (a * discr).sqrt();
            t1 = c / q;
            t2 = q / a;
            hit = true;
        }

        let t = if (t1 < t2) { t1 } else { t2 };
        if (hit) {
            *out_t = t;
            *out_P = ray.pos + t * ray.dir;
            *out_N = self.get_normal(*out_P);
        }

        return hit;
    }

    fn intersect_shadow(&self, ray: &Ray) -> bool {
        let local_ray = Ray {
            pos: self.transform.world_to_local_point(ray.pos),
            dir: self.transform.world_to_local_vector(ray.dir),
        };

        let radius = 1.0;
        let f  = local_ray.pos;
        let a  = vec3::dot(local_ray.dir, local_ray.dir);
        let bi = vec3::dot(-f, local_ray.dir);
        let c  = vec3::dot(f, f) - (radius * radius);
        let s  = f + (bi / a) * local_ray.dir;
        let discr = radius * radius - vec3::dot(s, s);

        let mut t1: f32 = -1.0;
        let mut t2: f32 = -1.0;
        if (discr >= 0.0) {
            let q = bi + bi.signum() * (a * discr).sqrt();
            t1 = c / q;
            t2 = q / a;
        }

        let t = if (t1 < t2) { t1 } else { t2 };
        let hit = if (t > 0.0) { true } else { false };
        return hit;
    }

    fn get_color(&self) -> &Vec3 {
        &self.color
    }

    fn get_transform(&self) -> &Transform {
        &self.transform
    }
}

// =====================================================================================================================
// AABox
// https://iquilezles.org/articles/intersectors/
// =====================================================================================================================
pub struct AABox {
    pub transform: Transform,
    pub size: Vec3,
    pub color: Vec3,
}

impl Primitive for AABox {
    fn intersect_illum(&self, ray: &Ray, out_t: &mut f32, out_P: &mut Vec3, out_N: &mut Vec3) -> bool {
        let local_ray = Ray {
            pos: self.transform.world_to_local_point(ray.pos),
            dir: self.transform.world_to_local_vector(ray.dir),
        };

        let ro = local_ray.pos;
        let rd = local_ray.dir;

        let m = vec3::from_scalar(1.0) / rd; // can precompute if traversing a set of aligned boxes
        let n = m*ro;                        // can precompute if traversing a set of aligned boxes
        let k = vec3::abs(m)*self.size;
        let t1 = -n - k;
        let t2 = -n + k;
        let tN = max(max( t1.x, t1.y ), t1.z);
        let tF = min(min( t2.x, t2.y ), t2.z);

        // tN < tF must be true for valid intersection
        // tF < 0 means the box is behind us
        let hit = if (tN < tF) && (tF >= 0.0) { true } else { false };
        if (hit) {
            let outside = if (tN > 0.0) { true } else { false };
            let t = if outside { tN } else { tF };
            *out_t = t;
            *out_P = ray.pos + t * ray.dir;
            let mut N = if outside { vec3::step(vec3::from_scalar(tN), t1) } else { vec3::step(t2, vec3::from_scalar(tF)) };
            N *= -vec3::sign(rd);
            *out_N = self.transform.local_to_world_vector(N);
        }

        return hit;
    }

    fn intersect_shadow(&self, ray: &Ray) -> bool {
        let local_ray = Ray {
            pos: self.transform.world_to_local_point(ray.pos),
            dir: self.transform.world_to_local_vector(ray.dir),
        };

        let ro = local_ray.pos;
        let rd = local_ray.dir;

        let m = vec3::from_scalar(1.0) / rd; // can precompute if traversing a set of aligned boxes
        let n = m*ro;                        // can precompute if traversing a set of aligned boxes
        let k = vec3::abs(m)*self.size;
        let t1 = -n - k;
        let t2 = -n + k;
        let tN = max(max( t1.x, t1.y ), t1.z);
        let tF = min(min( t2.x, t2.y ), t2.z);

        // tN < tF must be true for valid intersection
        // tF < 0 means the box is behind us
        let hit = if (tN < tF) && (tF >= 0.0) { true } else { false };
        return hit;
    }

    fn get_color(&self) -> &Vec3 {
        &self.color
    }

    fn get_transform(&self) -> &Transform {
        &self.transform
    }
}

// =====================================================================================================================
// RoundedBox
// https://www.shadertoy.com/view/WlSXRW
// =====================================================================================================================
pub struct RoundedBox {
    pub transform: Transform,
    pub size: Vec3,
    pub radius: f32,
    pub color: Vec3,
}

impl RoundedBox {
    fn get_normal(&self, P: Vec3) -> Vec3 {
        let pos = self.transform.world_to_local_point(P);
        let N = vec3::sign(pos) * vec3::normalize(vec3::max(vec3::abs(pos) - self.size, vec3::from_scalar(0.0)));
        self.transform.local_to_world_vector(N)
    }
}

impl Primitive for crate::primitives::RoundedBox {
    fn intersect_illum(&self, ray: &Ray, out_t: &mut f32, out_P: &mut Vec3, out_N: &mut Vec3) -> bool {
        let local_ray = Ray {
            pos: self.transform.world_to_local_point(ray.pos),
            dir: self.transform.world_to_local_vector(ray.dir),
        };

        let mut ro = local_ray.pos;
        let mut rd = local_ray.dir;
        let size = self.size;
        let rad  = self.radius;
        let vrad = vec3::from_scalar(rad);

        // bounding box
        let m = 1.0 / rd;
        let n = m*ro;
        let k = vec3::abs(m)*(size + vrad);
        let t1 = -n - k;
        let t2 = -n + k;
        let tN = max( max( t1.x, t1.y ), t1.z );
        let tF = min( min( t2.x, t2.y ), t2.z );
        if (tN > tF) || (tF < 0.0) {
            return false;
        }

        let mut t = tN;

        // convert to first octant
        let mut pos = ro+ t*rd;
        let s = vec3::sign(pos);
        ro  *= s;
        rd  *= s;
        pos *= s;

        // faces
        pos -= size;
        pos = vec3::max(pos.xyz(), pos.yzx());
        if (min(min(pos.x, pos.y), pos.z) < 0.0) {
            let hit = if (t >= 0.0) { true } else { false };
            if (hit) {
                *out_t = t;
                *out_P = ray.pos + t * ray.dir;
                *out_N = self.get_normal(*out_P);
            }
            return hit;
        }

        // some precomputation
        let oc = ro - size;
        let dd = rd*rd;
        let oo = oc*oc;
        let od = oc*rd;
        let ra2 = rad*rad;

        t = 1e20;

        // corner
        {
            let b = od.x + od.y + od.z;
            let c = oo.x + oo.y + oo.z - ra2;
            let h = b*b - c;
            if (h > 0.0) {
                t = -b - sqrt(h);
            }
        }

        // edge X
        {
            let a = dd.y + dd.z;
            let b = od.y + od.z;
            let c = oo.y + oo.z - ra2;
            let mut h = b*b - a*c;
            if (h > 0.0) {
                h = (-b - sqrt(h))/a;
                if (h > 0.0) && (h < t) && (abs(ro.x + rd.x*h) < size.x) {
                    t = h;
                }
            }
        }
        // edge Y
        {
            let a = dd.z + dd.x;
            let b = od.z + od.x;
            let c = oo.z + oo.x - ra2;
            let mut h = b*b - a*c;
            if (h > 0.0){
                h = (-b - sqrt(h))/a;
                if (h > 0.0) && (h < t) && (abs(ro.y + rd.y*h) < size.y)  {
                    t = h;
                }
            }
        }
        // edge Z
        {
            let a = dd.x + dd.y;
            let b = od.x + od.y;
            let c = oo.x + oo.y - ra2;
            let mut h = b*b - a*c;
            if (h> 0.0){
                h = (-b - sqrt(h))/a;
                if (h > 0.0) && (h < t) && (abs(ro.z + rd.z*h) < size.z) {
                    t = h;
                }
            }
        }

        let hit = if (t > 0.0) && (t < 1e19) { true } else { false };
        if (hit) {
            *out_t = t;
            *out_P = ray.pos + t * ray.dir;
            *out_N = self.get_normal(*out_P);
        }

        return hit;
    }

    fn intersect_shadow(&self, ray: &Ray) -> bool {
        let mut t = 0.0;
        let mut P = vec3::ZERO;
        let mut N = vec3::ZERO;
        let hit = self.intersect_illum(ray, &mut t, &mut P, &mut N);
        return hit && (t > 0.0);
    }

    fn get_color(&self) -> &Vec3 {
        &self.color
    }

    fn get_transform(&self) -> &Transform {
        &self.transform
    }
}

// =====================================================================================================================
// Plane
// =====================================================================================================================
pub struct Plane {
    pub transform : Transform,
    pub color: Vec3,
}

impl Primitive for Plane {
    fn intersect_illum(&self, ray: &Ray, out_t: &mut f32, out_P: &mut Vec3, out_N: &mut Vec3) -> bool {
        let local_ray = Ray {
            pos: self.transform.world_to_local_point(ray.pos),
            dir: self.transform.world_to_local_vector(ray.dir),
        };

        let plane_dir = vec3(0.0, 1.0, 0.0);
        let t = -vec3::dot(local_ray.pos, plane_dir) / vec3::dot(local_ray.dir, plane_dir);

        let hit = if (t > 0.0) { true } else { false };
        if (hit) {
            *out_t = t;
            *out_P = ray.pos + t * ray.dir;
            *out_N = self.transform.local_to_world_vector(plane_dir);
        }

        return hit;
    }

    fn intersect_shadow(&self, ray: &Ray) -> bool {
        let local_ray = Ray {
            pos: self.transform.world_to_local_point(ray.pos),
            dir: self.transform.world_to_local_vector(ray.dir),
        };

        let plane_dir = vec3(0.0, 1.0, 0.0);
        let t = -vec3::dot(local_ray.pos, plane_dir) / vec3::dot(local_ray.dir, plane_dir);

        let hit = if (t > 0.0) { true } else { false };
        return hit;
    }

    fn get_color(&self) -> &Vec3 {
        &self.color
    }

    fn get_transform(&self) -> &Transform {
        &self.transform
    }
}

// =====================================================================================================================
// Cylinder
// https://www.shadertoy.com/view/tl23Rm
// =====================================================================================================================
pub struct Cylinder {
    pub transform: Transform,
    pub start: Vec3,
    pub end: Vec3,
    pub radius: f32,
    pub color : Vec3,
}

impl Primitive for Cylinder {
    fn intersect_illum(&self, ray: &Ray, out_t: &mut f32, out_P: &mut Vec3, out_N: &mut Vec3) -> bool {
        let local_ray = Ray {
            pos: self.transform.world_to_local_point(ray.pos),
            dir: self.transform.world_to_local_vector(ray.dir),
        };

        let ro = local_ray.pos;
        let rd = local_ray.dir;
        let ra = self.radius;

        let pa = self.start;
        let pb = self.end;

        let ca = pb-pa;
        let oc = ro-pa;

        let caca = vec3::dot(ca, ca);
        let card = vec3::dot(ca, rd);
        let caoc = vec3::dot(ca, oc);

        let a = caca - card*card;
        let b = caca*vec3::dot(oc, rd) - caoc*card;
        let c = caca*vec3::dot(oc, oc) - caoc*caoc - ra*ra*caca;
        let mut h = b*b - a*c;

        if (h < 0.0) {
            return false;
        }

        h = sqrt(h);
        let mut t = (-b-h)/a;

        let mut hit = false;
        let mut N = vec3::ZERO;

        let y = caoc + t*card;
        if (y > 0.0) && (y < caca) {
            hit = true;
            N = (oc+t*rd - ca*y/caca)/ra;
        }
        else {
            t = if (y < 0.0) { 0.0 } else { caca };
            t = (t - caoc) / card;

            if (abs(b + a * t) < h) {
                hit = true;
                N = vec3::normalize(ca * sign(y) / caca);
            }
        }

        if (hit) {
            *out_t = t;
            *out_P = ray.pos + t * ray.dir;
            *out_N = self.transform.local_to_world_vector(N);
        }

        return hit && (t > 0.0);
    }

    fn intersect_shadow(&self, ray: &Ray) -> bool {
        let mut t = 0.0;
        let mut P = vec3::ZERO;
        let mut N = vec3::ZERO;
        let hit = self.intersect_illum(ray, &mut t, &mut P, &mut N);
        return hit && (t > 0.0);
    }

    fn get_color(&self) -> &Vec3 {
        &self.color
    }

    fn get_transform(&self) -> &Transform {
        &self.transform
    }
}

// =====================================================================================================================
// Ellipsoid
// https://iquilezles.org/articles/intersectors/
// =====================================================================================================================
pub struct Ellipsoid {
    pub transform : Transform,
    pub radii: Vec3,
    pub color: Vec3,
}

impl Ellipsoid {
    fn get_normal(&self, P: Vec3) -> Vec3 {
        let pos = self.transform.world_to_local_point(P);
        let N = vec3::normalize( pos / (self.radii * self.radii) );
        self.transform.local_to_world_vector(N)
    }
}

impl Primitive for Ellipsoid {
    fn intersect_illum(&self, ray: &Ray, out_t : &mut f32, out_P: &mut Vec3, out_N: &mut Vec3)  -> bool {
        let local_ray = Ray {
            pos: self.transform.world_to_local_point(ray.pos),
            dir: self.transform.world_to_local_vector(ray.dir),
        };

        let ocn = local_ray.pos / self.radii;
        let rdn = local_ray.dir / self.radii;
        let a = vec3::dot( rdn, rdn );
        let b = vec3::dot( ocn, rdn );
        let c = vec3::dot( ocn, ocn );
        let h = b*b - a*(c-1.0);
        if( h < 0.0 ) {
            // no intersection
            return false;
        }

        let t = (-b - h.sqrt()) / a;
        let hit = if (t > 0.0) { true } else { false };
        if (hit) {
            *out_t = t;
            *out_P = ray.pos + t * ray.dir;
            *out_N = self.get_normal(*out_P);
        }

        return hit;
    }

    fn intersect_shadow(&self, ray: &Ray) -> bool {
        let local_ray = Ray {
            pos: self.transform.world_to_local_point(ray.pos),
            dir: self.transform.world_to_local_vector(ray.dir),
        };

        let ocn = local_ray.pos / self.radii;
        let rdn = local_ray.dir / self.radii;
        let a = vec3::dot( rdn, rdn );
        let b = vec3::dot( ocn, rdn );
        let c = vec3::dot( ocn, ocn );
        let h = b*b - a*(c-1.0);
        if( h < 0.0 ) {
            // no intersection
            return false;
        }

        let t = (-b - h.sqrt()) / a;
        let hit = if (t > 0.0) { true } else { false };
        return hit;
    }

    fn get_color(&self) -> &Vec3 {
        &self.color
    }

    fn get_transform(&self) -> &Transform {
        &self.transform
    }
}

// =====================================================================================================================
// Torus
// https://iquilezles.org/articles/intersectors/
// =====================================================================================================================
pub struct Torus {
    pub transform : Transform,
    pub major_radius : f32,
    pub minor_radius : f32,
    pub color: Vec3,
}

impl Torus {
    fn get_normal(&self, P : Vec3) -> Vec3 {
        let pos = self.transform.world_to_local_point(P);
        let Ra = self.major_radius;
        let ra = self.minor_radius;
        let Ra2 = Ra * Ra;
        let ra2 = ra * ra;
        let N = vec3::normalize(pos*(vec3::dot(pos, pos) - ra2 - Ra2*vec3(1.0, 1.0, -1.0)));
        self.transform.local_to_world_vector(N)
    }
}

impl Primitive for Torus {
    fn intersect_illum(&self, ray: &Ray, out_t: &mut f32, out_P: &mut Vec3, out_N: &mut Vec3)  -> bool {
        let local_ray = Ray {
            pos: self.transform.world_to_local_point(ray.pos),
            dir: vec3::normalize(self.transform.world_to_local_vector(ray.dir)),
        };

        let ro = local_ray.pos;
        let rd = local_ray.dir;

        let mut po = 1.0;

        let Ra = self.major_radius;
        let ra = self.minor_radius;

        let Ra2 = Ra*Ra;
        let ra2 = ra*ra;

        let m = vec3::dot(ro, ro);
        let n = vec3::dot(ro, rd);

        // bounding sphere
        {
            let h = n*n - m + (Ra + ra)*(Ra + ra);
            if (h < 0.0) {
                return false;
            }
            //let t = -n-sqrt(h); // could use this to compute intersections from ro+t*rd
        }

        // find quartic equation
        let k = (m - ra2 - Ra2)/2.0;
        let mut k3 = n;
        let mut k2 = n*n + Ra2*rd.z*rd.z + k;
        let mut k1 = k*n + Ra2*ro.z*rd.z;
        let mut k0 = k*k + Ra2*ro.z*ro.z - Ra2*ra2;

        // prevent |c1| from being too close to zero
        if (abs(k3*(k3*k3 - k2) + k1) < 1e-4) {
            po = -1.0;

            let tmp=k1;
            k1=k3;
            k3=tmp;

            k0 = 1.0/k0;
            k1 = k1*k0;
            k2 = k2*k0;
            k3 = k3*k0;
        }

        let mut c2 = 2.0*k2 - 3.0*k3*k3;
        let mut c1 = k3*(k3*k3 - k2) + k1;
        let mut c0 = k3*(k3*(-3.0*k3*k3 + 4.0*k2) - 8.0*k1) + 4.0*k0;

        c2 /= 3.0;
        c1 *= 2.0;
        c0 /= 3.0;

        let Q = c2*c2 + c0;
        let R = 3.0*c0*c2 - c2*c2*c2 - c1*c1;


        let mut h = R*R - Q*Q*Q;
        let mut z;
        if (h < 0.0) {
            // 4 intersections
            let sQ = sqrt(Q);
            z = 2.0*sQ*cos(acos(R/(sQ*Q))/3.0);
        }
        else {
            // 2 intersections
            let sQ = pow(sqrt(h) + abs(R), 1.0/3.0);
            z = sign(R)*abs(sQ + Q/sQ);
        }
        z = c2 - z;

        let mut d1 = z   - 3.0*c2;
        let mut d2 = z*z - 3.0*c0;
        if (abs(d1) < 1.0e-4) {
            if (d2 < 0.0) {
                return false;
            }
            d2 = sqrt(d2);
        }
        else
        {
            if (d1 < 0.0) {
                return false;
            }
            d1 = sqrt(d1/2.0);
            d2 = c1/d1;
        }

        //----------------------------------

        let mut t = 1e20;

        h = d1*d1 - z + d2;
        if (h > 0.0) {
            h = sqrt(h);
            let mut t1 = -d1 - h - k3;
            //t1 = (po<0.0)?2.0/t1:t1;
            t1 = if (po < 0.0) { 2.0/t1 } else { t1 };

            let mut t2 = -d1 + h - k3;
            //t2 = (po<0.0)?2.0/t2:t2;
            t2 = if (po < 0.0) { 2.0/t2 } else { t2 };

            if (t1 > 0.0) {
                t = t1;
            }

            if (t2 > 0.0) {
                t = min(t, t2);
            }
        }

        h = d1*d1 - z - d2;
        if (h > 0.0) {
            h = sqrt(h);
            let mut t1 = d1 - h - k3;
            //t1 = (po<0.0)?2.0/t1:t1;
            t1 = if (po < 0.0) { 2.0/t1 } else { t1 };

            let mut t2 = d1 + h - k3;
            //t2 = (po<0.0)?2.0/t2:t2;
            t2 = if (po < 0.0) { 2.0/t2 } else { t2 };

            if (t1 > 0.0) {
                t = min(t, t1);
            }

            if (t2 > 0.0) {
                t = min(t, t2);
            }
        }

        let hit = if (t > 0.0) && (t < 1e20) { true } else { false };
        if (hit) {
            *out_t = t;
            *out_P = ray.pos + t * ray.dir;
            *out_N = self.get_normal(*out_P);
        }

        return hit
    }

    fn intersect_shadow(&self, ray: &Ray) -> bool {
        let mut t = 0.0;
        let mut P = vec3::ZERO;
        let mut N = vec3::ZERO;
        let hit = self.intersect_illum(ray, &mut t, &mut P, &mut N);
        return hit;
    }

    fn get_color(&self) -> &Vec3 {
        &self.color
    }

    fn get_transform(&self) -> &Transform {
        &self.transform
    }
}

// =====================================================================================================================
// Goursat
// https://iquilezles.org/articles/intersectors/
// =====================================================================================================================
pub struct Goursat {
    pub transform : Transform,
    pub ka : f32,
    pub kb : f32,
    pub color: Vec3,
}

impl Goursat {
    fn get_normal(&self, P : Vec3) -> Vec3 {
        let pos = self.transform.world_to_local_point(P);
        let N = vec3::normalize((4.0*pos*pos*pos) - (2.0*pos*self.kb*self.kb));
        self.transform.local_to_world_vector(N)
    }
}

impl Primitive for Goursat {
    fn intersect_illum(&self, ray: &Ray, out_t: &mut f32, out_P: &mut Vec3, out_N: &mut Vec3)  -> bool {
        let local_ray = Ray {
            pos: self.transform.world_to_local_point(ray.pos),
            dir: self.transform.world_to_local_vector(ray.dir),
        };

        let ro = local_ray.pos;
        let rd = local_ray.dir;
        let ka = self.ka;
        let kb = self.kb;

        let mut po = 1.0;
        let rd2 = rd*rd;
        let rd3 = rd2*rd;
        let ro2 = ro*ro;
        let ro3 = ro2*ro;

        // raw quartic
        let k4 = vec3::dot(rd2, rd2);
        let mut k3 = vec3::dot(ro, rd3);
        let mut k2 = vec3::dot(ro2, rd2) - kb/6.0;
        let mut k1 = vec3::dot(ro3, rd) - kb*vec3::dot(rd, ro)/2.0;
        let mut k0 = vec3::dot(ro2, ro2) + ka - kb*vec3::dot(ro, ro);

        // make leading coefficient 1
        k3 /= k4;
        k2 /= k4;
        k1 /= k4;
        k0 /= k4;

        // reduced cubic
        let mut c2 = k2 - k3*k3;
        let mut c1 = k1 + k3*(2.0*k3*k3 - 3.0*k2);
        let mut c0 = k0 + k3*(k3*(c2 + k2)*3.0 - 4.0*k1);

        // prevent |c1| from being too close to zero
        // reduced cubic
        if (abs(c1) < 0.01*abs(c2)) {
            po = -1.0;
            let tmp = k1;
            k1 = k3;
            k3 = tmp;
            k0 = 1.0/k0;
            k1 = k1 * k0;
            k2 = k2 * k0;
            k3 = k3 * k0;
            c2 = k2 - k3*k3;
            c1 = k1 + k3*(2.0*k3*k3 - 3.0*k2);
            c0 = k0 + k3*(k3*(c2 + k2)*3.0- 4.0*k1);
        }

        c0 /= 3.0;
        let Q = c2*c2 + c0;
        let R = c2*c2*c2 - 3.0*c0*c2 + c1*c1;
        let mut h = R*R - Q*Q*Q;

        // 2 intersections
        if (h > 0.0) {
            h = sqrt(h);
            let s = sign(R + h)*pow(abs(R + h), 1.0/3.0); // cube root
            let u = sign(R - h)*pow(abs(R - h), 1.0/3.0); // cube root
            let x = s + u + 4.0*c2;
            let y = s - u;
            let ks = x*x + y*y*3.0;
            let k = sqrt(ks);

            let mut t = -0.5*po*abs(y)*sqrt(6.0/(k + x)) - 2.0*c1*(k + x)/(ks +x*k) - k3;
            if (po <= 0.0) {
                t = 1.0/t
            }

            let hit =  if (t > 0.0) { true } else { false };

            if (hit) {
                *out_t = t;
                *out_P = ray.pos + t * ray.dir;
                *out_N = self.get_normal(*out_P);
            }

            return hit;
        }

        // 4 intersections
        let sQ = sqrt(Q);
        let w = sQ * cos(acos(-R / (sQ * Q)) / 3.0);
        //let w = sQ*cos(atan2(sqrt(-h), -R)/3.0);
        let d2 = -w - c2;

        // no intersection
        if (d2 < 0.0) {
            return false;
        }

        let d1 = sqrt(d2);
        let h1 = sqrt(w - 2.0 * c2 + c1 / d1);
        let h2 = sqrt(w - 2.0 * c2 - c1 / d1);

        let mut t1 = -d1 - h1 - k3;
        if (po < 0.0) { t1 = 1.0 / t1; }

        let mut t2 = -d1 + h1 - k3;
        if (po < 0.0) { t2 = 1.0 / t2; }

        let mut t3 = d1 - h2 - k3;
        if (po < 0.0) { t3 = 1.0 / t3; }

        let mut t4 = d1 + h2 - k3;
        if (po < 0.0) { t4 = 1.0 / t4; }

        let mut t = 1e20;
        if (t1 > 0.0) { t = t1 };
        if (t2 > 0.0) { t = min(t, t2) };
        if (t3 > 0.0) { t = min(t, t3) };
        if (t4 > 0.0) { t = min(t, t4) };

        let hit = if ((t > 0.0) && (t < 1e20)) { true } else { false };

        if (hit) {
            *out_t = t;
            *out_P = ray.pos + t * ray.dir;
            *out_N = self.get_normal(*out_P);
        }

        return hit;
    }

    fn intersect_shadow(&self, ray: &Ray) -> bool {
        let mut t = 0.0;
        let mut P = vec3::ZERO;
        let mut N = vec3::ZERO;
        let hit = self.intersect_illum(ray, &mut t, &mut P, &mut N);
        return hit;
    }

    fn get_color(&self) -> &Vec3 {
        &self.color
    }

    fn get_transform(&self) -> &Transform {
        &self.transform
    }
}
