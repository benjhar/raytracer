use std::ops::Add;

use linalg::{vector::Vector, Point};

use crate::{engine::ray::Ray, util::interval::Interval};

#[derive(Default, Clone, Copy)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    #[must_use]
    pub const fn new(x: Interval, y: Interval, z: Interval) -> Self {
        let mut aabb = Self { x, y, z };
        aabb.pad_to_minimum();
        aabb
    }

    /// Treat the two points a and b as extrema for the bounding box, so we don't require a
    /// particular minimum/maximum coordinate order
    #[must_use]
    pub fn build(north: Point<f64, 3>, south: Point<f64, 3>) -> Self {
        let x = if north.x() <= south.x() {
            Interval::new(north.x(), south.x())
        } else {
            Interval::new(south.x(), north.x())
        };

        let y = if north.y() <= south.y() {
            Interval::new(north.y(), south.y())
        } else {
            Interval::new(south.y(), north.y())
        };

        let z = if north.z() <= south.z() {
            Interval::new(north.z(), south.z())
        } else {
            Interval::new(south.z(), north.z())
        };

        let mut aabb = Self { x, y, z };
        aabb.pad_to_minimum();
        aabb
    }

    #[must_use]
    pub fn enclosing(box0: Self, box1: Self) -> Self {
        let x = Interval::enclosing(box0.x, box1.x);
        let y = Interval::enclosing(box0.y, box1.y);
        let z = Interval::enclosing(box0.z, box1.z);

        Self { x, y, z }
    }

    pub const fn axis_interval(&self, n: usize) -> &Interval {
        if n == 1 {
            return &self.y;
        }
        if n == 2 {
            return &self.z;
        }
        &self.x
    }

    #[must_use]
    pub fn hit(&self, r: &Ray, mut ray_t: Interval) -> bool {
        let ray_orig = r.origin();
        let ray_dir = r.direction();

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1. / ray_dir[axis];

            let t0 = (ax.min - ray_orig[axis]) * adinv;
            let t1 = (ax.max - ray_orig[axis]) * adinv;

            if t0 < t1 {
                if t0 > ray_t.min {
                    ray_t.min = t0;
                }
                if t1 < ray_t.max {
                    ray_t.max = t1;
                }
            } else {
                if t1 > ray_t.min {
                    ray_t.min = t1;
                }
                if t0 < ray_t.max {
                    ray_t.max = t0;
                }
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }

        true
    }

    #[must_use]
    pub const fn universe() -> Self {
        Self::new(
            Interval::universe(),
            Interval::universe(),
            Interval::universe(),
        )
    }

    #[must_use]
    pub const fn empty() -> Self {
        Self::new(Interval::empty(), Interval::empty(), Interval::empty())
    }

    #[must_use]
    pub fn longest_axis(&self) -> u8 {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() {
                0
            } else {
                2
            }
        } else if self.y.size() > self.z.size() {
            1
        } else {
            2
        }
    }

    /// Adjust the AABB so that no side is narrower than some delta, padding if necessary.
    const fn pad_to_minimum(&mut self) {
        let delta: f64 = 0.0001;
        if self.x.size() < delta {
            self.x = self.x.expand(delta);
        }
        if self.y.size() < delta {
            self.y = self.y.expand(delta);
        }
        if self.z.size() < delta {
            self.z = self.z.expand(delta);
        }
    }
}

impl Add<Vector<f64, 3>> for AABB {
    type Output = Self;
    #[expect(
        clippy::arithmetic_side_effects,
        reason = "Side effects are expected in general arithmetic operations"
    )]
    fn add(self, rhs: Vector<f64, 3>) -> Self::Output {
        Self::new(self.x + rhs.x(), self.y + rhs.y(), self.z + rhs.z())
    }
}
