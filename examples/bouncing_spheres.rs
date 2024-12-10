use linalg::Point;
use rand::{Rng, SeedableRng};
use raytracer::{
    bvh::BVHNode, camera::Camera, checker_texture::CheckerTexture, colour::Colour,
    dielectric::Dielectric, hittable_list::HittableList, lambertian::Lambertian, metals::Metal,
    sphere::Sphere, Vector,
};
use std::{fs::OpenOptions, sync::Arc};

fn setup_camera() -> Camera {
    let mut camera = Camera::default();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;

    camera.vfov = 20.;
    camera.lookfrom = Point::new([13., 2., 3.]);
    camera.lookat = Point::new([0., 0., 0.]);
    camera.vup = Vector::new([0., 1., 0.]);

    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.;

    camera
}

fn main() {
    let file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open("bouncing_spheres.ppm")
        .unwrap();
    let mut world = HittableList::default();

    let material_ground = Arc::new(Lambertian::new(Arc::new(CheckerTexture::from_colours(
        0.02,
        &Colour::new([0.2, 0.3, 0.1]),
        &Colour::new([0.9; 3]),
    ))));
    world.add(Arc::new(Sphere::new(
        Point::new([0., -1000., 0.]),
        None,
        1000.,
        material_ground,
    )));

    let mut rng = rand::rngs::StdRng::seed_from_u64(0);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let centre = Point::new([
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            ]);

            if (centre - Point::new([4., 0.2, 0.])).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vector::new([rng.gen(), rng.gen(), rng.gen()]);
                    let mat = Arc::new(Lambertian::from_colour(albedo));
                    let centre2 = centre + Vector::new([0., rng.gen::<f64>() * 0.5, 0.]);
                    world.add(Arc::new(Sphere::new(centre, Some(centre2), 0.2, mat)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vector::new([
                        rng.gen_range(0.5..=1.),
                        rng.gen_range(0.5..=1.),
                        rng.gen_range(0.5..=1.),
                    ]);
                    let fuzz = rng.gen::<f64>() * 0.5;
                    let mat = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(centre, None, 0.2, mat)));
                } else {
                    // glass
                    let mat = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(centre, None, 0.2, mat)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    let material2 = Arc::new(Lambertian::from_colour(Colour::new([0.4, 0.2, 0.1])));
    let material3 = Arc::new(Metal::new(Colour::new([0.7, 0.6, 0.5]), 0.));
    world.add(Arc::new(Sphere::new(
        Point::new([0., 1., 0.]),
        None,
        1.0,
        material1,
    )));

    world.add(Arc::new(Sphere::new(
        Point::new([-4., 1., 0.]),
        None,
        1.0,
        material2.clone(),
    )));

    world.add(Arc::new(Sphere::new(
        Point::new([4., 1., 0.]),
        None,
        1.,
        material3,
    )));

    let world = HittableList::from_object(Arc::new(BVHNode::build(world)));

    let mut camera = setup_camera();

    camera.render(file, world);
}
