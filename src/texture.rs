use crate::noise::Perlin;
use image::{open, ImageBuffer, Rgb};
use nalgebra::{clamp, Vector3};

pub trait Texture: Sync {
    fn value(&self, u: f32, v: f32, p: &Vector3<f32>) -> Vector3<f32>;
}

#[derive(Clone)]
pub struct SolidColor {
    color: Vector3<f32>,
}

impl SolidColor {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        SolidColor {
            color: Vector3::new(r, g, b),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, u: f32, v: f32, p: &Vector3<f32>) -> Vector3<f32> {
        self.color
    }
}

#[derive(Clone)]
pub struct Checker<T: Texture, U: Texture> {
    odd: T,
    even: U,
}

impl<T: Texture, U: Texture> Checker<T, U> {
    pub fn new(odd: T, even: U) -> Self {
        Checker { odd, even }
    }
}

impl<T: Texture, U: Texture> Texture for Checker<T, U> {
    fn value(&self, u: f32, v: f32, p: &Vector3<f32>) -> Vector3<f32> {
        let sines = f32::sin(10.0 * p.x) * f32::sin(10.0 * p.y) * f32::sin(10.0 * p.z);
        if sines < 0.0 {
            return self.odd.value(u, v, p);
        } else {
            return self.even.value(u, v, p);
        }
    }
}

#[derive(Clone)]
pub struct Image {
    data: ImageBuffer<Rgb<u8>, Vec<u8>>,
    width: u32,
    height: u32,
}

impl Image {
    pub fn new(path: &str) -> Image {
        let data = open(path).unwrap_or_default().into_rgb8();
        let (width, height) = (data.width(), data.height());
        Image {
            data,
            width,
            height,
        }
    }
}

impl Texture for Image {
    fn value(&self, u: f32, v: f32, p: &Vector3<f32>) -> Vector3<f32> {
        // If we have no texture data, then return solid cyan as a debugging aid.
        if self.data.len() == 0 {
            return Vector3::new(0.0, 1.0, 1.0);
        }

        // Clamp input texture coordinates to [0,1] x [1,0]
        let u = clamp(u, 0.0, 1.0);
        let v = 1.0 - clamp(v, 0.0, 1.0); // Flip V to image coordinates

        let i = ((u * self.width as f32) as u32).min(self.width - 1);
        let j = ((v * self.height as f32) as u32).min(self.height - 1);

        let pixel = self.data.get_pixel(i, j);
        const COLOR_SCALE: f32 = 1.0 / 255.0;

        Vector3::new(
            COLOR_SCALE * pixel.0[0] as f32,
            COLOR_SCALE * pixel.0[1] as f32,
            COLOR_SCALE * pixel.0[2] as f32,
        )
    }
}

#[derive(Clone)]
pub struct Noise {
    noise: Perlin,
    scale: f32,
}

impl Noise {
    pub fn new(scale: f32) -> Self {
        Noise {
            noise: Perlin::new(),
            scale: scale,
        }
    }
}

impl Texture for Noise {
    fn value(&self, u: f32, v: f32, p: &Vector3<f32>) -> Vector3<f32> {
        Vector3::new(0.5, 0.5, 0.5)
            * (1.0 + f32::sin(self.scale * p.z + 10.0 * self.noise.turb(&p, 7)))
    }
}
