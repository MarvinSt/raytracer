use crate::bounding_box::AABB;
use crate::hit::*;
use crate::material::Material;
use crate::ray::Ray;
use nalgebra::Vector3;

pub enum Plane {
    XY,
    XZ,
    YZ,
}

pub struct RectAA<M: Material> {
    plane: Plane,
    a0: f32,
    a1: f32,
    b0: f32,
    b1: f32,
    k: f32,
    material: M,
}

impl<M: Material> RectAA<M> {
    pub fn new(plane: Plane, a0: f32, a1: f32, b0: f32, b1: f32, k: f32, material: M) -> Self {
        Self {
            plane,
            a0,
            a1,
            b0,
            b1,
            k,
            material,
        }
    }

    pub fn xy(x0: f32, x1: f32, y0: f32, y1: f32, k: f32, material: M) -> Self {
        Self {
            plane: Plane::XY,
            a0: x0,
            a1: x1,
            b0: y0,
            b1: y1,
            k,
            material,
        }
    }

    pub fn xz(x0: f32, x1: f32, z0: f32, z1: f32, k: f32, material: M) -> Self {
        Self {
            plane: Plane::XZ,
            a0: x0,
            a1: x1,
            b0: z0,
            b1: z1,
            k,
            material,
        }
    }

    pub fn yz(y0: f32, y1: f32, z0: f32, z1: f32, k: f32, material: M) -> Self {
        Self {
            plane: Plane::YZ,
            a0: y0,
            a1: y1,
            b0: z0,
            b1: z1,
            k,
            material,
        }
    }
}

impl<M: Material> Hittable for RectAA<M> {
    fn bounding_box(&self) -> Option<AABB> {
        match &self.plane {
            Plane::XY => Some(AABB::new(
                Vector3::new(self.a0, self.b0, self.k - 0.0001),
                Vector3::new(self.a1, self.b1, self.k + 0.0001),
            )),
            Plane::XZ => Some(AABB::new(
                Vector3::new(self.a0, self.k - 0.0001, self.b0),
                Vector3::new(self.a1, self.k + 0.0001, self.b1),
            )),
            Plane::YZ => Some(AABB::new(
                Vector3::new(self.k - 0.0001, self.a0, self.b0),
                Vector3::new(self.k + 0.0001, self.a1, self.b1),
            )),
        }
    }

    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = match &self.plane {
            Plane::XY => (self.k - r.ori.z) / r.dir.z,
            Plane::XZ => (self.k - r.ori.y) / r.dir.y,
            Plane::YZ => (self.k - r.ori.x) / r.dir.x,
        };

        // calculate the intersections
        if t < t_min || t > t_max {
            return None;
        }

        let (a, b, on) = match &self.plane {
            Plane::XY => (
                r.ori.x + t * r.dir.x,
                r.ori.y + t * r.dir.y,
                Vector3::new(0.0, 0.0, 1.0),
            ),
            Plane::XZ => (
                r.ori.x + t * r.dir.x,
                r.ori.z + t * r.dir.z,
                Vector3::new(0.0, 1.0, 0.0),
            ),
            Plane::YZ => (
                r.ori.y + t * r.dir.y,
                r.ori.z + t * r.dir.z,
                Vector3::new(1.0, 0.0, 0.0),
            ),
        };

        if a < self.a0 || a > self.a1 || b < self.b0 || b > self.b1 {
            return None;
        }

        let u = (a - self.a0) / (self.a1 - self.a0);
        let v = (b - self.b0) / (self.b1 - self.b0);
        let p = r.point_at(t);

        let mut h = HitRecord {
            t: t,
            p: p,
            n: on,
            m: &self.material,
            front_face: false,
            u,
            v,
        };

        h.set_face_normal(r, &on);

        Some(h)
    }
}
