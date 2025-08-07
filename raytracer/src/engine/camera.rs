use image::ImageBuffer;
use linalg::vector::Vector;
use linalg::Point;
use rand::random;
use rayon::prelude::*;
use std::path::Path;

use crate::util::colour::{write_colour, Colour};
use crate::{degrees_to_radians, util::interval::Interval};

use super::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
};

#[derive(Default, Clone, Copy)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub vfov: f64,
    pub lookfrom: Point<f64, 3>,
    pub lookat: Point<f64, 3>,
    pub vup: Vector<f64, 3>,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    pub background: Colour,
    pixel_samples_scale: f64,
    sqrt_spp: u32,
    recip_sqrt_spp: f64,
    height: u32,
    centre: Point<f64, 3>,
    pixel_delta_v: Vector<f64, 3>,
    pixel_delta_u: Vector<f64, 3>,
    pixel00_loc: Point<f64, 3>,
    u: Vector<f64, 3>,
    v: Vector<f64, 3>,
    w: Vector<f64, 3>,
    defocus_disk_u: Vector<f64, 3>,
    defocus_disk_v: Vector<f64, 3>,
}

impl Camera {
    pub fn render(
        &mut self,
        #[allow(clippy::needless_pass_by_value)] filename: impl AsRef<Path>,
        world: impl Hittable + Clone + 'static,
    ) -> Result<(), image::ImageError> {
        self.initialise();

        let mut imgbuf = ImageBuffer::new(self.width, self.height);

        let max_depth = self.max_depth;

        imgbuf.par_enumerate_pixels_mut().for_each(|(i, j, pixel)| {
            let mut pixel_colour = Colour::zero();
            for s_i in 0..self.sqrt_spp {
                for s_j in 0..self.sqrt_spp {
                    let ray = self.get_ray(i, j, s_i, s_j);
                    pixel_colour = pixel_colour + self.ray_colour(ray, max_depth, &world);
                }
            }

            *pixel = write_colour(self.pixel_samples_scale * pixel_colour);
        });

        imgbuf.save(filename)
    }

    fn initialise(&mut self) {
        // Calculate the image height, and ensure that it's at least 1.
        self.height = (self.width as f64 / self.aspect_ratio) as u32;
        if self.height < 1 {
            self.height = 1;
        }

        self.sqrt_spp = f64::from(self.samples_per_pixel).sqrt() as u32;
        self.pixel_samples_scale = 1.0 / f64::from(self.sqrt_spp * self.sqrt_spp);
        self.recip_sqrt_spp = 1.0 / f64::from(self.sqrt_spp);

        self.centre = self.lookfrom;

        // Determine viewport dimensions
        let theta = degrees_to_radians(self.vfov);
        let h = (theta / 2.).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.width as f64 / self.height as f64);

        // Calculate the u,v,w basis vectors for the camera coordinate frame
        self.w = (self.lookfrom - self.lookat).unit();
        self.u = self.vup.cross(self.w).unit();
        self.v = self.w.cross(self.u);

        // Calculate vectors across horizontal and down vertical viewport edges
        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;

        // Calculate horizontal and vertical delta vectors from pixel to pixel
        self.pixel_delta_u = viewport_u / self.width as f64;
        self.pixel_delta_v = viewport_v / self.height as f64;

        // Calculate the location of the upper left pixel;
        let viewport_upper_left =
            self.centre - (self.focus_dist * self.w) - viewport_u / 2. - viewport_v / 2.;
        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;

        let defocus_radius = self.focus_dist * degrees_to_radians(self.defocus_angle / 2.).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    fn ray_colour(&self, ray: Ray, depth: u32, world: &dyn Hittable) -> Colour {
        if depth == 0 {
            return Colour::new([0., 0., 0.]);
        }

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

        let col_pt_2 = self.ray_colour(scattered, depth - 1, world);
        let colour_from_scatter = attenuation.hadamard(col_pt_2);

        colour_from_emission + colour_from_scatter
    }

    // Get a randomly sampled camera ray for te pixel at location i,j
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
        let px = ((f64::from(s_i) + random::<f64>()) * self.recip_sqrt_spp) - 0.5;
        let py = ((f64::from(s_j) + random::<f64>()) * self.recip_sqrt_spp) - 0.5;

        Vector::new([px, py, 0.])
    }

    /// Returns a random point in the square surrounding a pixel at the origin
    fn pixel_sample_square(&self) -> Vector<f64, 3> {
        let px = -0.5 * random::<f64>();
        let py = -0.5 * random::<f64>();
        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }

    /// Returns a random point in the camera defocus disk
    fn defocus_disk_sample(&self) -> Point<f64, 3> {
        let p = Vector::random_in_unit_disk();
        self.centre + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
    }
}
