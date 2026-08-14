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

unsafe impl Sync for BVHNode {}

impl BVHNode {
    fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis_index: usize) -> Ordering {
        let a_bb = a.bounding_box();
        let a_axis_interval = a_bb.axis_interval(axis_index);
        let b_bb = b.bounding_box();
        let b_axis_interval = b_bb.axis_interval(axis_index);
        a_axis_interval
            .min
            .partial_cmp(&b_axis_interval.min)
            .unwrap_or(Ordering::Equal)
    }

    fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 0)
    }
    fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 1)
    }
    fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 2)
    }

    pub fn new(objects: &mut [Arc<dyn Hittable>]) -> Self {
        let mut bbox = AABB::empty();
        for object in &*objects {
            bbox = AABB::enclosing(bbox, object.bounding_box());
        }

        let axis = bbox.longest_axis();

        let comparator = match axis {
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            _ => Self::box_z_compare,
        };

        let object_span = objects.len();

        let (left, right) = if let [obj] = objects {
            (obj.clone(), obj.clone())
        } else if let [obj1, obj2] = objects {
            (obj1.clone(), obj2.clone())
        } else {
            objects.sort_by(comparator);

            let mid = 0usize.midpoint(object_span);
            let (arr_left, arr_right) = objects.split_at_mut(mid);

            let (left, right): (Arc<dyn Hittable>, Arc<dyn Hittable>) = (
                Arc::new(Self::new(arr_left)),
                Arc::new(Self::new(arr_right)),
            );
            (left, right)
        };

        let bbox = AABB::enclosing(left.bounding_box(), right.bounding_box());

        Self { left, right, bbox }
    }

    #[must_use]
    pub fn build(mut list: HittableList) -> Self {
        Self::new(&mut list.objects)
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
