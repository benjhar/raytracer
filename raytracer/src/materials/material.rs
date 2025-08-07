use crate::{
    engine::{hittable::HitRecord, ray::Ray},
    util::colour::Colour,
};

use linalg::Point;

pub trait Material: Send + Sync {
    fn emitted(&self, u: f64, v: f64, point: &Point<f64, 3>) -> Colour {
        Colour::zero()
    }

    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Colour,
        scattered: &mut Ray,
    ) -> bool {
        false
    }
}
