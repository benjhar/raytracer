use linalg::{vector::Vector, Point};

#[derive(Default, Clone, Copy)]
pub struct Ray {
    origin: Point<f64, 3>,
    direction: Vector<f64, 3>,
    time: f64,
}

impl Ray {
    pub fn new(origin: Point<f64, 3>, direction: Vector<f64, 3>, time: Option<f64>) -> Self {
        Ray {
            origin,
            direction,
            time: time.unwrap_or(0.0),
        }
    }

    pub fn origin(&self) -> Point<f64, 3> {
        self.origin
    }

    pub fn direction(&self) -> Vector<f64, 3> {
        self.direction
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn at(&self, t: f64) -> Point<f64, 3> {
        self.origin + self.direction * t
    }
}
