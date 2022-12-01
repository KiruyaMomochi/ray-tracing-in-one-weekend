use ray_tracing_in_one_weekend::{
    material::{Dielectric, Lambertian, Metal},
    Camera, Color, Point3, RayTracer, Sphere, Vec3, World,
};
use std::{error::Error, fs, io::BufWriter, rc::Rc};

fn main() -> Result<(), Box<dyn Error>> {
    // Image
    // Use 16:9 aspect ratio
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_HEIGHT: u64 = 255;
    const SAMPLES_PER_PIXEL: u64 = 100;
    const MAX_DEPTH: i64 = 50;

    // World
    let mut world = World::new();

    let mat_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let mat_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let mat_left = Rc::new(Dielectric::new(1.5));
    let mat_left_inner = mat_left.clone();
    let mat_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    let sphere_ground = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, mat_ground);
    let sphere_center = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, mat_center);
    let sphere_left = Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, mat_left);
    let sphere_left_inner = Sphere::new(Point3::new(-1.0, 0.0, -1.0), -0.45, mat_left_inner);
    let sphere_right = Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, mat_right);

    world.add(sphere_ground);
    world.add(sphere_center);
    world.add(sphere_left);
    world.add(sphere_left_inner);
    world.add(sphere_right);

    // Camera (-1 to 1, -1 to 1, -1 to 0)
    let camera = Camera::new(
        Point3::new(-2.0, 2.0, 1.0),
        Point3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        ASPECT_RATIO,
    );
    println!("{}", camera);

    let mut file = BufWriter::new(fs::File::create("image.ppm")?);

    let tracer = RayTracer::new(world, camera, IMAGE_HEIGHT, SAMPLES_PER_PIXEL, MAX_DEPTH);
    tracer.trace(&mut file)?;

    Ok(())
}
