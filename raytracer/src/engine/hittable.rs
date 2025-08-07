use std::sync::Arc;

use linalg::{vector::Vector, Point};
use rand::random;

use crate::{
    bounding_volume_hierarchies::aabb::AABB,
    degrees_to_radians,
    materials::{Isotropic, Lambertian, Material},
    textures::Texture,
    util::{colour::Colour, interval::Interval},
};

use super::ray::Ray;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point<f64, 3>,
    pub normal: Vector<f64, 3>,
    pub material: Arc<dyn Material>,
    pub distance: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            p: Point::default(),
            normal: Vector::default(),
            material: Arc::new(Lambertian::from_colour(Colour::new([1.; 3]))),
            distance: 0.0,
            u: 0.,
            v: 0.,
            front_face: true,
        }
    }
}

impl HitRecord {
    /// Sets the hit normal vector.
    /// NOTE: the parameter `outward_normal` is assumed to have unit length.
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vector<f64, 3>) {
        self.front_face = ray.direction().dot(outward_normal) < 0.;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, ray_t: Interval, record: &mut HitRecord) -> bool;

    fn bounding_box(&self) -> AABB;
}

pub struct Translate {
    object: Arc<dyn Hittable>,
    offset: Vector<f64, 3>,
}

impl Translate {
    pub fn new(object: Arc<dyn Hittable>, offset: Vector<f64, 3>) -> Self {
        Self { object, offset }
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, ray_t: Interval, record: &mut HitRecord) -> bool {
        let offset_r = Ray::new(
            ray.origin() - self.offset,
            ray.direction(),
            Some(ray.time()),
        );

        if !self.object.hit(&offset_r, ray_t, record) {
            return false;
        }

        record.p = record.p + self.offset;

        true
    }

    fn bounding_box(&self) -> AABB {
        self.object.bounding_box() + self.offset
    }
}

pub struct RotateY {
    object: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: AABB,
}

impl RotateY {
    pub fn new(object: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = object.bounding_box();

        let mut min = [f64::INFINITY; 3];
        let mut max = [f64::NEG_INFINITY; 3];

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.x.max + (1 - i) as f64 * bbox.x.min;
                    let y = j as f64 * bbox.y.max + (1 - j) as f64 * bbox.y.min;
                    let z = k as f64 * bbox.z.max + (1 - k) as f64 * bbox.z.min;

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    let tester = Vector::new([new_x, y, new_z]);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        let bbox = AABB::build(Point::new(min), Point::new(max));

        Self {
            object,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, ray_t: Interval, record: &mut HitRecord) -> bool {
        // Transform the ray from world space to object space

        let origin = Point::new([
            (self.cos_theta * ray.origin().x()) - (self.sin_theta * ray.origin().z()),
            ray.origin().y(),
            (self.sin_theta * ray.origin().x()) + (self.cos_theta * ray.origin().z()),
        ]);

        let direction = Vector::new([
            (self.cos_theta * ray.direction().x()) - (self.sin_theta * ray.direction().z()),
            ray.direction().y(),
            (self.sin_theta * ray.direction().x()) + (self.cos_theta * ray.direction().z()),
        ]);

        let rotated_r = Ray::new(origin, direction, Some(ray.time()));

        // Determine whether an intersection exists in object space (and if so, where).

        if !self.object.hit(&rotated_r, ray_t, record) {
            return false;
        }

        // Transform the intersection from object space back to world space.

        record.p = Point::new([
            (self.cos_theta * record.p.x()) + (self.sin_theta * record.p.z()),
            record.p.y(),
            (-self.sin_theta * record.p.x()) + (self.cos_theta * record.p.z()),
        ]);

        record.normal = Vector::new([
            (self.cos_theta * record.normal.x()) + (self.sin_theta * record.normal.z()),
            record.normal.y(),
            (-self.sin_theta * record.normal.x()) + (self.cos_theta * record.normal.z()),
        ]);

        true
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Arc<dyn Material>,
}

impl ConstantMedium {
    pub fn new(boundary: Arc<dyn Hittable>, density: f64, texture: Arc<dyn Texture>) -> Self {
        let phase_function = Arc::new(Isotropic::new(texture));
        let neg_inv_density = -1. / density;

        Self {
            boundary,
            neg_inv_density,
            phase_function,
        }
    }

    pub fn from_colour(boundary: Arc<dyn Hittable>, density: f64, colour: Colour) -> Self {
        let phase_function = Arc::new(Isotropic::from_colour(colour));
        let neg_inv_density = -1. / density;

        Self {
            boundary,
            neg_inv_density,
            phase_function,
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, ray_t: Interval, record: &mut HitRecord) -> bool {
        let mut rec1 = HitRecord::default();
        let mut rec2 = HitRecord::default();

        if !self.boundary.hit(ray, Interval::universe(), &mut rec1) {
            return false;
        }

        if !self.boundary.hit(
            ray,
            Interval::new(rec1.distance + 0.0001, f64::INFINITY),
            &mut rec2,
        ) {
            return false;
        }

        if rec1.distance < ray_t.min {
            rec1.distance = ray_t.min;
        }
        if rec2.distance > ray_t.max {
            rec1.distance = ray_t.max;
        }

        if rec1.distance >= rec2.distance {
            return false;
        }

        if rec1.distance < 0. {
            rec1.distance = 0.;
        }

        let ray_length = ray.direction().length();
        let distance_inside_boundary = (rec2.distance - rec1.distance) * ray_length;
        let hit_distance = self.neg_inv_density * random::<f64>().log10();

        if hit_distance > distance_inside_boundary {
            return false;
        }

        record.distance = rec1.distance + hit_distance / ray_length;
        record.p = ray.at(record.distance);

        record.normal = Vector::new([1., 0., 0.]); // Arbitrary
        record.front_face = true; // also arbitrary
        record.material = self.phase_function.clone();

        true
    }

    fn bounding_box(&self) -> AABB {
        self.boundary.bounding_box()
    }
}
