use image::{DynamicImage, GenericImageView, ImageError, ImageReader, Rgba};

#[derive(Default)]
pub struct RtwImage {
    data: DynamicImage,
}

impl RtwImage {
    #[allow(clippy::needless_pass_by_value)]
    pub fn load(filename: impl ToString) -> Result<Self, ImageError> {
        Ok(Self {
            data: ImageReader::open(filename.to_string())?.decode()?,
        })
    }

    pub fn width(&self) -> u32 {
        self.data.width()
    }

    pub fn height(&self) -> u32 {
        self.data.height()
    }

    pub fn pixel_data(&self, i: u32, j: u32) -> Rgba<u8> {
        const MAGENTA: Rgba<u8> = Rgba([255, 0, 255, 0]);

        if self.width() * self.height() == 0 {
            return MAGENTA;
        }

        let x = i.clamp(0, self.width());
        let y = j.clamp(0, self.height());

        self.data.get_pixel(x, y)
    }
}
