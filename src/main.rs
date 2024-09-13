use linalg::Point;
use raytracer::{
    camera::Camera,
    colour::Colour,
    hittable_list::HittableList,
    material::{Lambertian, Materials, Metal},
    sphere::Sphere,
};
use std::{env, fs::OpenOptions, rc::Rc};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(args[1].clone())
        .unwrap();

    let mut world = HittableList::default();

    let material_ground = Rc::new(Materials::Lambertian(Lambertian::new(Colour::new([
        0.8, 0.8, 0.0,
    ]))));
    let material_centre = Rc::new(Materials::Lambertian(Lambertian::new(Colour::new([
        0.1, 0.2, 0.5,
    ]))));
    let material_left = Rc::new(Materials::Metal(Metal::new(Colour::new([0.8, 0.8, 0.8]))));
    let material_right = Rc::new(Materials::Metal(Metal::new(Colour::new([0.8, 0.6, 0.2]))));

    world.add(Rc::new(Sphere::new(
        Point::new([0., -100.5, -1.]),
        100.,
        material_ground,
    )));
    world.add(Rc::new(Sphere::new(
        Point::new([0., 0., -1.2]),
        0.5,
        material_centre,
    )));
    world.add(Rc::new(Sphere::new(
        Point::new([-1., 0., -1.]),
        0.5,
        material_left,
    )));
    world.add(Rc::new(Sphere::new(
        Point::new([1., 0., -1.]),
        0.5,
        material_right,
    )));

    let mut camera = Camera::default();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.width = 480;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;

    camera.render(file, world);
}
