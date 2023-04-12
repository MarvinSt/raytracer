use crate::ray::Ray;
use nalgebra::{Vector2, Vector3};
use rand::Rng;
use std::f32::consts::PI;

pub fn random_in_unit_circle() -> Vector2<f32> {
    let mut rng = rand::thread_rng();

    let a = rng.gen::<f32>();
    let b = rng.gen::<f32>();

    const TWO_PI: f32 = 2.0 * std::f32::consts::PI;

    if b < a {
        Vector2::new(a * f32::cos(TWO_PI * b / a), a * f32::sin(TWO_PI * b / a))
    } else {
        Vector2::new(b * f32::cos(TWO_PI * a / b), b * f32::sin(TWO_PI * a / b))
    }
}

pub struct Camera {
    // focal_length: f32,
    pub aspect_ratio: f32,
    origin: Vector3<f32>,
    horizontal: Vector3<f32>,
    vertical: Vector3<f32>,
    lower_left_corner: Vector3<f32>,
    u: Vector3<f32>,
    v: Vector3<f32>,
    // w: Vector3<f32>,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        lookfrom: Vector3<f32>,
        lookat: Vector3<f32>,
        vup: Vector3<f32>,
        vfov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Self {
        // let focal_length = 1.0;
        let theta: f32 = vfov / 180.0 * PI;
        let h: f32 = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w: Vector3<f32> = (&lookfrom - &lookat).normalize();
        let u: Vector3<f32> = (Vector3::cross(&vup, &w)).normalize();
        let v: Vector3<f32> = Vector3::cross(&w, &u);

        let origin: Vector3<f32> = lookfrom;
        let horizontal: Vector3<f32> = focus_dist * viewport_width * u;
        let vertical: Vector3<f32> = focus_dist * viewport_height * v;
        let lower_left_corner: Vector3<f32> =
            origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;
        let lens_radius = aperture / 2.0;

        Self {
            // focal_length: focal_length,
            aspect_ratio: aspect_ratio,
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: lower_left_corner,
            u: u,
            v: v,
            // w: w,
            lens_radius: lens_radius,
        }
    }

    pub fn ray(&self, u: f32, v: f32) -> Ray {
        let rd: Vector2<f32> = self.lens_radius * random_in_unit_circle();
        let offset: Vector3<f32> = self.u * rd.x + self.v * rd.y;

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + u * self.horizontal + v * self.vertical
                - (self.origin + offset),
        )
    }
}
