use linalg::{vector::Vector, Point};

#[derive(Default, Clone, Copy)]
pub struct Ray {
    origin: Point<f64, 3>,
    direction: Vector<f64, 3>,
}

impl Ray {
    pub fn new(origin: Point<f64, 3>, direction: Vector<f64, 3>) -> Self {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> Point<f64, 3> {
        self.origin
    }

    pub fn direction(&self) -> Vector<f64, 3> {
        self.direction
    }

    pub fn at(&self, t: f64) -> Point<f64, 3> {
        self.origin + self.direction * t
    }
}
