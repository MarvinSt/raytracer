use crate::{hit::Hittable, material::random_cosine_direction};
use nalgebra::Vector3;
use rand::Rng;

pub struct Onb {
    u: Vector3<f32>,
    v: Vector3<f32>,
    w: Vector3<f32>,
}

impl Onb {
    pub fn local(&self, a: &Vector3<f32>) -> Vector3<f32> {
        return a.x * self.u + a.y * self.v + a.z * self.w;
    }

    pub fn build_from_w(n: Vector3<f32>) -> Self {
        let w = n.normalize();
        let a = if w.x.abs() > 0.9 {
            Vector3::new(0.0, 1.0, 0.0)
        } else {
            Vector3::new(1.0, 0.0, 0.0)
        };
        let v = w.cross(&a).normalize();
        let u = w.cross(&v);

        Self { u, v, w }
    }
}

pub enum Pdf<'a> {
    Cosine {
        uvw: Onb,
    },
    Hittable {
        origin: Vector3<f32>,
        hittable: &'a Box<dyn Hittable>,
    },
    Mixture {
        p: &'a Pdf<'a>,
        q: &'a Pdf<'a>,
    },
}

impl<'a> Pdf<'a> {
    pub fn cosine_pdf(w: Vector3<f32>) -> Pdf<'a> {
        let uvw = Onb::build_from_w(w);
        Pdf::Cosine { uvw }
    }

    pub fn hittable_pdf(hittable: &'a Box<dyn Hittable>, origin: &Vector3<f32>) -> Self {
        Pdf::Hittable {
            origin: *origin,
            hittable,
        }
    }

    pub fn mixture_pdf(p: &'a Pdf, q: &'a Pdf) -> Self {
        Pdf::Mixture { p, q }
    }

    pub fn value(&self, direction: Vector3<f32>) -> f32 {
        match self {
            Pdf::Cosine { uvw } => {
                let cosine = f32::max(direction.normalize().dot(&uvw.w), 0.0);
                cosine / std::f32::consts::PI
            }
            Pdf::Hittable { origin, hittable } => hittable.pdf_value(*origin, direction),
            Pdf::Mixture { p, q } => 0.5 * p.value(direction) + 0.5 * q.value(direction),
        }
    }

    pub fn generate(&self) -> Vector3<f32> {
        match self {
            Pdf::Cosine { uvw } => uvw.local(&random_cosine_direction()),
            Pdf::Hittable { origin, hittable } => hittable.random(*origin),
            Pdf::Mixture { p, q } => {
                let mut rng = rand::thread_rng();
                if rng.gen::<bool>() {
                    p.generate()
                } else {
                    q.generate()
                }
            }
        }
    }
}
