use crate::{bhv::Bvh, bounding_box::AABB, material::*};
use cgmath::num_traits::Float;
use nalgebra::{Vector2, Vector3};
use rand::Rng;

use crate::ray::Ray;

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self) -> Option<AABB>;
}

#[derive(Copy, Clone, Default)]
pub struct HitRecord {
    pub p: Vector3<f32>,
    pub n: Vector3<f32>,
    pub t: f32,
    pub m: Material,
    pub front_face: bool,
}

#[inline]
pub fn random_double(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

#[inline]
pub fn random_int(min: u32, max: u32) -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

#[inline]
pub fn random_unit_vector() -> Vector3<f32> {
    const MIN: f32 = -1.0;
    const MAX: f32 = 1.0;

    let mut rng = rand::thread_rng();

    Vector3::new(
        rng.gen_range(MIN..MAX),
        rng.gen_range(MIN..MAX),
        rng.gen_range(MIN..MAX),
    )
    .normalize()
}

#[inline]
pub fn random_unit_circle() -> Vector2<f32> {
    const MIN: f32 = -1.0;
    const MAX: f32 = 1.0;

    let mut rng = rand::thread_rng();

    Vector2::new(rng.gen_range(MIN..MAX), rng.gen_range(MIN..MAX))
        / ((1.0 * 1.0 + 1.0 * 1.0).sqrt())
}

#[inline]
pub fn random_unit_sphere() -> Vector3<f32> {
    const MIN: f32 = -1.0;
    const MAX: f32 = 1.0;

    let mut rng = rand::thread_rng();

    Vector3::new(
        rng.gen_range(MIN..MAX),
        rng.gen_range(MIN..MAX),
        rng.gen_range(MIN..MAX),
    ) / ((1.0 * 1.0 + 1.0 * 1.0 + 1.0 * 1.0).sqrt())
}

// #[derive(Sync)]
pub struct World {
    // pub rec: HitRecord,
    pub objects: Vec<Box<dyn Hittable>>,
}

unsafe impl Sync for World {}
unsafe impl Send for World {}

impl<'a> World {
    pub fn new() -> Self {
        Self {
            // rec: HitRecord::default(),
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj);
    }

    // pub fn objects(&self) -> &Vec<Box<dyn Hittable>> {
    //     &self.objects
    // }

    pub fn generate_bvh(self) -> World {
        let bvh = Box::new(Bvh::new(self.objects));
        let mut world = World::new();
        world.add(bvh);
        world
    }

    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut closest_hit = t_max;

        for obj in self.objects.iter() {
            if let Some(current_hit) = obj.hit(r, t_min, closest_hit) {
                closest_hit = current_hit.t;
                hit_record = Some(current_hit).clone();
            }
        }

        hit_record
    }

    // pub fn bounding_box(&self) -> Option<AABB> {
    //     if self.objects.is_empty() {
    //         return None;
    //     }

    //     let mut output_box: AABB = AABB::default();

    //     let mut first_box = true;
    //     for obj in self.objects.iter() {
    //         // let mut temp_box: AABB = AABB::default();
    //         if let Some(temp_box) = obj.bounding_box() {
    //             output_box = if first_box {
    //                 temp_box
    //             } else {
    //                 AABB::surrounding_box(&output_box, &temp_box)
    //             };
    //             first_box = false;
    //         }
    //     }

    //     Some(output_box)
    // }
}

pub fn ray_color(ray: &Ray, world: &World, depth: u8) -> Vector3<f32> {
    // test object collision
    if let Some(hit) = world.hit(&ray, 0.001, 1.0e8) {
        let mut scattered: Ray = Ray::default();
        let mut attenuation: Vector3<f32> = Vector3::new(0.0, 0.0, 0.0);
        if depth == 0 {
            return attenuation;
        }
        if hit.m.scatter(&ray, &hit, &mut attenuation, &mut scattered) {
            let color: Vector3<f32> = ray_color(&scattered, &world, depth - 1);
            return attenuation.component_mul(&color);
        } else {
            return attenuation;
        }
    }

    // background
    let n: Vector3<f32> = ray.direction().normalize();
    let t = 0.5 * (n.y + 1.0);
    let env_color: Vector3<f32> =
        (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0);

    env_color

    /*
    // without recursion
    let mut depth = depth;
    let mut ray = ray.clone();
    let mut scattered: Ray = Ray::default();

    // background
    let n: Vector3<f32> = ray.direction().normalize();
    let t: f32 = 0.5 * (n.y + 1.0);
    let mut color: Vector3<f32> =
        (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0);

    loop {
        if let Some(hit) = world.hit(&ray, 0.001, 1.0e8) {
            depth -= 1;
            if depth == 0 {
                color = Vector3::new(0.0, 0.0, 0.0);
                break;
            }
            let mut attenuation: Vector3<f32> = Vector3::default();
            if hit.m.scatter(&ray, &hit, &mut attenuation, &mut scattered) {
                ray = scattered;
                color = color.component_mul(&attenuation);
            } else {
                color = Vector3::new(0.0, 0.0, 0.0);
                break;
            }
        } else {
            break;
        }
    }

    color
    */
}
