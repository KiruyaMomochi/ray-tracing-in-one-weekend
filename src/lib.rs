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
use log::debug;
pub use material::Material;
pub use object::Sphere;
pub use object::World;
use rand::Rng;
pub use ray::Ray;
pub use vec3::{Color, Point3, Vec3};

use rayon::prelude::*;
use std::{error::Error, io::Write};

pub struct RayTracer<H: Hit> {
    pub world: H,
    pub camera: Camera,
    pub background: Color,
    pub max_depth: i64,
    pub samples_per_pixel: u64,
    pub image_height: u64,
}

const COLOR_MAX: u8 = 255;

impl<H: Hit> RayTracer<H> {
    fn aspect_ratio(&self) -> f64 {
        self.camera.aspect_ratio()
    }

    pub fn trace_single(
        &self,
        i: u64,
        j: u64,
        image_width: u64,
        image_height: u64,
        t_min: f64,
        t_max: f64,
    ) -> Color {
        let mut rng = rand::thread_rng();
        let (width, height) = (image_width as f64, image_height as f64);
        let (i, j) = (i as f64, height - j as f64 - 1.0);

        let mut pixel_color_sum = Color::zeros();

        {
            debug!("## {} {} ({})", i, j, 0);
            let (u, v) = (i / (width - 1.0), j / (height - 1.0));
            let ray = self.camera.cast(u, v);

            pixel_color_sum += ray_color(
                ray,
                self.background,
                &self.world,
                self.max_depth,
                t_min,
                t_max,
            );
        }

        for run in 1..self.samples_per_pixel {
            debug!("## {} {} ({})", i, j, run);
            // u: left 0.0 -> 1.0 right
            // v: botm 0.0 -> 1.0 up
            // rng.gen: standard distribution, [0, 1)
            let u = (i + rng.gen::<f64>()) / (width - 1.0);
            let v = (j + rng.gen::<f64>()) / (height - 1.0);

            let ray = self.camera.cast(u, v);
            pixel_color_sum += ray_color(
                ray,
                self.background,
                &self.world,
                self.max_depth,
                t_min,
                t_max,
            );
        }

        debug!(
            "  final color: {:?}",
            pixel_color_sum / (self.samples_per_pixel as f64)
        );
        pixel_color_sum / (self.samples_per_pixel as f64)
    }

    pub fn trace_in<T: Write>(
        &self,
        buffer: &mut T,
        t_min: f64,
        t_max: f64,
    ) -> Result<(), Box<dyn Error>> {
        let image_height = self.image_height;
        let image_width: u64 = (self.aspect_ratio() * image_height as f64) as u64;

        writeln!(buffer, "P3")?;
        writeln!(buffer, "{} {}", image_width, image_height)?;
        writeln!(buffer, "{}", COLOR_MAX)?;

        // bar.set_position(j);
        let colors = (0..image_height)
            .into_par_iter()
            .progress_count(image_height)
            .flat_map(|j| {
                (0..image_width)
                    .into_par_iter()
                    .map(move |i| self.trace_single(i, j, image_width, image_height, t_min, t_max))
            })
            .collect::<Vec<_>>();

        for pixel_color in colors {
            writeln!(buffer, "{}", pixel_color.format_color())?;
        }

        Ok(())
    }

    pub fn trace<T: Write>(&self, buffer: &mut T) -> Result<(), Box<dyn Error>> {
        // To fix the shadow acne problem, which some hit rays may not at exactly t = 0
        // I have seen 0.0000000000000002775557561562895, so f64::EPSILON is not a choice here
        self.trace_in(buffer, 1e-10, f64::INFINITY)
    }
}

/// Returns the color of the ray-tracing
///
/// When our ray hits a sphere, the color is red.
///
/// Background color is a simple gradient, which
/// linearly blends white and blue depending on the height of the y coordinate.
pub fn ray_color<T: Hit>(
    ray: Ray,
    background: Color,
    object: &T,
    depth: i64,
    t_min: f64,
    t_max: f64,
) -> Color {
    debug!("  [{}] ray: {} -> {}", depth, ray.origin(), ray.direction());
    let color = if depth <= 0 {
        // If we've exceeded the ray bounce limit, no more light is gathered
        Color::BLACK
    } else if let Some(hit) = ray.clone().hit(object, t_min, t_max) {
        let emitted = hit.emitted;
        debug!(
            "  [{}]   hit at t = {} {}, normal {}",
            depth, hit.t, hit.point, hit.normal_outward
        );
        let hit = hit.into_against_ray();

        let color = if let Some((ray, attenuation)) = hit.material.scatter(&ray, &hit) {
            debug!("  [{}]   attenuation: {}", depth, attenuation);
            if attenuation.is_near_zero() {
                // short circuit
                debug!("  [{}]   attenuation is zero, short circuit", depth);
                return Color::BLACK;
            }
            // the scattered ray
            attenuation * ray_color(ray, background, object, depth - 1, t_min, t_max)
        } else {
            Color::BLACK
        };

        emitted + color
    } else {
        // The ray hits nothing, return the background color
        background
    };
    debug!("  [{}]   color: {}", depth, color);
    color
}
