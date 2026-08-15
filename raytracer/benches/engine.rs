use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use image::RgbImage;
use linalg::{vector::Vector, Point};
use raytracer::{
    bounding_volume_hierarchies::bvh::BVHNode,
    engine::{
        camera::{Camera, CameraSettings},
        hittable::{Hittable, RotateY, Translate},
        hittable_list::HittableList,
    },
    materials::{DiffuseLight, Lambertian},
    surface::{r#box, Quad},
    util::colour::Colour,
};
use std::{hint::black_box, num::NonZeroU32, sync::Arc, time::Duration};

fn cornell_bvh() -> BVHNode {
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

    BVHNode::build(world)
}

fn base_settings() -> CameraSettings {
    let mut settings = CameraSettings::default();

    settings.height = unsafe { NonZeroU32::new_unchecked(300) };
    settings.width = unsafe { NonZeroU32::new_unchecked(300) };
    settings.samples_per_pixel = unsafe { NonZeroU32::new_unchecked(200) };
    settings.max_depth = unsafe { NonZeroU32::new_unchecked(50) };
    settings.background = Colour::zero();

    settings
}

fn cornell_settings(mut settings: CameraSettings) -> CameraSettings {
    settings.defocus_angle = 0.;
    settings.focus_dist = 26.;
    settings.vfov = 40.;
    settings.lookfrom = Point::new([278., 278., -800.]);
    settings.lookat = Point::new([278., 278., 0.]);
    settings.vup = Vector::new([0., 1., 0.]);

    settings
}

fn render_benchmark(c: &mut Criterion) {
    let mut c = c.benchmark_group("render");
    c.sample_size(10);

    let base_settings = base_settings();

    let cornell_settings = cornell_settings(base_settings);
    let cornell_camera = Camera::new(&cornell_settings);
    let cornell_world = cornell_bvh();

    let bench_fn =
        |b: &mut criterion::Bencher<'_>,
         &(camera, ref world, width, height): &(Camera, BVHNode, u32, u32)| {
            b.iter(|| {
                camera.render(
                    black_box("/dev/null"),
                    black_box(world),
                    black_box(RgbImage::new(width, height)),
                )
            });
        };

    c.bench_with_input(
        BenchmarkId::from_parameter("cornell"),
        &(
            cornell_camera,
            cornell_world,
            cornell_settings.width.get(),
            cornell_settings.height.get(),
        ),
        bench_fn,
    );
    c.finish();
}

criterion_group!(camera, render_benchmark);
criterion_main!(camera);
