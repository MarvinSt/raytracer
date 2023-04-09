use crate::bounding_box::AABB;
use crate::hit::*;
use crate::ray::Ray;
use std::cmp::Ordering;

/*
Probably better to implement this alogithn:
https://www.pbr-book.org/3ed-2018/Primitives_and_Intersection_Acceleration/Bounding_Volume_Hierarchies#fragment-BVHAccelPrivateData-1
*/

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

    // fn axis_range(hittable: &Vec<Box<dyn Hittable>>, axis: usize) -> f32 {
    //     let (min, max) = hittable
    //         .iter()
    //         .fold((f32::MAX, f32::MIN), |(bmin, bmax), hit| {
    //             if let Some(aabb) = hit.bounding_box() {
    //                 (bmin.min(aabb.min()[axis]), bmax.max(aabb.max()[axis]))
    //             } else {
    //                 (bmin, bmax)
    //             }
    //         });
    //     max - min
    // }

    pub fn new(mut hittable: Vec<Box<dyn Hittable>>) -> Self {
        // let mut axis_ranges: Vec<(usize, f32)> = (0..3)
        //     .map(|a| (a, Self::axis_range(&hittable, a)))
        //     .collect();

        // let mut aabb = hittable.iter().reduce(|a, b| AABB::surrounding_box(a.bounding_box().unwrap(), b.bounding_box().unwrap()))
        // let mut aabb = AABB::default();
        // for object in hittable {
        //     aabb = aabb.extend_box(&object.bounding_box().unwrap());
        // }

        // axis_ranges.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        // let axis = axis_ranges[0].0;

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
        let aabb = self.aabb.hit(r, t_min, t_max);
        match aabb {
            None => None,
            Some(_aabb) => match &self.tree {
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
        }
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(self.aabb)
    }
}
