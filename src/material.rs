use crate::{hit::HitRecord, pdf::Pdf, ray::Ray, texture::Texture};
use nalgebra::Vector3;
use rand::{distributions::Uniform, prelude::Distribution, Rng};

pub enum ScatterRecord<'a> {
    Scatter {
        attenuation: Vector3<f32>,
        pdf: Pdf<'a>,
    },
    Specular {
        attenuation: Vector3<f32>,
        specular_ray: Ray,
    },
    Isotropic {
        attenuation: Vector3<f32>,
        scattered_ray: Ray,
    },
}

pub fn random_cosine_direction() -> Vector3<f32> {
    let mut rng = rand::thread_rng();
    let (r1, r2) = rng.gen::<(f32, f32)>();

    let phi = 2.0 * std::f32::consts::PI * r1;
    let x = f32::cos(phi) * f32::sqrt(r2);
    let y = f32::sin(phi) * f32::sqrt(r2);
    let z = f32::sqrt(1.0 - r2);

    Vector3::new(x, y, z)
}

pub fn random_unit_vector() -> Vector3<f32> {
    let mut rng = rand::thread_rng();
    let (r1, r2) = rng.gen::<(f32, f32)>();

    let x = f32::cos(2.0 * std::f32::consts::PI * r1) * 2.0 * f32::sqrt(r2 * (1.0 - r2));
    let y = f32::sin(2.0 * std::f32::consts::PI * r1) * 2.0 * f32::sqrt(r2 * (1.0 - r2));
    let z = 1.0 - 2.0 * r2;

    Vector3::new(x, y, z)
}

pub fn random_in_unit_sphere() -> Vector3<f32> {
    let mut rng = rand::thread_rng();

    const MIN: f32 = -1.0;
    const MAX: f32 = 1.0;

    let uni = Uniform::from(MIN..=MAX);

    loop {
        let v = Vector3::new(
            uni.sample(&mut rng),
            uni.sample(&mut rng),
            uni.sample(&mut rng),
        );

        if v.magnitude_squared() <= 1.0 {
            return v;
        }
    }
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
    let uv = r_in; // .normalize();
    let r_out_perp: Vector3<f32> = refraction_ratio * (uv + cos_theta * n);
    let r_out_parallel: Vector3<f32> = -n * ((1.0 - r_out_perp.norm_squared()).abs().sqrt());
    r_out_perp + r_out_parallel
}

fn reflectance(cosine: f32, refraction_ratio: f32) -> f32 {
    // Use Schlick's approximation for reflectance.
    let r0 = ((1.0 - refraction_ratio) / (1.0 + refraction_ratio)).powi(2);
    r0 + (1.0 - r0) * ((1.0 - cosine).powi(5))
}

pub trait Material: Sync {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<ScatterRecord> {
        None
    }

    fn scattering_pdf(&self, _rec: &HitRecord, _scattered: &Ray) -> f32 {
        0.0
    }

    fn emitted(&self, _rec: &HitRecord) -> Vector3<f32> {
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
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        // let uvw = Onb::build_from_w(rec.n);
        // let direction: Vector3<f32> = uvw.local(&random_cosine_direction());
        // let scattered = Ray::new(rec.p, direction.normalize());
        let attenuation: Vector3<f32> = self.albedo.value(rec.u, rec.v, &rec.p);
        let pdf = Pdf::cosine_pdf(rec.n);

        Some(ScatterRecord::Scatter { attenuation, pdf })
    }

    fn scattering_pdf(&self, rec: &HitRecord, scattered: &Ray) -> f32 {
        let cosine = f32::max(rec.n.dot(&scattered.nrm_dir), 0.0);
        cosine / std::f32::consts::PI
    }
}

#[derive(Clone)]
pub struct Metal<T: Texture> {
    albedo: T,
    fuzz: f32,
}

impl<T: Texture> Metal<T> {
    pub fn new(albedo: T, fuzz: f32) -> Self {
        Metal { albedo, fuzz }
    }
}

impl<T: Texture> Material for Metal<T> {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected: Vector3<f32> = reflect(&r_in.nrm_dir, &rec.n);
        let direction = reflected + self.fuzz * random_in_unit_sphere();
        let specular_ray = Ray::new(rec.p, direction);
        let attenuation: Vector3<f32> = self.albedo.value(rec.u, rec.v, &rec.p);

        // if direction.dot(&rec.n) > 0.0 {
        Some(ScatterRecord::Specular {
            specular_ray,
            attenuation,
        })
        // } else {
        //     None
        // }
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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let attenuation = Vector3::new(1.0, 1.0, 1.0);

        let refraction_ratio = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_dir = r_in.nrm_dir; // ;.direction().normalize();
        let cos_theta: f32 = (-unit_dir).dot(&rec.n);
        let sin_theta: f32 = f32::max(1.0 - cos_theta * cos_theta, 0.0).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let mut rng = rand::thread_rng();
        let direction =
            if cannot_refract || reflectance(cos_theta, refraction_ratio) > rng.gen::<f32>() {
                reflect(&unit_dir, &rec.n)
            } else {
                refract(&unit_dir, &rec.n, cos_theta, refraction_ratio)
            };

        let specular_ray = Ray::new(rec.p, direction);
        Some(ScatterRecord::Specular {
            specular_ray,
            attenuation,
        })
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
    fn emitted(&self, rec: &HitRecord) -> Vector3<f32> {
        if rec.front_face {
            self.emit.value(rec.u, rec.v, &rec.p)
        } else {
            Vector3::new(0.0, 0.0, 0.0)
        }
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
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let scattered_ray = Ray::new(rec.p, random_in_unit_sphere());
        let attenuation = self.albedo.value(rec.u, rec.v, &rec.p);

        Some(ScatterRecord::Isotropic {
            attenuation,
            scattered_ray,
        })
    }
}
