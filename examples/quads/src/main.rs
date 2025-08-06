use std::sync::Arc;

use linalg::{vector::Vector, Point};
use raytracer::{
    engine::{camera::Camera, hittable_list::HittableList},
    materials::Lambertian,
    surface::Quad,
    util::colour::Colour,
};

pub fn main() -> Result<(), image::ImageError> {
    let mut world = HittableList::new();

    let left_red = Arc::new(Lambertian::from_colour(Colour::new([1.0, 0.2, 0.2])));
    let back_green = Arc::new(Lambertian::from_colour(Colour::new([0.2, 1.0, 0.2])));
    let right_blue = Arc::new(Lambertian::from_colour(Colour::new([0.2, 0.2, 1.0])));
    let upper_orange = Arc::new(Lambertian::from_colour(Colour::new([1.0, 0.5, 0.])));
    let lower_teal = Arc::new(Lambertian::from_colour(Colour::new([0.2, 0.8, 0.8])));

    world.add(Arc::new(Quad::new(
        Point::new([-3., -2., 5.]),
        Vector::new([0., 0., -4.]),
        Vector::new([0., 4., 0.]),
        left_red,
    )));
    world.add(Arc::new(Quad::new(
        Point::new([-2., -2., 0.]),
        Vector::new([4., 0., 0.]),
        Vector::new([0., 4., 0.]),
        back_green,
    )));
    world.add(Arc::new(Quad::new(
        Point::new([3., -2., 1.]),
        Vector::new([0., 0., 4.]),
        Vector::new([0., 4., 0.]),
        right_blue,
    )));
    world.add(Arc::new(Quad::new(
        Point::new([-2., 3., 1.]),
        Vector::new([4., 0., 0.]),
        Vector::new([0., 0., 4.]),
        upper_orange,
    )));
    world.add(Arc::new(Quad::new(
        Point::new([-2., -3., 5.]),
        Vector::new([4., 0., 0.]),
        Vector::new([0., 0., -4.]),
        lower_teal,
    )));

    let mut cam = Camera::default();

    cam.aspect_ratio = 1.0;
    cam.width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.vfov = 80.;
    cam.lookfrom = Point::new([0., 0., 9.]);
    cam.lookat = Point::new([0.; 3]);
    cam.vup = Vector::new([0., 1., 0.]);

    cam.defocus_angle = 0.;
    cam.focus_dist = 2.;
    cam.background = Colour::new([0.7, 0.8, 1.0]);

    cam.render("quads.png", world)
}
