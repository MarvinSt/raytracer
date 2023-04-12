use cgmath::num_traits::abs;
use nalgebra::Vector3;
use rand::{distributions::Uniform, prelude::Distribution, Rng};

use crate::{hit::HitRecord, ray::Ray, texture::Texture};

pub fn random_unit_vector() -> Vector3<f32> {
    let mut rng = rand::thread_rng();

    const MIN: f32 = -1.0;
    const MAX: f32 = 1.0;

    let uni = Uniform::from(MIN..=MAX);

    Vector3::new(
        uni.sample(&mut rng),
        uni.sample(&mut rng),
        uni.sample(&mut rng),
    )
    .normalize()
}

pub fn random_in_unit_sphere() -> Vector3<f32> {
    let mut rng = rand::thread_rng();

    const SCL1: f32 = std::f32::consts::SQRT_2 / 2.0;
    const SCL2: f32 = std::f32::consts::SQRT_2 * 2.0;

    // unitrand in [-1,1].
    let u = SCL1 * rng.gen::<f32>();
    let v = SCL1 * rng.gen::<f32>();
    let w = SCL2 * f32::sqrt(1.0 - u * u - v * v);

    let x = w * u;
    let y = w * v;
    let z = 1.0 - 2.0 * (u * u + v * v);

    Vector3::new(x, y, z)
}

fn near_zero(v: &Vector3<f32>) -> bool {
    // Return true if the vector is close to zero in all dimensions.
    const S: f32 = 1e-8;
    return (abs(v[0]) < S) && (abs(v[1]) < S) && (abs(v[2]) < S);
}

fn reflect(v: &Vector3<f32>, n: &Vector3<f32>) -> Vector3<f32> {
    return v - 2.0 * v.dot(n) * n;
}

fn refract(
    r_in: &Vector3<f32>,
    n: &Vector3<f32>,
    cos_theta: f32,
    refraction_ratio: f32,
) -> Vector3<f32> {
    let uv: Vector3<f32> = r_in.normalize();
    let r_out_perp: Vector3<f32> = refraction_ratio * (uv + (cos_theta * n));
    let r_out_parallel: Vector3<f32> = -(1.0 - r_out_perp.norm_squared()).abs().sqrt() * n;
    r_out_perp + r_out_parallel
}

fn reflectance(cosine: f32, refraction_ratio: f32) -> f32 {
    // Use Schlick's approximation for reflectance.
    let r0 = ((1.0 - refraction_ratio) / (1.0 + refraction_ratio)).powi(2);
    r0 + (1.0 - r0) * ((1.0 - cosine).powi(5))
}

pub trait Material: Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vector3<f32>, Ray)>;

    fn emitted(&self, u: f32, v: f32, p: &Vector3<f32>) -> Vector3<f32> {
        Vector3::zeros()
    }
}

#[test]
fn test_reflectance() {
    let cosine = 0.0;
    let ref_idx = 1.5;
    let expected = 1.0;
    let actual = reflectance(cosine, ref_idx);
    assert_eq!(actual, expected);
}

#[derive(Clone)]
pub struct Lambertian<T: Texture> {
    albedo: T,
}

impl<T: Texture> Lambertian<T> {
    pub fn new(albedo: T) -> Self {
        Lambertian { albedo }
    }
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vector3<f32>, Ray)> {
        let direction: Vector3<f32> = rec.n + random_unit_vector();
        let scattered = if near_zero(&direction) {
            Ray::new(rec.p, rec.n)
        } else {
            Ray::new(rec.p, direction)
        };
        let attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        Some((attenuation, scattered))
    }
}

#[derive(Clone)]
pub struct Metal {
    albedo: Vector3<f32>,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vector3<f32>, fuzz: f32) -> Self {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vector3<f32>, Ray)> {
        let direction: Vector3<f32> = reflect(&r_in.direction(), &rec.n);
        let scattered = Ray::new(rec.p, direction + self.fuzz * random_in_unit_sphere());
        let attenuation = self.albedo;

        match scattered.direction().dot(&rec.n) > 0.0 {
            true => Some((attenuation, scattered)),
            false => None,
        }
    }
}

#[derive(Clone)]
pub struct Dielectric {
    refraction_index: f32,
}

impl Dielectric {
    pub fn new(refraction_index: f32) -> Self {
        Dielectric { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vector3<f32>, Ray)> {
        let attenuation = Vector3::new(1.0, 1.0, 1.0);

        let refraction_ratio = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let cos_theta: f32 = f32::min(-&r_in.direction().dot(&rec.n), 1.0);
        let sin_theta: f32 = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction: Vector3<f32>;

        let mut rng = rand::thread_rng();
        if cannot_refract || reflectance(cos_theta, refraction_ratio) > rng.gen::<f32>() {
            direction = reflect(&r_in.direction(), &rec.n);
        } else {
            direction = refract(&r_in.direction(), &rec.n, cos_theta, refraction_ratio);
        }

        let scattered = Ray::new(rec.p, direction);
        return Some((attenuation, scattered));
    }
}

#[derive(Clone)]
pub struct DiffuseLight<T: Texture> {
    emit: T,
}

impl<T: Texture> DiffuseLight<T> {
    pub fn new(emit: T) -> Self {
        DiffuseLight { emit }
    }
}

impl<T: Texture> Material for DiffuseLight<T> {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vector3<f32>, Ray)> {
        None
    }

    fn emitted(&self, u: f32, v: f32, p: &Vector3<f32>) -> Vector3<f32> {
        self.emit.value(u, v, p)
    }
}

#[derive(Clone)]
pub struct Isotropic<T: Texture> {
    albedo: T,
}

impl<T: Texture> Isotropic<T> {
    pub fn new(albedo: T) -> Self {
        Isotropic { albedo }
    }
}

impl<T: Texture> Material for Isotropic<T> {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vector3<f32>, Ray)> {
        let scattered = Ray::new(rec.p, random_in_unit_sphere());
        let attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        Some((attenuation, scattered))
    }
}
