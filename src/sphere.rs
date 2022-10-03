use std::ops::Neg;

use crate::hit::*;
use crate::material::Material;
use crate::ray::Ray;
use nalgebra::Vector3;

pub struct Sphere {
    center: Vector3<f32>,
    radius: f32,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vector3<f32>, radius: f32, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc: Vector3<f32> = r.origin() - self.center;

        // calculate the intersections
        let a: f32 = r.direction().magnitude_squared();
        let b_half: f32 = Vector3::dot(&oc, &r.direction());
        let c: f32 = oc.magnitude_squared() - self.radius * self.radius;

        let d = b_half * b_half - a * c;
        if d < 0.0 {
            return None;
        }

        let d_squared = d.sqrt();

        // find the nearest root that lies in the acceptable range.
        let mut root = (-b_half - d_squared) / a;
        if root < t_min || t_max < root {
            root = (-b_half + d_squared) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        // precalculate outputs
        let p: Vector3<f32> = r.point_at(root);
        let n: Vector3<f32> = (p - self.center) / self.radius;
        let f: bool = r.direction().dot(&n) < 0.0;

        // update hit record
        Some(HitRecord {
            t: root,
            p: p,
            n: if f { n } else { n.neg() },
            m: self.material,
            front_face: f,
        })
    }
}
