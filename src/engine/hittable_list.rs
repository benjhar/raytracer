use std::sync::Arc;

use crate::{bounding_volume_hierarchies::aabb::AABB, util::interval::Interval};

use super::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
};

#[derive(Clone, Default)]
pub struct HittableList<O: Clone + Hittable> {
    pub objects: Vec<Arc<O>>,
    bbox: AABB,
}

impl<O: Clone + Hittable> HittableList<O> {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bbox: AABB::default(),
        }
    }

    pub fn from_object(object: Arc<O>) -> Self {
        Self {
            objects: vec![object.clone()],
            bbox: object.bounding_box(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Arc<O>) {
        self.objects.push(object.clone());
        self.bbox = AABB::enclosing(self.bbox, object.bounding_box());
    }
}

impl<O: Clone + Hittable> Hittable for HittableList<O> {
    fn hit(&self, ray: &Ray, ray_t: Interval, record: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if object.hit(ray, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.distance;
                *record = temp_rec.clone();
            }
        }

        hit_anything
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
