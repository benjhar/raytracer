use linalg::{vector::Vector, Point};
use raytracer::{
    engine::{camera::Camera, hittable_list::HittableList},
    materials::Lambertian,
    surface::Sphere,
    textures::Image,
};
use std::sync::Arc;

fn main() -> Result<(), image::ImageError> {
    let earth_texture = Arc::new(Image::try_file("./assets/earthmap.jpg").unwrap());
    let earth_surface = Arc::new(Lambertian::new(earth_texture));
    let globe = Arc::new(Sphere::new(Point::new([0.; 3]), None, 2., earth_surface));

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.vfov = 20.;
    cam.lookfrom = Point::new([8.485, 8.485, 0.]);
    cam.lookat = Point::new([0.; 3]);
    cam.vup = Vector::new([0., 1., 0.]);

    cam.defocus_angle = 0.0;
    cam.focus_dist = 10.0;

    cam.render("earth.png", HittableList::from_object(globe))
}
