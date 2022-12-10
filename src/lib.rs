pub mod camera;
pub mod hit;
pub mod material;
pub mod object;
mod ray;
pub mod texture;
mod vec3;

pub use camera::Camera;
pub use hit::Hit;
use indicatif::ParallelProgressIterator;
pub use material::Material;
pub use object::Sphere;
pub use object::World;
use rand::Rng;
pub use ray::Ray;
pub use vec3::{Color, Point3, Vec3};
pub type HitRecord = hit::AgainstRayHitRecord;

use rayon::prelude::*;
use std::{error::Error, io::Write};

pub struct RayTracer {
    pub world: World,
    pub camera: Camera,
    pub background: Color,
    pub max_depth: i64,
    pub samples_per_pixel: u64,
    pub image_height: u64,
}

const COLOR_MAX: u8 = 255;

impl RayTracer {
    fn aspect_ratio(&self) -> f64 {
        self.camera.aspect_ratio()
    }

    pub fn trace_in<T: Write>(
        &self,
        buffer: &mut T,
        t_min: f64,
        t_max: f64,
    ) -> Result<(), Box<dyn Error>> {
        let camera = &self.camera;
        let world = &self.world;
        let background = self.background;
        let image_height = self.image_height;
        let samples_per_pixel = self.samples_per_pixel;
        let max_depth = self.max_depth;

        let image_width: u64 = (self.aspect_ratio() * image_height as f64) as u64;

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

                    let mut pixel_color_sum = Color::zeros();
                    for _ in 0..samples_per_pixel {
                        // u: left 0.0 -> 1.0 right
                        // v: botm 0.0 -> 1.0 up
                        // rng.gen: standard distribution, [0, 1)
                        let u = (i + rng.gen::<f64>()) / (width - 1.0);
                        let v = (j + rng.gen::<f64>()) / (height - 1.0);

                        let ray = camera.cast(u, v);
                        pixel_color_sum +=
                            ray_color(&ray, background, world, max_depth, t_min, t_max);
                    }

                    pixel_color_sum / (samples_per_pixel as f64)
                })
            })
            .collect::<Vec<_>>();

        for pixel_color in colors {
            writeln!(buffer, "{}", pixel_color.format_color())?;
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
pub fn ray_color<T: Hit>(
    ray: &Ray,
    background: Color,
    object: &T,
    depth: i64,
    t_min: f64,
    t_max: f64,
) -> Color {
    if depth <= 0 {
        // If we've exceeded the ray bounce limit, no more light is gathered
        Color::BLACK
    } else if let Some(hit) = ray.hit(object, t_min, t_max) {
        // emitted color from the object at hit point
        let emitted = hit.material.emit(hit.point, hit.u, hit.v);

        let color = if let Some((ray, attenuation)) = hit.material.scatter(ray, &hit) {
            // the scattered ray
            attenuation * ray_color(&ray, background, object, depth - 1, t_min, t_max)
        } else {
            Color::BLACK
        };

        emitted + color
    } else {
        // The ray hits nothing, return the background color
        background
    }
}
