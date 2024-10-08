use linalg::Point;
use rand::random;
use rayon::prelude::*;
use std::io;
use std::io::Write;
use tqdm::Iter;

use crate::{
    colour::{write_colour, Colour},
    degrees_to_radians,
    hittable::{HitRecord, Hittable},
    ray::Ray,
    Interval, Vector,
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
    pub fn render(&mut self, mut file: impl Write, world: impl Hittable + Clone + 'static) {
        self.initialise();

        let mut data = String::new();

        // For logging
        let mut stderr = io::stderr();

        let _ = file.write_fmt(format_args!("P3\n{} {}\n255\n", self.width, self.height));

        let height = self.height;
        let width = self.width;
        let samples_per_pixel = self.samples_per_pixel;
        let max_depth = self.max_depth;

        for j in (0..height).tqdm() {
            for i in 0..width {
                let pixel_colour: Colour = (0..samples_per_pixel)
                    .into_par_iter()
                    .map(|_| {
                        let ray = self.get_ray(i, j);
                        Self::ray_colour(ray, max_depth, &world)
                    })
                    .collect::<Vec<Colour>>()
                    .iter()
                    .fold(Colour::zero(), |acc, c| acc + *c);
                write_colour(&mut data, pixel_colour, samples_per_pixel);
            }
        }

        file.write_all(data.as_bytes())
            .expect("Could not write to file");
        let _ = stderr.write(b"\rDone.                  \n");
    }

    fn initialise(&mut self) {
        // Calculate the image height, and ensure that it's at least 1.
        self.height = (self.width as f64 / self.aspect_ratio) as u32;
        if self.height < 1 {
            self.height = 1;
        }

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

    fn ray_colour(ray: Ray, depth: u32, world: &dyn Hittable) -> Colour {
        if depth == 0 {
            return Colour::new([0., 0., 0.]);
        }

        let mut record = HitRecord::default();
        // 0.001 is used rather than zero to prevent shadow acne
        if world.hit(&ray, Interval::new(0.001, f64::INFINITY), &mut record) {
            let mut attenuation = Colour::new([1., 1., 1.]);
            let mut scattered = Ray::default();
            if record
                .material
                .scatter(&ray, &record, &mut attenuation, &mut scattered)
            {
                let col_pt_2 = Camera::ray_colour(scattered, depth - 1, world);
                let col = attenuation.hadamard(col_pt_2);

                return col;
            }
            return Colour::new([0., 0., 0.]);
        }

        let unit_direction = ray.direction().unit();
        let a = (unit_direction.y() + 1.0) * 0.5;

        Colour::new([1.0, 1.0, 1.0]) * (1.0 - a) + Colour::new([0.5, 0.7, 1.0]) * a
    }

    // Get a randomly sampled camera ray for te pixel at location i,j
    fn get_ray(&self, i: u32, j: u32) -> Ray {
        // Constructs a camera ray originating from the defocus disk and directed at a randomly
        // sampled point around the pixel location i, j.

        let pixel_centre =
            self.pixel00_loc + (self.pixel_delta_u * i as f64) + (self.pixel_delta_v * j as f64);
        let pixel_sample = pixel_centre + self.pixel_sample_square();

        let ray_origin = if self.defocus_angle <= 0. {
            self.centre
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = random::<f64>();

        Ray::new(ray_origin, ray_direction, Some(ray_time))
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
