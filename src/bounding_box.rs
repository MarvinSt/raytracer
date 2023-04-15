use crate::ray::Ray;
use nalgebra::Vector3;
use std::ops::Index;

#[derive(Debug, Copy, Clone)]
pub struct AABB {
    min: Vector3<f32>,
    max: Vector3<f32>,
}

impl Index<usize> for AABB {
    type Output = Vector3<f32>;

    fn index(&self, index: usize) -> &Vector3<f32> {
        if index == 0 {
            &self.min
        } else {
            &self.max
        }
    }
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
        // Algorithm from: https://github.com/svenstaro/bvh
        let mut ray_min = (&self[r.sign_x].x - r.ori.x) * r.inv_dir.x;
        let mut ray_max = (&self[1 - r.sign_x].x - r.ori.x) * r.inv_dir.x;

        let y_min = (self[r.sign_y].y - r.ori.y) * r.inv_dir.y;
        let y_max = (self[1 - r.sign_y].y - r.ori.y) * r.inv_dir.y;

        ray_min = f32::max(ray_min, y_min);
        ray_max = f32::min(ray_max, y_max);

        let z_min = (self[r.sign_z].z - r.ori.z) * r.inv_dir.z;
        let z_max = (self[1 - r.sign_z].z - r.ori.z) * r.inv_dir.z;

        ray_min = f32::max(ray_min, z_min);
        ray_max = f32::min(ray_max, z_max);

        f32::max(ray_min, t_min) <= f32::min(ray_max, t_max)

        /*
        for i in 0..3 {
            // let inv_dir = 1.0 / r.inv_dir[i];
            let mut t0 = (self.min[i] - r.ori[i]) * r.inv_dir[i];
            let mut t1 = (self.max[i] - r.ori[i]) * r.inv_dir[i];
            if t0 > t1 {
                swap(&mut t0, &mut t1);
            }

            let t_min = f32::max(t_min, t0);
            let t_max = f32::min(t_max, t1);
            if t_max <= t_min {
                return false;
            }
        }
        true

         */
    }
}
