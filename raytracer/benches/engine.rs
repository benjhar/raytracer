use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use image::RgbImage;
use linalg::{vector::Vector, Point};
use rayon::iter::ParallelIterator;
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
use std::{hint::black_box, num::NonZeroU32, sync::Arc};

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

fn cornell_cam_bvh(base_settings: CameraSettings) -> (Camera, BVHNode) {
    let cornell_settings = cornell_settings(base_settings);
    let cornell_camera = Camera::new(&cornell_settings);
    let cornell_bvh = cornell_bvh();
    (cornell_camera, cornell_bvh)
}

fn render_benchmark(c: &mut Criterion) {
    let mut c = c.benchmark_group("render");
    c.sample_size(10);

    let (cornell_camera, cornell_bvh) = cornell_cam_bvh(base_settings());

    fn bench_fn_builder(
        buffer_size: (u32, u32),
    ) -> impl Fn(&mut criterion::Bencher<'_>, &(Camera, BVHNode)) {
        move |b: &mut criterion::Bencher<'_>, &(camera, ref world): &(Camera, BVHNode)| {
            b.iter_batched(
                || RgbImage::new(buffer_size.0, buffer_size.1),
                |buffer| camera.render(black_box("/dev/null"), black_box(world), black_box(buffer)),
                criterion::BatchSize::SmallInput,
            );
        }
    }

    c.bench_with_input(
        BenchmarkId::from_parameter("cornell"),
        &(cornell_camera, cornell_bvh),
        bench_fn_builder((cornell_camera.width.into(), cornell_camera.height.into())),
    );
    c.finish();
}

fn get_ray_benchmark(c: &mut Criterion) {
    let mut c = c.benchmark_group("get_ray");

    let (cornell_camera, _) = cornell_cam_bvh(base_settings());

    fn bench_fn_builder(
        buffer_size: (u32, u32),
    ) -> impl FnMut(&mut criterion::Bencher<'_>, &Camera) {
        move |b: &mut criterion::Bencher<'_>, &camera: &Camera| {
            b.iter_batched(
                || RgbImage::new(buffer_size.0, buffer_size.1),
                |buffer| {
                    buffer.par_enumerate_pixels().for_each(|(i, j, _)| {
                        for s_i in 0..camera.sqrt_spp.into() {
                            for s_j in 0..camera.sqrt_spp.into() {
                                let ray = camera.get_ray(
                                    black_box(i),
                                    black_box(j),
                                    black_box(s_i),
                                    black_box(s_j),
                                );
                                black_box(ray);
                            }
                        }
                    })
                },
                criterion::BatchSize::SmallInput,
            );
        }
    }

    c.bench_with_input(
        BenchmarkId::from_parameter("cornell"),
        &cornell_camera,
        bench_fn_builder((cornell_camera.width.into(), cornell_camera.height.into())),
    );

    c.finish();
}

fn ray_colour_benchmark(c: &mut Criterion) {
    let mut c = c.benchmark_group("ray_colour");

    let mut settings = base_settings();
    settings.max_depth = NonZeroU32::MIN.saturating_add(1);
    settings.samples_per_pixel = NonZeroU32::MIN.saturating_add(3);
    let (cornell_camera, cornell_bvh) = cornell_cam_bvh(settings);

    fn bench_fn_builder(
        buffer_size: (u32, u32),
        camera: &Camera,
    ) -> impl FnMut(&mut criterion::Bencher<'_>, &(Camera, BVHNode)) {
        let mut rays = Vec::new();
        let buffer = RgbImage::new(buffer_size.0, buffer_size.1);
        buffer.enumerate_pixels().for_each(|(i, j, _)| {
            for s_i in 0..camera.sqrt_spp.into() {
                for s_j in 0..camera.sqrt_spp.into() {
                    rays.push(camera.get_ray(
                        black_box(i),
                        black_box(j),
                        black_box(s_i),
                        black_box(s_j),
                    ));
                }
            }
        });

        move |b: &mut criterion::Bencher<'_>, &(camera, ref world): &(Camera, BVHNode)| {
            b.iter_batched(
                || &rays,
                |rays| {
                    for ray in rays {
                        let colour = camera.ray_colour(*black_box(ray), camera.max_depth, world);
                        let _ = black_box(colour);
                    }
                },
                criterion::BatchSize::LargeInput,
            );
        }
    }

    c.bench_with_input(
        BenchmarkId::from_parameter("cornell"),
        &(cornell_camera, cornell_bvh),
        bench_fn_builder(
            (cornell_camera.width.into(), cornell_camera.height.into()),
            &cornell_camera,
        ),
    );

    c.finish();
}

criterion_group!(
    camera,
    render_benchmark,
    get_ray_benchmark,
    ray_colour_benchmark
);
criterion_main!(camera);
