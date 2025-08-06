use std::sync::Arc;

use linalg::{vector::Vector, Point};
use raytracer::{
    engine::{
        camera::Camera,
        hittable::{Hittable, RotateY, Translate},
        hittable_list::HittableList,
    },
    materials::{DiffuseLight, Lambertian},
    surface::{r#box, Quad},
    util::colour::Colour,
};

fn main() -> Result<(), image::ImageError> {
    let mut world = HittableList::new();

    let red = Arc::new(Lambertian::from_colour(Colour::new([0.65, 0.05, 0.05])));
    let white = Arc::new(Lambertian::from_colour(Colour::new([0.73, 0.73, 0.73])));
    let green = Arc::new(Lambertian::from_colour(Colour::new([0.12, 0.45, 0.15])));
    let light = Arc::new(DiffuseLight::from_colour(Colour::new([15.; 3])));

    world.add(Arc::new(Quad::new(
        Point::new([555., 0., 0.]),
        Vector::new([0., 555., 0.]),
        Vector::new([0., 0., 555.]),
        green,
    )));
    world.add(Arc::new(Quad::new(
        Point::zero(),
        Vector::new([0., 555., 0.]),
        Vector::new([0., 0., 555.]),
        red,
    )));
    world.add(Arc::new(Quad::new(
        Point::new([343., 554., 332.]),
        Vector::new([-130., 0., 0.]),
        Vector::new([0., 0., -105.]),
        light,
    )));
    world.add(Arc::new(Quad::new(
        Point::zero(),
        Vector::new([555., 0., 0.]),
        Vector::new([0., 0., 555.]),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point::new([555.; 3]),
        Vector::new([-555., 0., 0.]),
        Vector::new([0., 0., -555.]),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point::new([0., 0., 555.]),
        Vector::new([555., 0., 0.]),
        Vector::new([0., 555., 0.]),
        white.clone(),
    )));

    let mut box1: Arc<dyn Hittable> = r#box(
        &Point::zero(),
        &Point::new([165., 330., 165.]),
        white.clone(),
    );
    box1 = Arc::new(RotateY::new(box1, 15.));
    box1 = Arc::new(Translate::new(box1, Vector::new([265., 0., 295.])));
    world.add(box1);

    let mut box2: Arc<dyn Hittable> = r#box(&Point::zero(), &Point::new([165., 165., 165.]), white);
    box2 = Arc::new(RotateY::new(box2, -18.));
    box2 = Arc::new(Translate::new(box2, Vector::new([130., 0., 65.])));
    world.add(box2);

    let mut cam = Camera::default();

    cam.aspect_ratio = 1.0;
    cam.width = 600;
    cam.samples_per_pixel = 200;
    cam.max_depth = 50;
    cam.background = Colour::zero();

    cam.vfov = 40.;
    cam.lookfrom = Point::new([278., 278., -800.]);
    cam.lookat = Point::new([278., 278., 0.]);
    cam.vup = Vector::new([0., 1., 0.]);

    cam.defocus_angle = 0.;
    cam.focus_dist = 26.;

    cam.render("cornell_box.png", world)
}
