use std::f64;

use image::{ImageError, Rgba};

use crate::util::{colour::Colour, rtw_image::RtwImage};

use super::Texture;

pub struct Image {
    image: RtwImage,
}

impl Image {
    pub fn try_file(filename: impl ToString) -> Result<Self, ImageError> {
        Ok(Self {
            image: RtwImage::load(filename)?,
        })
    }
}

impl Texture for Image {
    fn value(&self, u: f64, v: f64, _: &linalg::Point<f64, 3>) -> Colour {
        let u = u.clamp(0., 1.);
        let v = 1. - v.clamp(0., 1.);

        let i = (u * self.image.width() as f64) as u32;
        let j = (v * self.image.height() as f64) as u32;
        let Rgba([r, g, b, _]) = self.image.pixel_data(i, j);

        let colour_scale = 1.0 / 255.0;
        Colour::new([r, g, b].map(f64::from)) * colour_scale
    }
}
