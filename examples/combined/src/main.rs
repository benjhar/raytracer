use std::sync::Arc;

use linalg::{vector::Vector, Point};
use rand::Rng;
use raytracer::{
    bounding_volume_hierarchies::bvh::BVHNode,
    engine::{
        camera::Camera,
        hittable::{ConstantMedium, RotateY, Translate},
        hittable_list::HittableList,
    },
    materials::{Dielectric, DiffuseLight, Lambertian, Metal},
    surface::{r#box, Quad, Sphere},
    textures::{Fractal, Image, Solid},
    util::colour::Colour,
};

const BOXES_PER_SIDE: u32 = 20;

#[allow(clippy::too_many_lines)]
fn main() -> Result<(), image::ImageError> {
    let mut boxes1 = HittableList::new();
    let ground = Arc::new(Lambertian::from_colour(Colour::new([0.48, 0.83, 0.53])));

    let w = 100.0;
    let y0 = 0.0;
    for i in 0..BOXES_PER_SIDE {
        for j in 0..BOXES_PER_SIDE {
            let x0 = -1000.0 + f64::from(i) * w;
            let z0 = -1000.0 + f64::from(j) * w;
            let x1 = x0 + w;
            let y1 = rand::thread_rng().gen_range(1.0..101.0);
            let z1 = z0 + w;

            boxes1.add(r#box(
                &Point::new([x0, y0, z0]),
                &Point::new([x1, y1, z1]),
                ground.clone(),
            ));
        }
    }

    let mut world = HittableList::new();

    world.add(Arc::new(BVHNode::build(boxes1)));

    let light = Arc::new(DiffuseLight::from_colour(Colour::new([7.; 3])));
    world.add(Arc::new(Quad::new(
        Point::new([123., 554., 147.]),
        Vector::new([300., 0., 0.]),
        Vector::new([0., 0., 265.]),
        light,
    )));

    let centre1 = Point::new([400., 400., 200.]);
    let centre2 = centre1 + Vector::new([30., 0., 0.]);
    let sphere_material = Arc::new(Lambertian::from_colour(Colour::new([0.7, 0.3, 0.1])));
    world.add(Arc::new(Sphere::moving(
        centre1,
        centre2,
        50.,
        sphere_material,
    )));

    let smooth_texture = Arc::new(Solid::new(0.0, 0.0, 0.0));
    let glass = Arc::new(Dielectric::new(Colour::one(), 1.5, smooth_texture));

    world.add(Arc::new(Sphere::new(
        Point::new([260., 150., 45.]),
        50.,
        glass.clone(),
    )));
    world.add(Arc::new(Sphere::new(
        Point::new([0., 150., 145.]),
        50.,
        Arc::new(Metal::new(
            Colour::new([0.8, 0.8, 0.9]),
            Arc::new(Solid::new(1.0, 1.0, 1.0)),
        )),
    )));

    let boundary = Arc::new(Sphere::new(
        Point::new([360., 150., 145.]),
        70.,
        glass.clone(),
    ));
    world.add(boundary.clone());
    world.add(Arc::new(ConstantMedium::from_colour(
        boundary,
        0.2,
        Colour::new([0.2, 0.4, 0.9]),
    )));
    // let boundary2 = Arc::new(Sphere::new(Point::zero(), 5000., glass));
    // world.add(Arc::new(ConstantMedium::from_colour(
    //     boundary2,
    //     0.0001,
    //     Colour::one(),
    // )));

    let emat = Arc::new(Lambertian::new(Arc::new(
        Image::try_file("../earth/assets/earthmap.jpg").expect("Should find earthmap asset"),
    )));
    world.add(Arc::new(Sphere::new(
        Point::new([400., 200., 400.]),
        100.,
        emat,
    )));
    let pertext = Arc::new(Fractal::new(1.0, 1.0, 1, 1.0, 1.0));
    world.add(Arc::new(Sphere::new(
        Point::new([220., 280., 300.]),
        80.,
        Arc::new(Lambertian::new(pertext)),
    )));

    let mut boxes2 = HittableList::new();
    let white = Arc::new(Lambertian::from_colour(Colour::new([0.73; 3])));
    let ns = 1000;
    for _ in 0..ns {
        boxes2.add(Arc::new(Sphere::new(
            Point::random_range(0., 165.),
            10.,
            white.clone(),
        )));
    }

    world.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(Arc::new(BVHNode::build(boxes2)), 15.)),
        Vector::new([-100., 270., 395.]),
    )));

    let mut cam = Camera::default();

    cam.aspect_ratio = 1.0;
    cam.width = 800;
    cam.samples_per_pixel = 10000;
    cam.max_depth = 40;
    cam.background = Colour::zero();

    cam.vfov = 40.;
    cam.lookfrom = Point::new([478., 278., -600.]);
    cam.lookat = Point::new([278., 278., 0.]);
    cam.vup = Vector::new([0., 1., 0.]);

    cam.defocus_angle = 0.;
    cam.focus_dist = 9.;

    cam.render("combined.png", world)
}
