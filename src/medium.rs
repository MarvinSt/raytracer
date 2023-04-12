use crate::{
    bounding_box::AABB,
    hit::{HitRecord, Hittable},
    material::Isotropic,
    ray::Ray,
    texture::Texture,
};
use nalgebra::Vector3;
use rand::Rng;

pub struct Constant<H: Hittable, T: Texture> {
    boundary: H,
    phase_function: Isotropic<T>,
    neg_inv_density: f32,
}

impl<H: Hittable, T: Texture> Constant<H, T> {
    pub fn new(boundary: H, density: f32, texture: T) -> Self {
        Constant {
            boundary: boundary,
            phase_function: Isotropic::new(texture),
            neg_inv_density: -1.0 / density,
        }
    }
}

impl<H: Hittable, T: Texture> Hittable for Constant<H, T> {
    fn bounding_box(&self) -> Option<AABB> {
        self.boundary.bounding_box()
    }

    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut rng = rand::thread_rng();
        let hit1 = self.boundary.hit(r, f32::MIN, f32::MAX);
        match hit1 {
            Some(hit1) => {
                let hit2 = self.boundary.hit(r, hit1.t + 0.0001, f32::MAX);
                match hit2 {
                    Some(hit2) => {
                        let t_min = hit1.t.max(t_min);
                        let t_max = hit2.t.min(t_max);
                        // hit2.t = hit2.t.min(t_max);

                        if t_min >= t_max {
                            return None;
                        }

                        let t_min = t_min.max(0.0);

                        let ray_length = r.direction().magnitude();
                        let distance_inside_boundary = (t_max - t_min) * ray_length;
                        // let hit_distance = self.neg_inv_density * (rng.gen::<f32>().ln());
                        let rand_double: f32 = rng.gen_range(0.0..=1.0);
                        let hit_distance = self.neg_inv_density * rand_double.ln();
                        if hit_distance > distance_inside_boundary {
                            return None;
                        }

                        let t = t_min + hit_distance / ray_length;
                        let p = r.point_at(t);

                        return Some(HitRecord {
                            p,
                            n: Vector3::new(1.0, 0.0, 0.0), // arbitrary
                            t,
                            u: 0.0,
                            v: 0.0,
                            m: &self.phase_function,
                            front_face: true, // arbitrary
                        });
                    }
                    None => return None,
                }
            }
            None => None,
        }
    }
}
