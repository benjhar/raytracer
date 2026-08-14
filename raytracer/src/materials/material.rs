use crate::{
    engine::{hittable::HitRecord, ray::Ray},
    util::colour::Colour,
};

use linalg::Point;

pub trait Material: Sync {
    fn emitted(&self, _u: f64, _v: f64, _point: &Point<f64, 3>) -> Colour {
        Colour::zero()
    }

    fn scatter(
        &self,
        _ray_in: &Ray,
        _record: &HitRecord,
        _attenuation: &mut Colour,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }
}
