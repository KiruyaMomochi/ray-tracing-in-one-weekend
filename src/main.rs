use ray_tracing_in_one_weekend::{
    material::Lambertian,
    Camera, Color, Point3, RayTracer, Sphere, World,
};
use std::{error::Error, fs, io::BufWriter, rc::Rc};

fn main() -> Result<(), Box<dyn Error>> {
    // Image
    // Use 16:9 aspect ratio
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_HEIGHT: u64 = 255;
    const SAMPLES_PER_PIXEL: u64 = 20;
    const MAX_DEPTH: i64 = 50;

    // World
    let mut world = World::new();

    let r = 45.0_f64.to_radians().cos();

    let mat_left = Rc::new(Lambertian::new(Color::blue()));
    let mat_right = Rc::new(Lambertian::new(Color::red()));

    let sphere_left = Sphere::new(Point3::new(-r, 0.0, -1.0), r, mat_left);
    let sphere_right = Sphere::new(Point3::new(r, 0.0, -1.0), r, mat_right);

    world.add(sphere_left);
    world.add(sphere_right);

    // Camera (-1 to 1, -1 to 1, -1 to 0)
    let camera = Camera::new(90.0, ASPECT_RATIO);
    println!("{}", camera);

    let mut file = BufWriter::new(fs::File::create("image.ppm")?);

    let tracer = RayTracer::new(world, camera, IMAGE_HEIGHT, SAMPLES_PER_PIXEL, MAX_DEPTH);
    tracer.trace(&mut file)?;

    Ok(())
}
