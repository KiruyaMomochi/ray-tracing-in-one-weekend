mod camera;
mod hit;
pub mod material;
mod ray;
mod sphere;
mod vec3;
mod world;

pub use camera::Camera;
pub use hit::Hit;
use indicatif::ParallelProgressIterator;
pub use material::Material;
use rand::Rng;
pub use ray::Ray;
pub use sphere::Sphere;
pub use vec3::{Color, Point3, Vec3};
pub use world::World;
pub type HitRecord = hit::AgainstRayHitRecord;

use rayon::prelude::*;
use std::{error::Error, io::Write};

pub struct RayTracer {
    aspect_ratio: f64,
    world: World,
    camera: Camera,
    pub max_depth: i64,
    pub samples_per_pixel: u64,
    pub image_height: u64,
}

const COLOR_MAX: u8 = 255;

impl RayTracer {
    pub fn new(
        world: World,
        camera: Camera,
        image_height: u64,
        samples_per_pixel: u64,
        max_depth: i64,
    ) -> Self {
        Self {
            aspect_ratio: camera.aspect_ratio(),
            world,
            camera,
            image_height,
            samples_per_pixel,
            max_depth,
        }
    }

    pub fn trace_in<T: Write>(
        &self,
        buffer: &mut T,
        t_min: f64,
        t_max: f64,
    ) -> Result<(), Box<dyn Error>> {
        let camera = &self.camera;
        let world = &self.world;
        let image_height = self.image_height;
        let samples_per_pixel = self.samples_per_pixel;

        let image_width: u64 = (self.aspect_ratio * image_height as f64) as u64;

        writeln!(buffer, "P3")?;
        writeln!(buffer, "{} {}", image_width, image_height)?;
        writeln!(buffer, "{}", COLOR_MAX)?;

        // bar.set_position(j);
        let colors = (0..image_height)
            .into_par_iter()
            .progress_count(image_height)
            .flat_map(|j| {
                (0..image_width).into_par_iter().map(move |i| {
                    let mut rng = rand::thread_rng();
                    let (width, height) = (image_width as f64, image_height as f64);
                    let (i, j) = (i as f64, height - j as f64 - 1.0);

                    let mut pixel_color_sum = Color::zero();
                    for _ in 0..samples_per_pixel {
                        // u: left 0.0 -> 1.0 right
                        // v: botm 0.0 -> 1.0 up
                        // rng.gen: standard distribution, [0, 1)
                        let u = (i + rng.gen::<f64>()) / (width - 1.0);
                        let v = (j + rng.gen::<f64>()) / (height - 1.0);

                        let ray = camera.cast(u, v);
                        pixel_color_sum += ray_color(&ray, world, self.max_depth, t_min, t_max);
                    }

                    pixel_color_sum / (samples_per_pixel as f64)
                })
            })
            .collect::<Vec<_>>();

        for pixel_color in colors {
            // for pixel_color in colors {
            writeln!(buffer, "{}", pixel_color.format_color())?;
            // }
        }

        Ok(())
    }

    pub fn trace<T: Write>(&self, buffer: &mut T) -> Result<(), Box<dyn Error>> {
        // To fix the shadow acne problem, which some hit rays may not at exactly t = 0
        self.trace_in(buffer, 0.001, f64::INFINITY)
    }
}

/// Returns the color of the ray-tracing
///
/// When our ray hits a sphere, the color is red.
///
/// Background color is a simple gradient, which
/// linearly blends white and blue depending on the height of the y coordinate.
pub fn ray_color<T: Hit>(ray: &Ray, hittable: &T, depth: i64, t_min: f64, t_max: f64) -> Color {
    if depth <= 0 {
        // If we've exceeded the ray bounce limit, no more light is gathered
        Color::black()
    } else if let Some(hit) = ray.hit(hittable, t_min, t_max) {
        if let Some((ray, attenuation)) = hit.material.scatter(ray, &hit) {
            // Return the scattered ray
            attenuation * ray_color(&ray, hittable, depth - 1, t_min, t_max)
        } else {
            Color::black()
        }
    } else {
        // Scale the ray direction to unit length -1 <= direction.y <= 1
        let direction = ray.direction().normalized();

        // Scale direction into 0 <= t <= 1
        let t = 0.5 * (direction.y() + 1.0);

        // linear blend / linear interpolation / lerp
        // blended = (1 - t) * start + t * end
        let blue = Color::new(0.5, 0.7, 1.0);
        (1.0 - t) * Color::white() + t * blue
    }
}
