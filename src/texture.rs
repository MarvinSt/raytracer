use nalgebra::Vector3;

// #[derive(Copy, Clone)]
// pub enum Texture {
//     SolidColor {
//         color: Vector3<f32>,
//     },
//     Checker {
//         odd: Vector3<f32>,
//         even: Vector3<f32>,
//     },
// }

pub trait Texture: Sync {
    fn value(&self, u: f32, v: f32, p: &Vector3<f32>) -> Vector3<f32>;
}

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
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
