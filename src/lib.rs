mod camera;
mod hit;
mod ray;
mod sphere;
mod vec3;
mod world;

pub use camera::Camera;
pub use hit::Hit;
use indicatif::ProgressBar;
pub use ray::Ray;
pub use sphere::Sphere;
pub use vec3::{Color, Point3, Vec3};
pub use world::World;

use std::{error::Error, io::Write};

pub struct RayTracer {
    aspect_ratio: f64,
    pub world: World,
    pub camera: Camera,
}

const COLOR_MAX: u8 = 255;

impl RayTracer {
    pub fn new(world: World, camera: Camera) -> Self {
        Self {
            aspect_ratio: camera.viewport_width / camera.viewport_height,
            world,
            camera,
        }
    }

    pub fn trace_in<T: Write>(
        &self,
        buffer: &mut T,
        image_height: u64,
        t_min: f64,
        t_max: f64,
    ) -> Result<(), Box<dyn Error>> {
        let camera = &self.camera;
        let world = &self.world;

        let image_width: u64 = (self.aspect_ratio * image_height as f64) as u64;

        writeln!(buffer, "P3")?;
        writeln!(buffer, "{} {}", image_width, image_height)?;
        writeln!(buffer, "{}", COLOR_MAX)?;

        let bar = ProgressBar::new(image_height);

        for j in (0..image_height).rev() {
            bar.set_length(j);
            for i in 0..image_width {
                let (i, j) = (i as f64, j as f64);
                let (width, height) = (image_width as f64, image_height as f64);

                // u: left 0.0 -> 1.0 right
                // v: botm 0.0 -> 1.0 up
                let (u, v) = (i / (width - 1.0), j / (height - 1.0));

                let direction =
                    camera.lower_left_corner + u * camera.horizontal + v * camera.vertical
                        - camera.origin;
                let ray = Ray::new(camera.origin, direction);

                let pixel = ray_color(&ray, world, t_min, t_max);
                writeln!(buffer, "{}", pixel.format_color())?;
            }
        }

        bar.finish();

        Ok(())
    }

    pub fn trace<T: Write>(&self, buffer: &mut T, image_height: u64) -> Result<(), Box<dyn Error>> {
        self.trace_in(buffer, image_height, 0.0, f64::INFINITY)
    }
}

/// Returns the color of the ray-tracing
///
/// When our ray hits a sphere, the color is red.
///
/// Background color is a simple gradient, which
/// linearly blends white and blue depending on the height of the y coordinate.
pub fn ray_color<T: Hit>(ray: &Ray, hittable: &T, t_min: f64, t_max: f64) -> Color {
    if let Some(hit) = ray.hit(hittable, t_min, t_max) {
        // Obtain the unit normal vector: -1 <= . <= 1
        let normal = hit.normal_outward.normalized();
        // For color, scale to 0 <= . <= 1
        let color = 0.5 * (normal + 1.0);

        return color;
    }

    // Scale the ray direction to unit length -1 <= direction.y <= 1
    let direction = ray.direction().normalized();

    // Scale direction into 0 <= t <= 1
    let t = 0.5 * (direction.y() + 1.0);

    // linear blend / linear interpolation / lerp
    // blended = (1 - t) * start + t * end
    let blue = Color::new(0.5, 0.7, 1.0);
    (1.0 - t) * Color::white() + t * blue
}
