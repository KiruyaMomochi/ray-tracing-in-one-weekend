use ray_tracing_in_one_weekend::{Camera, Point3, RayTracer, Sphere, World};
use std::{error::Error, fs, io::BufWriter};

fn main() -> Result<(), Box<dyn Error>> {
    // Image
    // Use 16:9 aspect ratio
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_HEIGHT: u64 = 255;
    const SAMPLES_PER_PIXEL: u64 = 100;

    // World
    let mut world = World::new();
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    // Camera (-1 to 1, -1 to 1, -1 to 0)
    let camera = Camera::new(2.0, ASPECT_RATIO, 1.0);
    println!("{}", camera);

    let mut file = BufWriter::new(fs::File::create("image.ppm")?);

    let tracer = RayTracer::new(world, camera, IMAGE_HEIGHT, SAMPLES_PER_PIXEL);
    tracer.trace(&mut file)?;

    Ok(())
}
