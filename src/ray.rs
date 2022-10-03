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
}
