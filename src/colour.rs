use crate::{Interval, Vector};

pub type Colour = Vector<f64, 3>;

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }
    0.0
}

pub fn write_colour(out: &mut String, pixel_colour: Colour, samples_per_pixel: u32) {
    let mut r = pixel_colour.x();
    let mut g = pixel_colour.y();
    let mut b = pixel_colour.z();

    let scale = 1.0 / samples_per_pixel as f64;
    // TODO: SIMD?
    r *= scale;
    g *= scale;
    b *= scale;

    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    static INTENSITY: Interval = Interval::new(0.000, 0.999);
    out.push_str(
        format!(
            "{} {} {}\n",
            (255. * INTENSITY.clamp(r)) as u8,
            (255. * INTENSITY.clamp(g)) as u8,
            (255. * INTENSITY.clamp(b)) as u8
        )
        .as_str(),
    )
}
