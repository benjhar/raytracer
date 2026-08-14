use linalg::{vector::Vector, Point};

#[derive(Default, Clone, Copy)]
pub struct Ray {
    origin: Point<f64, 3>,
    direction: Vector<f64, 3>,
    time: f64,
}

impl Ray {
    #[must_use]
    pub fn new(origin: Point<f64, 3>, direction: Vector<f64, 3>, time: Option<f64>) -> Self {
        Self {
            origin,
            direction,
            time: time.unwrap_or(0.0),
        }
    }

    pub const fn origin(&self) -> Point<f64, 3> {
        self.origin
    }

    pub const fn direction(&self) -> Vector<f64, 3> {
        self.direction
    }

    #[must_use]
    pub const fn time(&self) -> f64 {
        self.time
    }

    pub fn at(&self, t: f64) -> Point<f64, 3> {
        Point::new([
            self.direction.x().mul_add(t, self.origin.x()),
            self.direction.y().mul_add(t, self.origin.y()),
            self.direction.z().mul_add(t, self.origin.z()),
        ])
    }
}
