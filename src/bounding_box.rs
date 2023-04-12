use crate::ray::Ray;
use nalgebra::Vector3;
use std::mem::swap;

#[derive(Copy, Clone)]
pub struct AABB {
    min: Vector3<f32>,
    max: Vector3<f32>,
}

impl AABB {
    pub fn new(min: Vector3<f32>, max: Vector3<f32>) -> Self {
        Self { min: min, max: max }
    }

    pub fn default() -> Self {
        Self {
            min: Vector3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY),
            max: Vector3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY),
        }
    }

    pub fn min(&self) -> Vector3<f32> {
        self.min
    }

    pub fn max(&self) -> Vector3<f32> {
        self.max
    }

    pub fn diff(&self) -> Vector3<f32> {
        return self.max - self.min;
    }

    pub fn max_axis(&self) -> usize {
        let diff = self.diff();
        let cmp_a = diff[0] >= diff[1];
        let cmp_b = diff[1] >= diff[2];
        if cmp_a && cmp_b {
            return 0;
        } else {
            if cmp_b {
                return 1;
            } else {
                return 2;
            }
        }
    }

    /*
    pub fn centroid(&self) -> Vector3<f32> {
        (self.min + self.max) * 0.5
    }
    */

    pub fn sort_value_axis(&self, axis: usize) -> f32 {
        self.min[axis] + self.max[axis]
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

    pub fn extend_box(&self, aabb: &AABB) -> Self {
        let small: Vector3<f32> = Vector3::new(
            f32::min(self.min.x, aabb.min.x),
            f32::min(self.min.y, aabb.min.y),
            f32::min(self.min.z, aabb.min.z),
        );

        let big: Vector3<f32> = Vector3::new(
            f32::max(self.max.x, aabb.max.x),
            f32::max(self.max.y, aabb.max.y),
            f32::max(self.max.z, aabb.max.z),
        );

        return AABB::new(small, big);
    }

    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> bool {
        for i in 0..3 {
            let inv_dir = 1.0 / r.direction()[i];
            let mut t0 = (self.min[i] - r.origin()[i]) * inv_dir;
            let mut t1 = (self.max[i] - r.origin()[i]) * inv_dir;
            if inv_dir < 0.0 {
                swap(&mut t0, &mut t1);
            }

            let t_min = t_min.max(t0); // if t0 > t_min { t0 } else { t_min };
            let t_max = t_max.min(t1); // if t1 < t_max { t1 } else { t_max };
            if t_max <= t_min {
                return false;
            }
        }
        true
        // Some(*self)
    }
}
