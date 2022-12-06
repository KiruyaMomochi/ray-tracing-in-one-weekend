use rand::Rng;
use ray_tracing_in_one_weekend::{
    material::{Dielectric, Lambertian, Metal},
    Camera, Color, Point3, RayTracer, Sphere, Vec3, World,
};
use std::{error::Error, fs, io::BufWriter, sync::Arc};

fn random_scene() -> World {
    let mut rng = rand::thread_rng();
    let mut world = World::new();

    let ground_mat = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let ground_sphere = Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_mat);

    world.add(ground_sphere);

    for a in -11..=11 {
        for b in -11..=11 {
            let choose_mat: f64 = rng.gen();
            let center = Point3::new(
                (a as f64) + rng.gen_range(0.0..0.9),
                0.2,
                (b as f64) + rng.gen_range(0.0..0.9),
            );
            let new_center = center + Vec3::new(0.0, rng.gen_range(0.0..0.5), 0.0);

            if choose_mat < 0.8 {
                // Diffuse
                let albedo = Color::random(0.0..1.0) * Color::random(0.0..1.0);
                let sphere_mat = Arc::new(Lambertian::new(albedo));
                let sphere = Sphere::new(center, 0.2, sphere_mat).into_moving(0.0, 1.0, new_center);

                world.add(sphere);
            } else if choose_mat < 0.95 {
                // Metal
                let albedo = Color::random(0.4..1.0);
                let fuzz = rng.gen_range(0.0..0.5);
                let sphere_mat = Arc::new(Metal::new(albedo, fuzz));
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world.add(sphere);
            } else {
                // Glass
                let sphere_mat = Arc::new(Dielectric::new(1.5));
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world.add(sphere);
            }
        }
    }

    let mat1 = Arc::new(Dielectric::new(1.5));
    let mat2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let mat3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

    let sphere1 = Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1);
    let sphere2 = Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2);
    let sphere3 = Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3);

    world.add(sphere1);
    world.add(sphere2);
    world.add(sphere3);

    world
}

fn main() -> Result<(), Box<dyn Error>> {
    // Image
    // Use 16:9 aspect ratio
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_HEIGHT: u64 = 400 / ASPECT_RATIO as u64;
    const SAMPLES_PER_PIXEL: u64 = 100;
    const MAX_DEPTH: i64 = 50;

    // World
    let world = random_scene();

    // Camera (-1 to 1, -1 to 1, -1 to 0)
    let camera = Camera::builder()
        .look_from(13.0, 2.0, 3.0)
        .look_at(0.0, 0.0, 0.0)
        .view_up(0.0, 1.0, 0.0)
        .vertical_field_of_view(20.0)
        .aspect_ratio(ASPECT_RATIO)
        .aperture(0.1)
        .focus_distance(10.0)
        .build();
    println!("{}", camera);

    let mut file = BufWriter::new(fs::File::create("image.ppm")?);

    let tracer = RayTracer::new(world, camera, IMAGE_HEIGHT, SAMPLES_PER_PIXEL, MAX_DEPTH);
    tracer.trace(&mut file)?;

    Ok(())
}
