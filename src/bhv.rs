use std::sync::Arc;

use nalgebra::Vector3;

use crate::bounding_box::AABB;
use crate::hit::*;
use crate::ray::Ray;

/*
Probably better to implement this alogithn:
https://www.pbr-book.org/3ed-2018/Primitives_and_Intersection_Acceleration/Bounding_Volume_Hierarchies#fragment-BVHAccelPrivateData-1
 */

#[derive(Copy, Clone, Debug)]
pub enum SplitMethod {
    Middle,
    EqualCounts,
    SAH,
}

enum Axis {
    X,
    Y,
    Z,
}

enum BVHNodeData {
    Interior {
        second_child_offset: usize,
        axis: Axis,
    },
    Leaf {
        primitives_offset: usize,
        num_prims: usize,
    },
}

// #[derive(Debug)]
struct BVHNode {
    aabb: AABB,
    data: BVHNodeData,
}

enum BVHBuildNode {
    Interior {
        bounds: AABB,
        children: [Box<BVHBuildNode>; 2],
        split_axis: Axis,
    },
    Leaf {
        bounds: AABB,
        first_prim_offset: usize,
        num_prims: usize,
    },
}

pub struct BVH {
    nodes: Vec<BVHNode>,
    primitives: Vec<Arc<dyn Hittable>>,
}

struct BVHPrimitiveInfo {
    pub prim_number: usize,
    pub bounds: AABB,
}

impl BVHPrimitiveInfo {
    fn new(pn: usize, bb: AABB) -> BVHPrimitiveInfo {
        BVHPrimitiveInfo {
            prim_number: pn,
            bounds: bb,
        }
    }
}

// #[inline]
fn box_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>, axis: usize) -> std::cmp::Ordering {
    if let Some(box_a) = a.bounding_box() {
        if let Some(box_b) = b.bounding_box() {
            if box_a.min().data.0[axis] < box_b.min().data.0[axis] {
                return std::cmp::Ordering::Less;
            } else {
                return std::cmp::Ordering::Greater;
            }
        }
    }
    std::cmp::Ordering::Greater
}

/*
fn recursiveBuild(primitives,
    std::vector<BVHPrimitiveInfo> &primitiveInfo, int start,
    int end, int *totalNodes,
    std::vector<std::shared_ptr<Primitive>> &mut orderedPrims: Primitive[]) {
BVHBuildNode *node = arena.Alloc<BVHBuildNode>();
(*totalNodes)++;
    // Compute bounds of all primitives in BVH node
   Bounds3f bounds;
   for (int i = start; i < end; ++i)
       bounds = Union(bounds, primitiveInfo[i].bounds);

int nPrimitives = end - start;
if (nPrimitives == 1) {
    // Create leaf BVHBuildNode
       int firstPrimOffset = orderedPrims.size();
       for (int i = start; i < end; ++i) {
           int primNum = primitiveInfo[i].primitiveNumber;
           orderedPrims.push_back(primitives[primNum]);
       }
       node->InitLeaf(firstPrimOffset, nPrimitives, bounds);
       return node;

} else {
    // Compute bound of primitive centroids, choose split dimension dim>>
       Bounds3f centroidBounds;
       for (int i = start; i < end; ++i)
           centroidBounds = Union(centroidBounds, primitiveInfo[i].centroid);
       int dim = centroidBounds.MaximumExtent();

    // Partition primitives into two sets and build children>>
       int mid = (start + end) / 2;
       if (centroidBounds.pMax[dim] == centroidBounds.pMin[dim]) {
           // Create leaf BVHBuildNode>>
       } else {
           // Partition primitives based on splitMethod>>
           node->InitInterior(dim,
                              recursiveBuild(arena, primitiveInfo, start, mid,
                                             totalNodes, orderedPrims),
                              recursiveBuild(arena, primitiveInfo, mid, end,
                                             totalNodes, orderedPrims));
       }

}
return node;
}

 */

