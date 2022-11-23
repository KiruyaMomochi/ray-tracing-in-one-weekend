use ray_tracing_in_one_weekend::{
    material::{Lambertian, Metal}, Camera, Color, Point3, RayTracer, Sphere, World,
};
use std::{error::Error, fs, io::BufWriter, rc::Rc};

fn main() -> Result<(), Box<dyn Error>> {
    // Image
    // Use 16:9 aspect ratio
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_HEIGHT: u64 = 255;
    const SAMPLES_PER_PIXEL: u64 = 20;

    // World
    let mut world = World::new();

    let mat_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let mat_center = Lambertian::new(Color::new(0.7, 0.3, 0.3));
    let mat_left = Metal::new(Color::new(0.8, 0.8, 0.8));
    let mat_right = Metal::new(Color::new(0.8, 0.6, 0.2));

    let sphere_ground = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, Rc::new(mat_ground));
    let sphere_center = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, Rc::new(mat_center));
    let sphere_left = Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, Rc::new(mat_left));
    let sphere_right = Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, Rc::new(mat_right));

    world.add(sphere_ground);
    world.add(sphere_center);
    world.add(sphere_left);
    world.add(sphere_right);

    // Camera (-1 to 1, -1 to 1, -1 to 0)
    let camera = Camera::new(2.0, ASPECT_RATIO, 1.0);
    println!("{}", camera);

    let mut file = BufWriter::new(fs::File::create("image.ppm")?);

    let tracer = RayTracer::new(world, camera, IMAGE_HEIGHT, SAMPLES_PER_PIXEL);
    tracer.trace(&mut file)?;

    Ok(())
}
