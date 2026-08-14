use image::RgbImage;
use std::{num::NonZeroU32, sync::Arc};

use linalg::{vector::Vector, Point};
use raytracer::{
    bounding_volume_hierarchies::bvh::BVHNode,
    engine::{
        camera::{Camera, CameraSettings},
        hittable::{ConstantMedium, Hittable, RotateY, Translate},
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
    let light = Arc::new(DiffuseLight::from_colour(Colour::new([7.; 3])));

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
        Point::new([113., 554., 127.]),
        Vector::new([330., 0., 0.]),
        Vector::new([0., 0., 305.]),
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
    world.add(Arc::new(ConstantMedium::from_colour(
        box1,
        0.01,
        Colour::zero(),
    )));

    let mut box2: Arc<dyn Hittable> = r#box(&Point::zero(), &Point::new([165., 165., 165.]), white);
    box2 = Arc::new(RotateY::new(box2, -18.));
    box2 = Arc::new(Translate::new(box2, Vector::new([130., 0., 65.])));
    world.add(Arc::new(ConstantMedium::from_colour(
        box2,
        0.01,
        Colour::one(),
    )));

    let world = BVHNode::build(world);

    let mut cam_settings = CameraSettings::default();

    cam_settings.width = NonZeroU32::new(600).unwrap();
    cam_settings.height = NonZeroU32::new(600).unwrap();
    cam_settings.samples_per_pixel = NonZeroU32::new(20).unwrap();
    cam_settings.max_depth = NonZeroU32::new(50).unwrap();
    cam_settings.background = Colour::zero();

    cam_settings.vfov = 40.;
    cam_settings.lookfrom = Point::new([278., 278., -800.]);
    cam_settings.lookat = Point::new([278., 278., 0.]);
    cam_settings.vup = Vector::new([0., 1., 0.]);

    cam_settings.defocus_angle = 0.;
    cam_settings.focus_dist = 26.;

    let imgbuf = RgbImage::new(cam_settings.width.into(), cam_settings.height.into());

    let cam = Camera::new(&cam_settings);

    cam.render("foggy_cornell.png", &world, imgbuf)
}
