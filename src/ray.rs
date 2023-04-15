use crate::{hit::Hittable, material::ScatterRecord, pdf::Pdf};
use nalgebra::Vector3;

#[derive(Default, Copy, Clone)]
pub struct Ray {
    pub ori: Vector3<f32>,
    pub dir: Vector3<f32>,
    pub nrm_dir: Vector3<f32>,
    pub inv_dir: Vector3<f32>,
    pub sign_x: usize,
    pub sign_y: usize,
    pub sign_z: usize,
}

impl Ray {
    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>) -> Self {
        let nrm_dir = direction.normalize();
        Ray {
            ori: origin,
            dir: direction,
            nrm_dir: nrm_dir,
            inv_dir: Vector3::new(1.0 / direction.x, 1.0 / direction.y, 1.0 / direction.z),
            sign_x: (direction.x < 0.0) as usize,
            sign_y: (direction.y < 0.0) as usize,
            sign_z: (direction.z < 0.0) as usize,
        }
    }

    // pub fn origin(&self) -> Vector3<f32> {
    //     self.ori
    // }

    // pub fn direction(&self) -> Vector3<f32> {
    //     self.dir
    // }

    pub fn point_at(&self, t: f32) -> Vector3<f32> {
        self.ori + t * self.dir
    }

    pub fn color(
        &self,
        background: &Vector3<f32>,
        world: &Box<dyn Hittable>,
        lights: &Box<dyn Hittable>,
        depth: u8,
    ) -> Vector3<f32> {
        if depth <= 0 {
            // exceeded depth count, no light remaining
            return Vector3::new(0.0, 0.0, 0.0);
        }

        // test object collision
        match world.hit(self, 0.001, f32::MAX) {
            None => *background,
            Some(hit) => {
                // get the emitted color
                let emitted = hit.m.emitted(&hit);

                // get scatter record
                match hit.m.scatter(self, &hit) {
                    None => emitted,
                    Some(ScatterRecord::Scatter { attenuation, pdf }) => {
                        let light_pdf = Pdf::hittable_pdf(lights, &hit.p);
                        let mixed_pdf = Pdf::mixture_pdf(&light_pdf, &pdf);
                        let direction = mixed_pdf.generate();
                        let scattered = Ray::new(hit.p, direction);
                        let pdf_val = mixed_pdf.value(scattered.dir);
                        let scattering_pdf = hit.m.scattering_pdf(&hit, &scattered);

                        let color: Vector3<f32> =
                            scattered.color(background, &world, &lights, depth - 1);

                        emitted + attenuation.component_mul(&color) * scattering_pdf / pdf_val
                    }
                    Some(ScatterRecord::Specular {
                        attenuation,
                        specular_ray,
                    }) => {
                        let color: Vector3<f32> =
                            specular_ray.color(background, &world, &lights, depth - 1);

                        attenuation.component_mul(&color)
                    }
                    Some(ScatterRecord::Isotropic {
                        attenuation,
                        scattered_ray,
                    }) => {
                        let color: Vector3<f32> =
                            scattered_ray.color(background, &world, &lights, depth - 1);

                        emitted + attenuation.component_mul(&color)
                    }
                }
            }
        }
    }

    /*
    pub fn color_iter(
        &self,
        background: &Vector3<f32>,
        world: &World,
        max_depth: u8,
    ) -> Vector3<f32> {
        let mut cur_ray = *self;

        let mut color = Vector3::new(0.0, 0.0, 0.0);
        let mut global_attenuation = Vector3::new(1.0, 1.0, 1.0);

        for _depth in 0..max_depth {
            match world.hit(&cur_ray, 0.001, f32::MAX) {
                None => return color + background.component_mul(&global_attenuation),
                Some(hit) => {
                    // get the emitted color
                    let emitted = hit.m.emitted(&hit);
                    color = color + emitted.component_mul(&global_attenuation);

                    // test for scattering
                    match hit.m.scatter(&cur_ray, &hit) {
                        None => return color,
                        Some((attenuation, scattered, pdf)) => {
                            // update global attenuation based on current scattered attenuation
                            // global_attenuation.component_mul_assign(&attenuation);
                            global_attenuation = global_attenuation.component_mul(&attenuation);

                            // update current ray
                            cur_ray = scattered;
                        }
                    }
                }
            }
        }

        color
    }
     */
}
