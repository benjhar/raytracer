use image::RgbImage;
use linalg::vector::Vector;
use linalg::Point;
use rand::random;
use rayon::prelude::*;
use std::num::NonZeroU32;
use std::path::Path;

use crate::util::colour::{write_colour, Colour};
use crate::util::interval::Interval;

use super::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
};

pub struct CameraSettings {
    pub width: NonZeroU32,
    pub height: NonZeroU32,
    pub samples_per_pixel: NonZeroU32,
    pub max_depth: NonZeroU32,
    pub vfov: f64,
    pub lookfrom: Point<f64, 3>,
    pub lookat: Point<f64, 3>,
    pub vup: Vector<f64, 3>,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    pub background: Colour,
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            width: unsafe { NonZeroU32::new_unchecked(480) },
            height: unsafe { NonZeroU32::new_unchecked(360) },
            samples_per_pixel: unsafe { NonZeroU32::new_unchecked(20) },
            max_depth: unsafe { NonZeroU32::new_unchecked(15) },
            vfov: 40.,
            lookfrom: Vector::zero(),
            lookat: Point::new([1., 1., 0.]),
            vup: Vector::new([0., 1., 0.]),
            defocus_angle: 0.,
            focus_dist: 25.,
            background: Colour::zero(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Camera {
    pub width: NonZeroU32,
    pub height: NonZeroU32,
    pub samples_per_pixel: NonZeroU32,
    pub max_depth: NonZeroU32,
    pub vfov: f64,
    pub lookfrom: Point<f64, 3>,
    pub lookat: Point<f64, 3>,
    pub vup: Vector<f64, 3>,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    pub background: Colour,
    pixel_samples_scale: f64,
    sqrt_spp: NonZeroU32,
    recip_sqrt_spp: f64,
    centre: Point<f64, 3>,
    pixel_delta_v: Vector<f64, 3>,
    pixel_delta_u: Vector<f64, 3>,
    pixel00_loc: Point<f64, 3>,
    defocus_disk_u: Vector<f64, 3>,
    defocus_disk_v: Vector<f64, 3>,
}

impl Camera {
    #[must_use]
    pub fn new(settings: &CameraSettings) -> Self {
        let CameraSettings {
            width,
            height,
            samples_per_pixel,
            max_depth,
            vfov,
            lookfrom,
            lookat,
            vup,
            defocus_angle,
            focus_dist,
            background,
        } = *settings;

        let width_f64 = nzu32_f64_cast(width);
        let height_f64 = nzu32_f64_cast(height);

        let sqrt_spp;
        #[expect(
            clippy::as_conversions,
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss,
            reason = "f64 is created from a u32, then sqrt so must necessarily be smaller. Cannot be signed."
        )]
        {
            sqrt_spp = NonZeroU32::new(nzu32_f64_cast(samples_per_pixel).sqrt() as u32)
                .unwrap_or(NonZeroU32::MIN);
        }
        let pixel_samples_scale = nzu32_f64_cast(sqrt_spp).powi(2).recip();
        let recip_sqrt_spp = nzu32_f64_cast(sqrt_spp).recip();

        let centre = lookfrom;

        // Determine viewport dimensions
        let theta = vfov.to_radians();
        let h = (theta / 2.).tan();
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (width_f64 / height_f64);

        // Calculate the u,v,w basis vectors for the camera coordinate frame
        let w = (lookfrom - lookat).unit();
        let u = vup.cross(w).unit();
        let v = w.cross(u);

        // Calculate vectors across horizontal and down vertical viewport edges
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        // Calculate horizontal and vertical delta vectors from pixel to pixel
        let pixel_delta_u = viewport_u / width_f64;
        let pixel_delta_v = viewport_v / height_f64;

        // Calculate the location of the upper left pixel;
        let viewport_upper_left = centre - (focus_dist * w) - viewport_u / 2. - viewport_v / 2.;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        let defocus_radius = focus_dist * (defocus_angle.to_radians() / 2.).tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self {
            width,
            height,
            samples_per_pixel,
            max_depth,
            vfov,
            lookfrom,
            lookat,
            vup,
            defocus_angle,
            focus_dist,
            background,
            pixel_samples_scale,
            sqrt_spp,
            recip_sqrt_spp,
            centre,
            pixel_delta_v,
            pixel_delta_u,
            pixel00_loc,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    /// Renders `world` into an image, saved at `filename`.
    ///
    /// # Errors
    /// Returns an error if the resulting image could not be saved.
    #[expect(
        clippy::arithmetic_side_effects,
        reason = "Vector<f64, 3> does not cause side-effects"
    )]
    pub fn render(
        &self,
        #[allow(clippy::needless_pass_by_value)] filename: impl AsRef<Path>,
        world: &(impl Hittable + Clone + 'static),
        mut image: RgbImage,
    ) -> Result<(), image::ImageError> {
        let max_depth = self.max_depth;

        image.par_enumerate_pixels_mut().for_each(|(i, j, pixel)| {
            let mut pixel_colour = Colour::zero();
            for s_i in 0..self.sqrt_spp.into() {
                for s_j in 0..self.sqrt_spp.into() {
                    let ray = self.get_ray(i, j, s_i, s_j);
                    pixel_colour = pixel_colour + self.ray_colour(ray, max_depth, world);
                }
            }

            *pixel = write_colour(self.pixel_samples_scale * pixel_colour);
        });

        image.save(filename)
    }

    fn ray_colour(&self, ray: Ray, depth: NonZeroU32, world: &impl Hittable) -> Colour {
        let mut record = HitRecord::default();
        // 0.001 is used rather than zero to prevent shadow acne
        if !world.hit(&ray, Interval::new(0.001, f64::INFINITY), &mut record) {
            return self.background;
        }

        let mut scattered: Ray = Ray::default();
        let mut attenuation: Colour = Colour::one();
        let colour_from_emission = record.material.emitted(record.u, record.v, &record.p);

        if !record
            .material
            .scatter(&ray, &record, &mut attenuation, &mut scattered)
        {
            return colour_from_emission;
        }

        let col_pt_2 = if depth == NonZeroU32::MIN {
            Colour::zero()
        } else {
            let depth_u32: u32 = depth.into();
            self.ray_colour(
                scattered,
                NonZeroU32::new(depth_u32.saturating_sub(1)).unwrap_or(NonZeroU32::MIN),
                world,
            )
        };
        let colour_from_scatter = attenuation.hadamard(col_pt_2);

        #[expect(
            clippy::arithmetic_side_effects,
            reason = "Vector<f64, 3> does not cause side-effects"
        )]
        {
            colour_from_emission + colour_from_scatter
        }
    }

    // Get a randomly sampled camera ray for te pixel at location i,j
    #[expect(
        clippy::arithmetic_side_effects,
        reason = "Vector<f64, 3> does not cause side-effects"
    )]
    fn get_ray(&self, i: u32, j: u32, s_i: u32, s_j: u32) -> Ray {
        // Constructs a camera ray originating from the defocus disk and directed at a randomly
        // sampled point around the pixel location i, j.

        let offset = self.sample_square_stratified(s_i, s_j);
        let pixel_sample = self.pixel00_loc
            + ((f64::from(i) + offset.x()) * self.pixel_delta_u)
            + ((f64::from(j) + offset.y()) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0. {
            self.centre
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = random::<f64>();

        Ray::new(ray_origin, ray_direction, Some(ray_time))
    }

    /// Returns the vector to a random point in the square sub-pixel specified by grid indices
    /// `s_i` and `s_j`, for an idealized unit square pixel [-.5,-.5] to [+.5,+.5]
    fn sample_square_stratified(&self, s_i: u32, s_j: u32) -> Vector<f64, 3> {
        let px = (f64::from(s_i) + random::<f64>()).mul_add(self.recip_sqrt_spp, -0.5);
        let py = (f64::from(s_j) + random::<f64>()).mul_add(self.recip_sqrt_spp, -0.5);

        Vector::new([px, py, 0.])
    }

    /// Returns a random point in the camera defocus disk
    #[expect(
        clippy::arithmetic_side_effects,
        reason = "Vector<f64, 3> does not cause side-effects"
    )]
    fn defocus_disk_sample(&self) -> Point<f64, 3> {
        let p = Vector::random_in_unit_disk();
        self.centre + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
    }
}

fn nzu32_f64_cast(nz: NonZeroU32) -> f64 {
    let nz_u32: u32 = nz.into();
    nz_u32.into()
}