impl BVH {
    fn recursive_build(
        primitives: &[Arc<dyn Hittable>],
        primitive_info: &mut Vec<BVHPrimitiveInfo>,
        start: usize,
        end: usize,
        max_prims_per_node: usize,
        total_nodes: &mut usize,
        ordered_prims: &mut Vec<Arc<dyn Hittable>>,
        split_method: SplitMethod,
    ) -> BVHBuildNode {
        BVHBuildNode::Leaf {
            bounds: AABB::default(),
            first_prim_offset: 0,
            num_prims: 0,
        }
    }

    fn build(primitives: &[Arc<dyn Hittable>]) {
        let max_prims_per_node = 5;
        let split_method = SplitMethod::Middle;

        // get primitive info structure
        let mut primitive_info: Vec<BVHPrimitiveInfo> = primitives
            .iter()
            .enumerate()
            .map(|(i, p)| BVHPrimitiveInfo::new(i, p.bounding_box().unwrap()))
            .collect();

        // build tree
        let mut total_nodes = 0;
        let mut ordered_prims = Vec::with_capacity(primitives.len());
        let root: BVHBuildNode = BVH::recursive_build(
            primitives,
            &mut primitive_info,
            0usize,
            primitives.len(),
            max_prims_per_node,
            &mut total_nodes,
            &mut ordered_prims,
            split_method,
        );
    }
}

/*
bvh_node::bvh_node(
    std::vector<shared_ptr<hittable>>& src_objects,
    size_t start, size_t end, double time0, double time1
) {
    auto objects = src_objects; // Create a modifiable array of the source scene objects

    int axis = random_int(0,2);
    auto comparator = (axis == 0) ? box_x_compare
                    : (axis == 1) ? box_y_compare
                                  : box_z_compare;

    size_t object_span = end - start;

    if (object_span == 1) {
        left = right = objects[start];
    } else if (object_span == 2) {
        if (comparator(objects[start], objects[start+1])) {
            left = objects[start];
            right = objects[start+1];
        } else {
            left = objects[start+1];
            right = objects[start];
        }
    } else {
        std::sort(objects.begin() + start, objects.begin() + end, comparator);

        auto mid = start + object_span/2;
        left = make_shared<bvh_node>(objects, start, mid, time0, time1);
        right = make_shared<bvh_node>(objects, mid, end, time0, time1);
    }

    aabb box_left, box_right;

    if (  !left->bounding_box (time0, time1, box_left)
       || !right->bounding_box(time0, time1, box_right)
    )
        std::cerr << "No bounding box in bvh_node constructor.\n";

    box = surrounding_box(box_left, box_right);
}
 */

impl Hittable for BVH {
    fn bounding_box(&self) -> Option<AABB> {
        Some(self.nodes[0].aabb)
    }

    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if self.nodes.is_empty() {
            return None;
        }

        let mut nodes = Vec::new();
        let mut node_idx;
        let mut t_max = t_max;
        let mut cur_hit: Option<HitRecord> = None;

        // push the root node
        nodes.push(0);

        // let inv_dir: Vector3<f32> = Vector3::new(1.0 / r.dir.x, 1.0 / r.dir.y, 1.0 / r.dir.z);
        // let dir_is_neg = [
        //     (inv_dir.x < 0.0) as usize,
        //     (inv_dir.y < 0.0) as usize,
        //     (inv_dir.z < 0.0) as usize,
        // ];

        loop {
            if let Some(next) = nodes.pop() {
                node_idx = next;
            } else {
                break;
            }

            let hit = self.nodes[node_idx].aabb.hit(r, t_min, t_max);
            if hit.is_some() {
                match &self.nodes[node_idx].data {
                    BVHNodeData::Leaf {
                        num_prims,
                        primitives_offset,
                    } => {
                        for i in 0..*num_prims {
                            if let Some(rec) =
                                self.primitives[primitives_offset + i].hit(r, t_min, t_max)
                            {
                                t_max = rec.t;
                                cur_hit = Some(rec);
                            }
                        }
                    }
                    BVHNodeData::Interior {
                        second_child_offset,
                        axis,
                    } => {
                        // let axis_num = match axis {
                        //     Axis::X => 0,
                        //     Axis::Y => 1,
                        //     Axis::Z => 2,
                        // };
                        nodes.push(node_idx + 1);
                        nodes.push(*second_child_offset);
                    }
                }
            }
        }

        cur_hit
    }
}
