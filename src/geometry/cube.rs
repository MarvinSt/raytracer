use super::rectangle::RectAA;
use crate::bounding_box::AABB;
use crate::hit::*;
use crate::material::Material;
use crate::ray::Ray;
use nalgebra::Vector3;

pub struct Cube {
    box_min: Vector3<f32>,
    box_max: Vector3<f32>,
    sides: World,
}

impl Cube {
    pub fn new<M: Material + Clone + 'static>(
        p0: Vector3<f32>,
        p1: Vector3<f32>,
        material: M,
    ) -> Self {
        let mut sides = World::new();

        sides.add(Box::new(RectAA::xy(
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p1.z,
            material.clone(),
        )));
        sides.add(Box::new(RectAA::xy(
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p0.z,
            material.clone(),
        )));

        sides.add(Box::new(RectAA::xz(
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p1.y,
            material.clone(),
        )));
        sides.add(Box::new(RectAA::xz(
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p0.y,
            material.clone(),
        )));

        sides.add(Box::new(RectAA::yz(
            p0.y,
            p1.y,
            p0.z,
            p1.z,
            p1.x,
            material.clone(),
        )));
        sides.add(Box::new(RectAA::yz(
            p0.y,
            p1.y,
            p0.z,
            p1.z,
            p0.x,
            material.clone(),
        )));

        Self {
            box_min: p0,
            box_max: p1,
            sides: sides,
        }
    }
}

impl Hittable for Cube {
    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB::new(self.box_min, self.box_max))
    }

    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }
}
