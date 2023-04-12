use crate::bounding_box::AABB;
use crate::hit::*;
use crate::ray::Ray;
use std::cmp::Ordering;

enum BvhNode {
    Branch { left: Box<Bvh>, right: Box<Bvh> },
    Leaf(Box<dyn Hittable>),
}

pub struct Bvh {
    tree: BvhNode,
    aabb: AABB,
}

impl Bvh {
    fn aabb_compare(axis: usize) -> impl FnMut(&Box<dyn Hittable>, &Box<dyn Hittable>) -> Ordering {
        move |a, b| {
            if let (Some(a), Some(b)) = (a.bounding_box(), b.bounding_box()) {
                let ac = a.sort_value_axis(axis);
                let bc = b.sort_value_axis(axis);
                ac.partial_cmp(&bc).unwrap()
            } else {
                panic!["no bounding box in bvh node"]
            }
        }
    }

    pub fn new(mut hittable: Vec<Box<dyn Hittable>>) -> Self {
        // Find the total bounding box of all hittable objects in the current node
        let aabb = hittable.iter().fold(AABB::default(), |a, b| {
            a.extend_box(&b.bounding_box().unwrap())
        });

        // Determine the largest axis
        let axis = aabb.max_axis();

        hittable.sort_unstable_by(Self::aabb_compare(axis));
        let len = hittable.len();

        match len {
            0 => panic!["no elements in scene"],
            1 => {
                let leaf = hittable.pop().unwrap();
                match leaf.bounding_box() {
                    Some(aabb) => Bvh {
                        tree: BvhNode::Leaf(leaf),
                        aabb,
                    },
                    None => {
                        panic!["no bounding box in bvh node"]
                    }
                }
            }
            _ => {
                let right = Self::new(hittable.drain(len / 2..).collect());
                let left = Self::new(hittable);
                let aabb = AABB::surrounding_box(&left.aabb, &right.aabb);
                Bvh {
                    tree: BvhNode::Branch {
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                    aabb,
                }
            }
        }
    }
}

impl Hittable for Bvh {
    fn hit(&self, r: &Ray, t_min: f32, mut t_max: f32) -> Option<HitRecord> {
        match self.aabb.hit(r, t_min, t_max) {
            true => match &self.tree {
                BvhNode::Leaf(leaf) => leaf.hit(&r, t_min, t_max),
                BvhNode::Branch { left, right } => {
                    let left = left.hit(&r, t_min, t_max);
                    if let Some(l) = &left {
                        t_max = l.t
                    };
                    let right = right.hit(&r, t_min, t_max);
                    if right.is_some() {
                        right
                    } else {
                        left
                    }
                }
            },
            false => return None,
        }
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(self.aabb)
    }
}
