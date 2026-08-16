use std::sync::Arc;

use linalg::{vector::Vector, Point};

use crate::{
    bounding_volume_hierarchies::aabb::AABB,
    materials::{Isotropic, Lambertian, Material},
    textures::Texture,
    thread_rng,
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
        Self {
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
    #[expect(
        clippy::arithmetic_side_effects,
        reason = "Vector<f64, 3> does not cause side-effects"
    )]
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vector<f64, 3>) {
        self.front_face = ray.direction().dot(outward_normal) < 0.;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

pub trait Hittable: Sync {
    fn hit(&self, ray: &Ray, ray_t: Interval, record: &mut HitRecord) -> bool;

    fn bounding_box(&self) -> AABB;
}

pub struct Translate {
    object: Arc<dyn Hittable>,
    offset: Vector<f64, 3>,
}

unsafe impl Sync for Translate {}

impl Translate {
    pub fn new(object: Arc<dyn Hittable>, offset: Vector<f64, 3>) -> Self {
        Self { object, offset }
    }
}

impl Hittable for Translate {
    #[expect(
        clippy::arithmetic_side_effects,
        reason = "Vector<f64, 3> does not cause side-effects"
    )]
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

    #[expect(
        clippy::arithmetic_side_effects,
        reason = "Vector<f64, 3> does not cause side-effects"
    )]
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

unsafe impl Sync for RotateY {}

impl RotateY {
    pub fn new(object: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = object.bounding_box();

        let mut min = [f64::INFINITY; 3];
        let mut max = [f64::NEG_INFINITY; 3];

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = f64::from(i)
                        .mul_add(bbox.x.max, f64::from(1i32.saturating_sub(i)) * bbox.x.min);
                    let y = f64::from(j)
                        .mul_add(bbox.y.max, f64::from(1i32.saturating_sub(j)) * bbox.y.min);
                    let z = f64::from(k)
                        .mul_add(bbox.z.max, f64::from(1i32.saturating_sub(k)) * bbox.z.min);

                    let new_x = cos_theta.mul_add(x, sin_theta * z);
                    let new_z = cos_theta.mul_add(z, -sin_theta * x);

                    let tester = [new_x, y, new_z];

                    for (min_item, (max_item, tester_item)) in
                        min.iter_mut().zip(max.iter_mut().zip(tester.iter()))
                    {
                        *min_item = min_item.min(*tester_item);
                        *max_item = max_item.max(*tester_item);
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
            self.cos_theta
                .mul_add(ray.origin().x(), -(self.sin_theta * ray.origin().z())),
            ray.origin().y(),
            self.cos_theta
                .mul_add(ray.origin().z(), self.sin_theta * ray.origin().x()),
        ]);

        let direction = Vector::new([
            self.cos_theta
                .mul_add(ray.direction().x(), -(self.sin_theta * ray.direction().z())),
            ray.direction().y(),
            self.cos_theta
                .mul_add(ray.direction().z(), self.sin_theta * ray.direction().x()),
        ]);

        let rotated_r = Ray::new(origin, direction, Some(ray.time()));

        // Determine whether an intersection exists in object space (and if so, where).

        if !self.object.hit(&rotated_r, ray_t, record) {
            return false;
        }

        // Transform the intersection from object space back to world space.

        record.p = Point::new([
            self.cos_theta
                .mul_add(record.p.x(), self.sin_theta * record.p.z()),
            // (self.cos_theta * record.p.x()) + (self.sin_theta * record.p.z()),
            record.p.y(),
            self.cos_theta
                .mul_add(record.p.z(), -self.sin_theta * record.p.x()),
        ]);

        record.normal = Vector::new([
            self.cos_theta
                .mul_add(record.normal.x(), self.sin_theta * record.normal.z()),
            record.normal.y(),
            self.cos_theta
                .mul_add(record.normal.z(), -self.sin_theta * record.normal.x()),
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

unsafe impl Sync for ConstantMedium {}

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
        let hit_distance = self.neg_inv_density * thread_rng().f64_inclusive().log10();

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
