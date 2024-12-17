use std::{f64::consts::PI, sync::Arc};

use linalg::{vector::Vector, Point};

use crate::{
    bounding_volume_hierarchies::aabb::AABB,
    engine::{
        hittable::{HitRecord, Hittable},
        ray::Ray,
    },
    materials::{Lambertian, Material},
    util::{colour::Colour, interval::Interval},
};

#[derive(Clone)]
pub struct Sphere {
    centre: Ray,
    radius: f64,
    material: Arc<dyn Material>,
    bbox: AABB,
}

unsafe impl Send for Sphere {}
unsafe impl Sync for Sphere {}

impl Sphere {
    pub fn new(centre: Point<f64, 3>, radius: f64, material: Arc<dyn Material>) -> Self {
        let rvec = Vector::new([radius; 3]);
        let bbox = AABB::build(centre - rvec, centre + rvec);
        Self {
            centre: Ray::new(centre, Vector::new([0., 0., 0.]), None),
            radius,
            material,
            bbox,
        }
    }

    pub fn moving(
        centre: Point<f64, 3>,
        centre2: Point<f64, 3>,
        radius: f64,
        material: Arc<dyn Material>,
    ) -> Self {
        let centre = Ray::new(centre, centre2 - centre, None);

        let rvec = Vector::new([radius; 3]);
        let box1 = AABB::build(centre.at(0.) - rvec, centre.at(0.) + rvec);
        let box2 = AABB::build(centre.at(1.) - rvec, centre.at(1.) + rvec);
        let bbox = AABB::enclosing(box1, box2);

        Self {
            centre,
            radius,
            material,
            bbox,
        }
    }

    fn get_uv(p: &Point<f64, 3>, u: &mut f64, v: &mut f64) {
        // p: a given point on the sphere of radius one, centered at the origin.
        // u: returned value [0,1] of angle around the Y axis from X=-1.
        // v: returned value [0,1] of angle from Y=-1 to Y=+1.
        //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
        //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
        //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>

        let neg_p = Vector::zero() - *p;
        let theta = neg_p.y().acos();
        let phi = neg_p.z().atan2(p.x()) + PI;

        *u = phi / (2. * PI);
        *v = theta / PI;
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere {
            centre: Ray::default(),
            radius: 0.0,
            material: Arc::new(Lambertian::from_colour(Colour::new([0.8; 3]))),
            bbox: AABB::default(),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval, record: &mut HitRecord) -> bool {
        let current_centre = self.centre.at(ray.time());
        let oc = ray.origin() - current_centre;
        let a = ray.direction().length_squared();
        let half_b = oc.dot(&ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0. {
            return false;
        }
        let sqrt_disc = discriminant.sqrt();

        let mut root = (-half_b - sqrt_disc) / a;

        if !ray_t.surrounds(root) {
            root = (-half_b + sqrt_disc) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        record.distance = root;
        record.p = ray.at(record.distance);
        let outward_normal = (record.p - current_centre) / self.radius;
        record.set_face_normal(ray, &outward_normal);
        Self::get_uv(&outward_normal, &mut record.u, &mut record.v);
        record.material = self.material.clone();

        true
    }
    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

impl Material for Sphere {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Colour,
        scattered: &mut Ray,
    ) -> bool {
        self.material
            .scatter(ray_in, record, attenuation, scattered)
    }
}
