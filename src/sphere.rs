use crate::bounding_box::AABB;
use crate::hit::*;
use crate::material::Material;
use crate::ray::Ray;
use nalgebra::Vector3;

pub struct Sphere<M: Material> {
    center: Vector3<f32>,
    radius: f32,
    material: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Vector3<f32>, radius: f32, material: M) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl<M: Material> Sphere<M> {
    fn get_uv(p: &Vector3<f32>) -> (f32, f32) {
        // p: a given point on the sphere of radius one, centered at the origin.
        // u: returned value [0,1] of angle around the Y axis from X=-1.
        // v: returned value [0,1] of angle from Y=-1 to Y=+1.
        //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
        //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
        //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>

        let theta = f32::acos(-p.y);
        let phi = f32::atan2(-p.z, p.x) + std::f32::consts::PI;

        let u = phi / (2.0 * std::f32::consts::PI);
        let v = theta / std::f32::consts::PI;

        (u, v)
    }
}

impl<M: Material> Hittable for Sphere<M> {
    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB::new(
            self.center - Vector3::new(self.radius, self.radius, self.radius),
            self.center + Vector3::new(self.radius, self.radius, self.radius),
        ))
    }

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

        // get uv's
        let (u, v) = Sphere::<M>::get_uv(&p);

        // update hit record
        Some(HitRecord {
            t: root,
            p: p,
            n: if f { n } else { -n },
            m: &self.material,
            front_face: f,
            u,
            v,
        })
    }
}
