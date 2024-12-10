use std::sync::Arc;

use crate::util::colour::Colour;

use super::{Solid, Texture};

pub struct Checker {
    inv_scale: f64,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

unsafe impl Send for Checker {}
unsafe impl Sync for Checker {}

impl Checker {
    pub fn new(scale: f64, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1. / scale,
            even,
            odd,
        }
    }

    pub fn from_colours(scale: f64, c1: &Colour, c2: &Colour) -> Self {
        Self::new(scale, Arc::new(Solid::new(*c1)), Arc::new(Solid::new(*c2)))
    }
}

impl Texture for Checker {
    fn value(&self, u: f64, v: f64, p: &linalg::Point<f64, 3>) -> Colour {
        let uint = (self.inv_scale * u).floor() as i32;
        let vint = (self.inv_scale * v).floor() as i32;

        let is_even = (uint + vint) % 2 == 0;

        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}
