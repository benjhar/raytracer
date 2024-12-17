use crate::{
    engine::{hittable::HitRecord, ray::Ray},
    util::colour::Colour,
};

pub trait Material {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Colour,
        scattered: &mut Ray,
    ) -> bool;
}
