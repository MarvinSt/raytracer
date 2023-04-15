use crate::{bounding_box::AABB, material::*, ray::Ray};
use nalgebra::Vector3;
use rand::{
    distributions::{Distribution, Uniform},
    seq::SliceRandom,
    Rng,
};

pub trait Hittable: Sync {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self) -> Option<AABB>;
    fn pdf_value(&self, _origin: Vector3<f32>, _direction: Vector3<f32>) -> f32 {
        0.0
    }
    fn random(&self, _origin: Vector3<f32>) -> Vector3<f32> {
        Vector3::new(1.0, 0.0, 0.0)
    }
}

#[derive(Copy, Clone)]
pub struct HitRecord<'a> {
    pub p: Vector3<f32>,
    pub n: Vector3<f32>,
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub m: &'a dyn Material,
    pub front_face: bool,
}

impl HitRecord<'_> {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vector3<f32>) {
        self.front_face = r.dir.dot(outward_normal) < 0.0;
        self.n = if self.front_face {
            *outward_normal
        } else {
            -outward_normal
        };
    }
}

pub struct World {
    // pub rec: HitRecord,
    pub objects: Vec<Box<dyn Hittable>>,
    pub aabb: Option<AABB>,
}

unsafe impl Sync for World {}
unsafe impl Send for World {}

impl World {
    pub fn new() -> Self {
        Self {
            // rec: HitRecord::default(),
            objects: Vec::new(),
            aabb: Some(AABB::default()),
        }
    }

    pub fn push<H: Hittable + 'static>(&mut self, obj: H) {
        self.objects.push(Box::new(obj));

        let aabb = self.objects.iter().fold(AABB::default(), |a, b| {
            a.extend_box(&b.bounding_box().unwrap())
        });

        self.aabb = Some(aabb);

        // if aabb.min() == AABB::default().min() && aabb.max() == AABB::default().max() {
        //     None
        // } else {
        //     Some(aabb)
        // }
    }
}

impl Hittable for World {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut closest_hit = t_max;

        for obj in self.objects.iter() {
            if let Some(current_hit) = obj.hit(r, t_min, closest_hit) {
                closest_hit = current_hit.t;
                hit_record = Some(current_hit);
            }
        }

        hit_record
    }

    fn bounding_box(&self) -> Option<AABB> {
        return self.aabb;
    }

    fn pdf_value(&self, origin: Vector3<f32>, direction: Vector3<f32>) -> f32 {
        self.objects
            .iter()
            .map(|h| h.pdf_value(origin, direction))
            .sum::<f32>()
            / self.objects.len() as f32
    }

    fn random(&self, origin: Vector3<f32>) -> Vector3<f32> {
        if self.objects.len() == 0 {
            Vector3::new(1.0, 0.0, 0.0)
        } else {
            self.objects
                .choose(&mut rand::thread_rng())
                .unwrap()
                .random(origin)
        }
    }
}

pub fn random_double(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..=max) as f32
}

pub fn random_int(min: u32, max: u32) -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..=max)
}

pub fn random_color_vector() -> Vector3<f32> {
    const MIN: f32 = 0.0;
    const MAX: f32 = 1.0;

    let mut rng = rand::thread_rng();

    let uni = Uniform::from(MIN..=MAX);

    Vector3::new(
        uni.sample(&mut rng),
        uni.sample(&mut rng),
        uni.sample(&mut rng),
    )
}
