use std::sync::Arc;

use crate::{
    bounding_volume_hierarchies::aabb::AABB,
    engine::{hittable, hittable_list::HittableList},
    materials::Material,
    util::interval::Interval,
};
use linalg::{vector::Vector, Point};

pub struct Quad {
    q: Point<f64, 3>,
    u: Vector<f64, 3>,
    v: Vector<f64, 3>,
    material: Arc<dyn Material>,
    bbox: AABB,
    normal: Vector<f64, 3>,
    d: f64,
    w: Vector<f64, 3>,
}

unsafe impl Send for Quad {}
unsafe impl Sync for Quad {}

impl Quad {
    pub fn new(
        q: Point<f64, 3>,
        u: Vector<f64, 3>,
        v: Vector<f64, 3>,
        material: Arc<dyn Material>,
    ) -> Self {
        let bbox_diagonal1 = AABB::build(q, q + u + v);
        let bbox_diagonal2 = AABB::build(q + u, q + v);
        let bbox = AABB::enclosing(bbox_diagonal1, bbox_diagonal2);
        let n = u.cross(v);
        let normal = n.unit();
        let d = normal.dot(&q);
        let w = n / n.dot(&n);

        Self {
            q,
            u,
            v,
            material,
            bbox,
            normal,
            d,
            w,
        }
    }

    fn is_interior(a: f64, b: f64, record: &mut hittable::HitRecord) -> bool {
        let unit_interval = Interval::new(0.0, 1.0);
        // Given the hit point in plane coordinates, return false if it is outside the primitive,
        // otherwise set the hit record UV coordinates and return true.

        if !unit_interval.contains(a) || !unit_interval.contains(b) {
            return false;
        }

        record.u = a;
        record.v = b;
        true
    }
}

impl hittable::Hittable for Quad {
    fn bounding_box(&self) -> AABB {
        self.bbox
    }

    fn hit(
        &self,
        ray: &crate::engine::ray::Ray,
        ray_t: crate::util::interval::Interval,
        record: &mut hittable::HitRecord,
    ) -> bool {
        let denom = self.normal.dot(&ray.direction());

        // No hit if the ray is parallel to the plane.
        if denom.abs() < 1e-8 {
            return false;
        }

        // Return false if the hit point parameter t is outside the ray interval.
        let t = (self.d - self.normal.dot(&ray.origin())) / denom;
        if !ray_t.contains(t) {
            return false;
        }

        let intersection = ray.at(t);
        let planar_hit_point_vector = intersection - self.q;
        let alpha = self.w.dot(&planar_hit_point_vector.cross(self.v));
        let beta = self.w.dot(&self.u.cross(planar_hit_point_vector));

        if !Self::is_interior(alpha, beta, record) {
            return false;
        }

        record.distance = t;
        record.p = intersection;
        record.material = self.material.clone();
        record.set_face_normal(ray, &self.normal);

        true
    }
}

pub fn r#box(
    a: &Point<f64, 3>,
    b: &Point<f64, 3>,
    material: Arc<dyn Material>,
) -> Arc<HittableList> {
    let mut sides = HittableList::new();

    let min = Point::new([a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z())]);
    let max = Point::new([a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z())]);

    let dx = Vector::new([max.x() - min.x(), 0., 0.]);
    let dy = Vector::new([0., max.y() - min.y(), 0.]);
    let dz = Vector::new([0., 0., max.z() - min.z()]);

    sides.add(Arc::new(Quad::new(
        Point::new([min.x(), min.y(), max.z()]),
        dx,
        dy,
        material.clone(),
    )));
    sides.add(Arc::new(Quad::new(
        Point::new([max.x(), min.y(), max.z()]),
        -dz,
        dy,
        material.clone(),
    )));
    sides.add(Arc::new(Quad::new(
        Point::new([max.x(), min.y(), min.z()]),
        -dx,
        dy,
        material.clone(),
    )));
    sides.add(Arc::new(Quad::new(
        Point::new([min.x(), min.y(), min.z()]),
        dz,
        dy,
        material.clone(),
    )));
    sides.add(Arc::new(Quad::new(
        Point::new([min.x(), max.y(), max.z()]),
        dx,
        -dz,
        material.clone(),
    )));
    sides.add(Arc::new(Quad::new(
        Point::new([min.x(), min.y(), min.z()]),
        dx,
        dz,
        material,
    )));

    Arc::new(sides)
}
