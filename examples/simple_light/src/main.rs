use std::sync::Arc;

use linalg::{vector::Vector, Point};
use raytracer::{
    engine::{camera::Camera, hittable_list::HittableList},
    materials::{DiffuseLight, Lambertian},
    surface::{Quad, Sphere},
    textures::Fractal,
    util::colour::Colour,
};

fn main() -> Result<(), image::ImageError> {
    let mut world = HittableList::new();

    let pertext = Arc::new(Fractal::with_seed(4, 1., 1., 7, 0.5, 2.));
    let permat = Arc::new(Lambertian::new(pertext));
    world.add(Arc::new(Sphere::new(
        Point::new([0., -1000., 0.]),
        1000.,
        permat.clone(),
    )));
    world.add(Arc::new(Sphere::new(Point::new([0., 2., 0.]), 2., permat)));

    let diffuse_light = Arc::new(DiffuseLight::from_colour(Colour::new([4., 4., 4.])));
    world.add(Arc::new(Sphere::new(
        Point::new([0., 7., 0.]),
        2.,
        diffuse_light.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point::new([3., 1., -2.]),
        Vector::new([2., 0., 0.]),
        Vector::new([0., 2., 0.]),
        diffuse_light,
    )));

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Colour::zero();

    cam.vfov = 20.;
    cam.lookfrom = Point::new([26., 3., 6.]);
    cam.lookat = Point::new([0., 2., 0.]);
    cam.vup = Vector::new([0., 1., 0.]);

    cam.defocus_angle = 0.;
    cam.focus_dist = 30.;

    cam.render("simple_light.png", world)
}
