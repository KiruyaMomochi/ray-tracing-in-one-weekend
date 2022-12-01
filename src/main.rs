use rand::Rng;
use ray_tracing_in_one_weekend::{
    material::{Dielectric, Lambertian, Metal},
    Camera, Color, Point3, RayTracer, Sphere, Vec3, World,
};
use std::{error::Error, fs, io::BufWriter, rc::Rc};

fn random_scene() -> World {
    let mut rng = rand::thread_rng();
    let mut world = World::new();

    let ground_mat = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let ground_sphere = Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_mat);

    world.add(ground_sphere);

    for a in -11..=11 {
        for b in -11..=11 {
            let choose_mat: f64 = rng.gen();
            let center = Point3::new((a as f64) + rng.gen_range(0.0..0.9),
                                     0.2,
                                     (b as f64) + rng.gen_range(0.0..0.9));

            if choose_mat < 0.8 {
                // Diffuse
                let albedo = Color::random(0.0..1.0) * Color::random(0.0..1.0);
                let sphere_mat = Rc::new(Lambertian::new(albedo));
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world.add(sphere);
            } else if choose_mat < 0.95 {
                // Metal
                let albedo = Color::random(0.4..1.0);
                let fuzz = rng.gen_range(0.0..0.5);
                let sphere_mat = Rc::new(Metal::new(albedo, fuzz));
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world.add(sphere);
            } else {
                // Glass
                let sphere_mat = Rc::new(Dielectric::new(1.5));
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world.add(sphere);
            }
        }
    }

    let mat1 = Rc::new(Dielectric::new(1.5));
    let mat2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let mat3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

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
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_HEIGHT: u64 = 255;
    const SAMPLES_PER_PIXEL: u64 = 100;
    const MAX_DEPTH: i64 = 500;

    // World
    let world = random_scene();

    // Camera (-1 to 1, -1 to 1, -1 to 0)
    let camera = Camera::new(
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        10.0,
        ASPECT_RATIO,
        0.1,
    );
    println!("{}", camera);

    let mut file = BufWriter::new(fs::File::create("image.ppm")?);

    let tracer = RayTracer::new(world, camera, IMAGE_HEIGHT, SAMPLES_PER_PIXEL, MAX_DEPTH);
    tracer.trace(&mut file)?;

    Ok(())
}
