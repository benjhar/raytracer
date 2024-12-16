use linalg::{vector::Vector, Point};
use raytracer::{
    engine::{camera::Camera, hittable_list::HittableList},
    materials::Lambertian,
    surface::Sphere,
    textures::Checker,
    util::colour::Colour,
};
use std::sync::Arc;

fn main() -> Result<(), image::ImageError> {
    let mut world = HittableList::default();

    let checker = Arc::new(Checker::from_colours(
        0.01,
        &Colour::new([0.2, 0.3, 0.1]),
        &Colour::new([0.9, 0.9, 0.9]),
    ));

    let sphere_mat = Arc::new(Lambertian::new(checker.clone()));

    world.add(Arc::new(Sphere::new(
        Point::new([0., -10., 0.]),
        None,
        10.,
        sphere_mat.clone(),
    )));
    world.add(Arc::new(Sphere::new(
        Point::new([0., 10., 0.]),
        None,
        10.,
        sphere_mat,
    )));

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.vfov = 20.;
    cam.lookfrom = Point::new([13., 2., 3.]);
    cam.lookat = Point::new([0., 0., 0.]);
    cam.vup = Vector::new([0., 1., 0.]);

    cam.defocus_angle = 0.;
    cam.focus_dist = 10.;

    cam.render("checkered_spheres.png", world)
}
