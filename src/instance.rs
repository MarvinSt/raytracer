use crate::{
    bounding_box::AABB,
    hit::{HitRecord, Hittable},
    ray::Ray,
};
use nalgebra::Vector3;

pub struct Translate<H: Hittable> {
    obj: H,
    offset: Vector3<f32>,
}

pub struct Rotate<H: Hittable> {
    obj: H,
    sin_theta: f32,
    cos_theta: f32,
    aabb: Option<AABB>,
}

impl<H: Hittable> Translate<H> {
    pub fn new(obj: H, offset: Vector3<f32>) -> Self {
        Self { obj, offset }
    }
}

impl<H: Hittable> Rotate<H> {
    pub fn new(obj: H, angle: f32) -> Self {
        let angle = angle.to_radians();
        let cos_theta = f32::cos(angle);
        let sin_theta = f32::sin(angle);

        let bbox = obj.bounding_box();

        let mut min: Vector3<f32> = Vector3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
        let mut max: Vector3<f32> =
            Vector3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);

        match bbox {
            Some(bbox) => {
                for i in 0..2 {
                    for j in 0..2 {
                        for k in 0..2 {
                            let x = i as f32 * bbox.max().x + (1 - i) as f32 * bbox.min().x;
                            let y = j as f32 * bbox.max().y + (1 - j) as f32 * bbox.min().y;
                            let z = k as f32 * bbox.max().z + (1 - k) as f32 * bbox.min().z;

                            let newx = cos_theta * x + sin_theta * z;
                            let newz = -sin_theta * x + cos_theta * z;

                            let tester = Vector3::new(newx, y, newz);

                            for c in 0..3 {
                                min[c] = min[c].min(tester[c]);
                                max[c] = max[c].max(tester[c]);
                            }
                        }
                    }
                }

                Self {
                    obj,
                    cos_theta,
                    sin_theta,
                    aabb: Some(AABB::new(min, max)),
                }
            }
            None => Self {
                obj,
                cos_theta,
                sin_theta,
                aabb: None,
            },
        }
    }
}

impl<H: Hittable> Hittable for Translate<H> {
    fn bounding_box(&self) -> Option<AABB> {
        match self.obj.bounding_box() {
            Some(aabb) => Some(AABB::new(
                aabb.min() + self.offset,
                aabb.max() + self.offset,
            )),
            None => None,
        }
    }

    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let moved_r = Ray::new(r.origin() - self.offset, r.direction());

        match self.obj.hit(&moved_r, t_min, t_max) {
            Some(mut hit) => {
                let on = hit.n;
                hit.p += self.offset;
                hit.set_face_normal(&moved_r, &on);
                Some(hit)
            }
            None => None,
        }
    }
}

impl<H: Hittable> Hittable for Rotate<H> {
    fn bounding_box(&self) -> Option<AABB> {
        return self.aabb;
    }

    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut origin: Vector3<f32> = r.origin();
        let mut direction: Vector3<f32> = r.direction();

        origin.x = self.cos_theta * r.origin().x - self.sin_theta * r.origin().z;
        origin.z = self.sin_theta * r.origin().x + self.cos_theta * r.origin().z;

        direction.x = self.cos_theta * r.direction().x - self.sin_theta * r.direction().z;
        direction.z = self.sin_theta * r.direction().x + self.cos_theta * r.direction().z;

        let rotated_r = Ray::new(origin, direction);

        match self.obj.hit(&rotated_r, t_min, t_max) {
            Some(mut hit) => {
                let mut p: Vector3<f32> = hit.p;
                let mut n: Vector3<f32> = hit.n;

                p[0] = self.cos_theta * hit.p.x + self.sin_theta * hit.p.z;
                p[2] = -self.sin_theta * hit.p.x + self.cos_theta * hit.p.z;

                n[0] = self.cos_theta * hit.n.x + self.sin_theta * hit.n.z;
                n[2] = -self.sin_theta * hit.n.x + self.cos_theta * hit.n.z;

                hit.p = p;
                hit.set_face_normal(&rotated_r, &n);
                Some(hit)
            }
            None => None,
        }
    }
}
