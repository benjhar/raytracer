use std::{cmp::Ordering, sync::Arc};

use crate::{
    engine::{
        hittable::{HitRecord, Hittable},
        hittable_list::HittableList,
        ray::Ray,
    },
    util::interval::Interval,
};

use super::aabb::AABB;

#[derive(Clone)]
pub struct BVHNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: AABB,
}

impl BVHNode {
    fn box_compare<O: Clone + Hittable>(a: &Arc<O>, b: &Arc<O>, axis_index: usize) -> Ordering {
        let a_bb = a.bounding_box();
        let a_axis_interval = a_bb.axis_interval(axis_index);
        let b_bb = b.bounding_box();
        let b_axis_interval = b_bb.axis_interval(axis_index);
        a_axis_interval
            .min
            .partial_cmp(&b_axis_interval.min)
            .unwrap()
    }

    fn box_x_compare<O: Clone + Hittable>(a: &Arc<O>, b: &Arc<O>) -> Ordering {
        Self::box_compare(a, b, 0)
    }
    fn box_y_compare<O: Clone + Hittable>(a: &Arc<O>, b: &Arc<O>) -> Ordering {
        Self::box_compare(a, b, 1)
    }
    fn box_z_compare<O: Clone + Hittable>(a: &Arc<O>, b: &Arc<O>) -> Ordering {
        Self::box_compare(a, b, 2)
    }

    pub fn new<O: Clone + Hittable + 'static>(
        mut objects: Vec<Arc<O>>,
        start: usize,
        end: usize,
    ) -> Self {
        let mut bbox = AABB::empty();
        for object in &objects {
            bbox = AABB::enclosing(bbox, object.bounding_box());
        }

        let axis = bbox.longest_axis();
        // let mut rng = rand::thread_rng();
        // let axis = rng.gen_range(0..=2);

        let comparator = match axis {
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            _ => Self::box_z_compare,
        };

        let object_span = end - start;

        let (left, right) = if object_span == 1 {
            (
                objects[start].clone() as Arc<dyn Hittable>,
                objects[start].clone() as Arc<dyn Hittable>,
            )
        } else if object_span == 2 {
            (
                objects[start].clone() as Arc<dyn Hittable>,
                objects[start + 1].clone() as Arc<dyn Hittable>,
            )
        } else {
            objects.sort_by(comparator);

            let mid = start + object_span / 2;
            (
                Arc::new(BVHNode::new(objects.clone(), start, mid)) as Arc<dyn Hittable>,
                Arc::new(BVHNode::new(objects.clone(), mid, end)) as Arc<dyn Hittable>,
            )
        };

        let bbox = AABB::enclosing(left.bounding_box(), right.bounding_box());

        Self { left, right, bbox }
    }

    pub fn build<O: Clone + Hittable + 'static>(list: HittableList<O>) -> Self {
        let len = list.objects.len();
        Self::new(list.objects, 0, len)
    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, ray_t: Interval, record: &mut HitRecord) -> bool {
        if !self.bbox.hit(ray, ray_t) {
            return false;
        }

        let hit_left = self.left.hit(ray, ray_t, record);
        let hit_right = self.right.hit(
            ray,
            Interval::new(
                ray_t.min,
                if hit_left { record.distance } else { ray_t.max },
            ),
            record,
        );

        hit_left || hit_right
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
