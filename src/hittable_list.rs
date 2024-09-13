use std::sync::Arc;

use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    Interval,
};

#[derive(Default, Clone)]
pub struct HittableList<O: Clone + Default + Hittable> {
    pub objects: Vec<Arc<O>>,
}

impl<O: Clone + Default + Hittable> HittableList<O> {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn from_object(object: Arc<O>) -> Self {
        Self {
            objects: vec![object],
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Arc<O>) {
        self.objects.push(object);
    }
}

impl<O: Clone + Default + Material + Hittable> Hittable for HittableList<O> {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        ray_t: Interval,
        record: &mut crate::hittable::HitRecord,
    ) -> bool {
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
}
