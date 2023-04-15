use crate::bounding_box::AABB;
use crate::hit::*;
use crate::ray::Ray;
use std::cmp::Ordering;

#[derive(Debug)]
struct Node {
    aabb: AABB,
    child_index: usize,
    min_index: usize,
    max_index: usize,
}

pub struct Bvh {
    nodes: Vec<Node>,
    objects: Vec<Box<dyn Hittable>>,
}

impl Node {
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

    fn new(objects: &mut Vec<Box<dyn Hittable>>, min_index: usize, max_index: usize) -> Self {
        // Find the total bounding box of all hittable objects in the current node
        let aabb = objects[min_index..max_index]
            .iter()
            .fold(AABB::default(), |a, b| {
                a.extend_box(&b.bounding_box().unwrap())
            });

        // Determine the largest axis
        let axis = aabb.max_axis();

        objects[min_index..max_index].sort_by(Self::aabb_compare(axis));

        Node {
            aabb,
            child_index: 0,
            min_index,
            max_index,
        }
    }
}

impl Bvh {
    pub fn new(hittable: Vec<Box<dyn Hittable>>) -> Bvh {
        let objects = hittable;
        let count = objects.len();

        let mut bvh = Bvh {
            objects: objects,
            nodes: vec![],
        };

        // Generate the root node
        bvh.nodes.push(Node::new(&mut bvh.objects, 0, count));

        // Recursively build the node tree, by constantly splitting the nodes
        let mut node_idx = 0;
        while node_idx < bvh.nodes.len() {
            bvh.split_node(node_idx);
            node_idx += 1;
        }

        bvh
    }

    fn split_node(&mut self, node_index: usize) {
        let min_index = self.nodes[node_index].min_index;
        let max_index = self.nodes[node_index].max_index;

        if (max_index - min_index) < 2 {
            return;
        }

        let mid_index = min_index + (max_index - min_index) / 2;

        // Calculate the child inxdex of the current node
        // This is the index of the left node; the right node is the next index
        let child_index = self.nodes.len();

        // Build and push the left side node
        self.nodes
            .push(Node::new(&mut self.objects, min_index, mid_index));

        // Build and push the right side node
        self.nodes
            .push(Node::new(&mut self.objects, mid_index, max_index));

        // Set the current node child index
        self.nodes[node_index].child_index = child_index;
    }
}

impl Hittable for Bvh {
    fn hit(&self, r: &Ray, t_min: f32, mut t_max: f32) -> Option<HitRecord> {
        // let mut hitlist = vec![];
        let mut hit: Option<HitRecord> = None;

        // hitlist.push(0);

        let mut stack: [usize; 64] = [0; 64];
        let mut size: usize = 1;

        while size > 0 {
            size -= 1;
            let node = &self.nodes[stack[size]];
            // let node = &self.nodes[hitlist.pop().unwrap()];

            if node.child_index == 0 {
                // object hit test
                for i in node.min_index..node.max_index {
                    match self.objects[i].hit(r, t_min, t_max) {
                        Some(h) => {
                            hit = Some(h);
                            t_max = h.t;
                        }
                        None => {}
                    }
                }
            } else {
                if self.nodes[node.child_index + 0].aabb.hit(r, t_min, t_max) {
                    // hitlist.push(node.child_index + 0);
                    stack[size] = node.child_index + 0;
                    size += 1;
                }
                if self.nodes[node.child_index + 1].aabb.hit(r, t_min, t_max) {
                    // hitlist.push(node.child_index + 1);
                    stack[size] = node.child_index + 1;
                    size += 1;
                }
            }
        }

        hit
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(self.nodes[0].aabb)
    }
}
