use std::sync::Arc;

use crate::{colour::Colour, solid_colour::SolidColour, texture::Texture};

pub struct CheckerTexture {
    inv_scale: f64,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

unsafe impl Send for CheckerTexture {}
unsafe impl Sync for CheckerTexture {}

impl CheckerTexture {
    pub fn new(scale: f64, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1. / scale,
            even,
            odd,
        }
    }

    pub fn from_colours(scale: f64, c1: &Colour, c2: &Colour) -> Self {
        Self::new(
            scale,
            Arc::new(SolidColour::new(*c1)),
            Arc::new(SolidColour::new(*c2)),
        )
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &linalg::Point<f64, 3>) -> Colour {
        // let xint = (self.inv_scale * p.x()).floor() as i32;
        // let yint = (self.inv_scale * p.y()).floor() as i32;
        // let zint = (self.inv_scale * p.z()).floor() as i32;
        //
        // let is_even = (xint + yint + zint) % 2 == 0;

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
