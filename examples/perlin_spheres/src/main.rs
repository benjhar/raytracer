use linalg::{vector::Vector, Point};
use raytracer::{
    engine::{camera::Camera, hittable_list::HittableList},
    materials::Lambertian,
    surface::Sphere,
    textures::Fractal,
    util::colour::Colour,
};
use std::sync::Arc;

fn main() -> Result<(), image::ImageError> {
    let mut world = HittableList::new();

    let pertext = Arc::new(Fractal::with_seed(0, 1., 4., 7, 0.5, 2.));
    let permat = Arc::new(Lambertian::new(pertext));
    world.add(Arc::new(Sphere::new(
        Point::new([0., -1000., 0.]),
        1000.,
        permat.clone(),
    )));
    world.add(Arc::new(Sphere::new(
        Point::new([0., 2., 0.]),
        2.,
        permat.clone(),
    )));

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.vfov = 20.;
    cam.lookfrom = Point::new([13., 2., 3.]);
    cam.vup = Vector::new([0., 1., 0.]);

    cam.defocus_angle = 0.;
    cam.focus_dist = 10.;
    cam.background = Colour::new([0.7, 0.8, 1.0]);

    cam.render("perlin_spheres.png", world)
}
