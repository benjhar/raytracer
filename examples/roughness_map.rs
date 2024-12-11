use std::{fs::OpenOptions, sync::Arc};

use linalg::{vector::Vector, Point};
use raytracer::{
    engine::{camera::Camera, hittable_list::HittableList},
    materials::{Dielectric, Lambertian, Metal},
    surface::Sphere,
    textures::{Fractal, Solid},
    util::colour::Colour,
};

fn main() {
    let file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open("roughness_map.ppm")
        .unwrap();

    let mut world = HittableList::new();

    let material_ground = Arc::new(Lambertian::new(Arc::new(Solid::new(0.9, 0.9, 0.9))));
    world.add(Arc::new(Sphere::new(
        Point::new([0., -1000., 0.]),
        None,
        1000.,
        material_ground.clone(),
    )));

    let roughness_map = Arc::new(Fractal::new(1., 4., 7, 0.5, 2.0));
    let gold_mat = Arc::new(Metal::new(
        Colour::new([0.944, 0.776, 0.373]),
        roughness_map,
    ));

    let glass_roughness_map = Arc::new(Fractal::new(0.7, 6., 9, 0.5, 2.0));
    let glass_mat = Arc::new(Dielectric::new(1.5, glass_roughness_map));

    let sphere1 = Arc::new(Sphere::new(Point::new([-2., 2., 0.]), None, 2., gold_mat));

    world.add(sphere1);

    let sphere2 = Arc::new(Sphere::new(
        Point::new([3., 1.5, -1.]),
        None,
        1.5,
        glass_mat,
    ));

    world.add(sphere2);

    let sphere3 = Arc::new(Sphere::new(
        Point::new([0., 1., 4.5]),
        None,
        1.,
        material_ground.clone(),
    ));

    world.add(sphere3);

    let sphere4 = Arc::new(Sphere::new(
        Point::new([2., 1., -5.]),
        None,
        1.,
        material_ground,
    ));

    world.add(sphere4);

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.width = 1920;
    cam.samples_per_pixel = 1000;
    cam.max_depth = 50;

    cam.vfov = 40.;
    cam.lookfrom = Point::new([13., 2., 0.]);
    cam.lookat = Point::new([-2., 2., 0.]);
    cam.vup = Vector::new([0., 1., 0.]);

    cam.defocus_angle = 0.;
    cam.focus_dist = 2.;

    cam.render(file, world);
}
