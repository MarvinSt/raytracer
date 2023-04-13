use crate::hit::World;
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

        // Self {
        //     ori: origin,
        //     dir: direction,
        // }
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

    pub fn color(&self, background: &Vector3<f32>, world: &World, depth: u8) -> Vector3<f32> {
        if depth <= 0 {
            // exceeded depth count, no light remaining
            return Vector3::zeros();
        }

        // test object collision
        match world.hit(self, 0.001, f32::MAX) {
            Some(hit) => {
                // get the emitted color
                let emitted = hit.m.emitted(hit.u, hit.v, &hit.p);

                // test for scattering
                match hit.m.scatter(self, &hit) {
                    Some((attenuation, scattered)) => {
                        let color: Vector3<f32> = scattered.color(background, &world, depth - 1);
                        emitted + attenuation.component_mul(&color)
                    }
                    None => emitted,
                }
            }
            None => *background,
        }
    }
}
