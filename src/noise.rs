use crate::{hit::random_int, material::random_unit_vector};
use core::array::from_fn;
use nalgebra::Vector3;

#[derive(Copy, Clone)]
pub struct Perlin {
    ranvec: [Vector3<f32>; Perlin::POINT_COUNT],
    perm_x: [i16; Perlin::POINT_COUNT],
    perm_y: [i16; Perlin::POINT_COUNT],
    perm_z: [i16; Perlin::POINT_COUNT],
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn new() -> Perlin {
        Perlin {
            ranvec: Self::perlin_generate_vec(),
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        }
    }

    pub fn turb(&self, p: &Vector3<f32>, depth: u8) -> f32 {
        let mut accum = 0.0;
        let mut temp_p = p.clone();
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }

    fn noise(&self, p: &Vector3<f32>) -> f32 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as i16;
        let j = p.y.floor() as i16;
        let k = p.z.floor() as i16;

        let mut c = [Vector3::new(0.0, 0.0, 0.0); 8];

        let mut q = 0;
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[q] = self.ranvec[(self.perm_x[((i + di) & 255) as usize]
                        ^ self.perm_y[((j + dj) & 255) as usize]
                        ^ self.perm_z[((k + dk) & 255) as usize])
                        as usize];
                    q += 1;
                }
            }
        }

        Self::trilinear_interp(&c, u, v, w)
    }

    fn trilinear_interp(c: &[Vector3<f32>], u: f32, v: f32, w: f32) -> f32 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        let mut accum = 0.0;
        let mut q = 0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vector3::new(u - i as f32, v - j as f32, w - k as f32);
                    accum += (i as f32 * uu + (1 - i) as f32 * (1.0 - uu))
                        * (j as f32 * vv + (1 - j) as f32 * (1.0 - vv))
                        * (k as f32 * ww + (1 - k) as f32 * (1.0 - ww))
                        * c[q].dot(&weight_v);
                    q += 1;
                }
            }
        }

        accum
    }

    fn permute(p: &mut [i16], n: usize) {
        for i in (0..n).rev() {
            let target = random_int(0, i as u32) as usize;
            p.swap(i, target);
        }
    }

    fn perlin_generate_perm() -> [i16; Perlin::POINT_COUNT] {
        let mut p: [i16; Perlin::POINT_COUNT] = from_fn(|i| i as i16);
        Perlin::permute(&mut p, Perlin::POINT_COUNT);
        p
    }

    fn perlin_generate_vec() -> [Vector3<f32>; Perlin::POINT_COUNT] {
        from_fn(|_i| random_unit_vector())
    }
}
