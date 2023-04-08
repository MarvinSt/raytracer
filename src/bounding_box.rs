use crate::ray::Ray;
use nalgebra::Vector3;
use std::mem::swap;

#[derive(Default, Copy, Clone)]
pub struct AABB {
    min: Vector3<f32>,
    max: Vector3<f32>,
}

impl AABB {
    pub fn new(min: Vector3<f32>, max: Vector3<f32>) -> Self {
        Self { min: min, max: max }
    }

    pub fn min(&self) -> Vector3<f32> {
        self.min
    }

    pub fn max(&self) -> Vector3<f32> {
        self.max
    }

    pub fn centroid(&self) -> Vector3<f32> {
        (self.min + self.max) * 0.5
    }

    pub fn surrounding_box(box0: &AABB, box1: &AABB) -> Self {
        let small: Vector3<f32> = Vector3::new(
            f32::min(box0.min.x, box1.min.x),
            f32::min(box0.min.y, box1.min.y),
            f32::min(box0.min.z, box1.min.z),
        );

        let big: Vector3<f32> = Vector3::new(
            f32::max(box0.max.x, box1.max.x),
            f32::max(box0.max.y, box1.max.y),
            f32::max(box0.max.z, box1.max.z),
        );

        return AABB::new(small, big);
    }

    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<AABB> {
        for i in 0..2 {
            let inv_dir = 1.0 / r.direction()[i];
            let mut t0 = (self.min[i] - r.origin()[i]) * inv_dir;
            let mut t1 = (self.max[i] - r.origin()[i]) * inv_dir;
            if inv_dir < 0.0 {
                swap(&mut t0, &mut t1);
            }

            let t_min = if t0 > t_min { t0 } else { t_min };
            let t_max = if t1 < t_max { t1 } else { t_max };
            if t_max <= t_min {
                return None;
            }
        }

        Some(AABB::default())
    }

    fn hit_ori(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<AABB> {
        for i in 0..2 {
            let t0 = f32::min(
                (self.min[i] - r.origin()[i]) / r.direction()[i],
                (self.max[i] - r.origin()[i]) / r.direction()[i],
            );
            let t1 = f32::max(
                (self.min[i] - r.origin()[i]) / r.direction()[i],
                (self.max[i] - r.origin()[i]) / r.direction()[i],
            );

            let t_min = f32::max(t0, t_min);
            let t_max = f32::min(t1, t_max);

            if t_max <= t_min {
                return None;
            }
        }

        Some(AABB::default())
    }
}