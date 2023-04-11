use cgmath::num_traits::abs;
use nalgebra::Vector3;
use std::ops::Neg;

use crate::{
    hit::{random_double, random_in_unit_sphere, random_unit_vector, HitRecord},
    ray::Ray,
    texture::Texture,
};

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
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vector3<f32>,
        scattered: &mut Ray,
    ) -> bool;

    fn emitted(&self, u: f32, v: f32, p: &Vector3<f32>) -> Vector3<f32> {
        Vector3::new(0.0, 0.0, 0.0)
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
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vector3<f32>,
        scattered: &mut Ray,
    ) -> bool {
        let direction: Vector3<f32> = rec.n + random_unit_vector();
        if near_zero(&direction) {
            *scattered = Ray::new(rec.p, rec.n);
        } else {
            *scattered = Ray::new(rec.p, direction);
        }
        *attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        true
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
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vector3<f32>,
        scattered: &mut Ray,
    ) -> bool {
        let direction: Vector3<f32> = reflect(&r_in.direction(), &rec.n);
        *scattered = Ray::new(rec.p, direction + self.fuzz * random_in_unit_sphere());
        *attenuation = self.albedo;
        scattered.direction().dot(&rec.n) > 0.0
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
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vector3<f32>,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Vector3::new(1.0, 1.0, 1.0);

        let refraction_ratio = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let cos_theta: f32 = f32::min(Vector3::dot(&r_in.direction().neg(), &rec.n), 1.0);
        let sin_theta: f32 = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction: Vector3<f32>;

        if cannot_refract || reflectance(cos_theta, refraction_ratio) > random_double(0.0, 1.0) {
            direction = reflect(&r_in.direction(), &rec.n);
        } else {
            direction = refract(&r_in.direction(), &rec.n, cos_theta, refraction_ratio);
        }
        *scattered = Ray::new(rec.p, direction);
        return true;
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
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vector3<f32>,
        scattered: &mut Ray,
    ) -> bool {
        false
    }

    fn emitted(&self, u: f32, v: f32, p: &Vector3<f32>) -> Vector3<f32> {
        self.emit.value(u, v, p)
    }
}
