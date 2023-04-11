use crate::hit::World;
use nalgebra::Vector3;

#[derive(Default, Copy, Clone)]
pub struct Ray {
    pub ori: Vector3<f32>,
    pub dir: Vector3<f32>,
}

impl Ray {
    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>) -> Self {
        Self {
            ori: origin,
            dir: direction,
        }
    }

    pub fn origin(&self) -> Vector3<f32> {
        self.ori
    }

    pub fn direction(&self) -> Vector3<f32> {
        self.dir
    }

    pub fn point_at(&self, t: f32) -> Vector3<f32> {
        self.ori + t * self.dir
    }

    pub fn color(&self, background: &Vector3<f32>, world: &World, depth: u8) -> Vector3<f32> {
        let mut attenuation: Vector3<f32> = Vector3::new(0.0, 0.0, 0.0);

        if depth <= 0 {
            return attenuation;
        }

        // test object collision
        match world.hit(self, 0.001, 1.0e8) {
            Some(hit) => {
                let mut scattered: Ray = Ray::default();

                // get the emitted color
                let emitted = hit.m.emitted(hit.u, hit.v, &hit.p);

                // test for scattering
                if !hit.m.scatter(self, &hit, &mut attenuation, &mut scattered) {
                    return emitted;
                }

                // calculate scattered color
                let color: Vector3<f32> = scattered.color(background, &world, depth - 1);
                return emitted + attenuation.component_mul(&color);
            }
            None => *background,
        }
    }
}

pub fn ray_color(ray: &Ray, background: &Vector3<f32>, world: &World, depth: u8) -> Vector3<f32> {
    let mut attenuation: Vector3<f32> = Vector3::new(0.0, 0.0, 0.0);

    if depth <= 0 {
        return attenuation;
    }

    // test object collision
    match world.hit(&ray, 0.001, 1.0e8) {
        Some(hit) => {
            let mut scattered: Ray = Ray::default();

            // get the emitted color
            let emitted = hit.m.emitted(hit.u, hit.v, &hit.p);

            // test for scattering
            if !hit.m.scatter(&ray, &hit, &mut attenuation, &mut scattered) {
                return emitted;
            }

            // calculate scattered color
            let color: Vector3<f32> = ray_color(&scattered, background, &world, depth - 1);
            return emitted + attenuation.component_mul(&color);
        }
        None => {
            // background
            // let n: Vector3<f32> = ray.direction().normalize();
            // let t = 0.5 * (n.y + 1.0);
            // let env_color: Vector3<f32> =
            //     (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0);

            *background
        }
    }
}
