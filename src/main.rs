use indicatif::ProgressBar;
use std::{
    fs,
    io::{BufWriter, Write},
};
use vec3::{Point3, Vec3};

mod ray;
mod vec3;

use crate::{ray::Ray, vec3::Color};

/// Simple camera
#[derive(Debug, Clone)]
struct Camera {
    // Virtual viewport to pass scene rays
    pub viewport_height: f64,
    pub viewport_width: f64,
    /// Distance between projection plane and projection point
    pub focal_length: f64,
    pub origin: Point3,
    /// x-asis
    pub horizontal: Vec3<f64>,
    /// y-axis
    pub vertical: Vec3<f64>,
    pub lower_left_corner: Point3,
}

impl Camera {
    pub fn new(viewport_height: f64, aspect_ratio: f64, focal_length: f64) -> Self {
        let origin = Point3::zero();
        let horizontal = Vec3::new(viewport_height, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        Self {
            viewport_height,
            viewport_width: viewport_height * aspect_ratio,
            focal_length,
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }
}

/// Returns the color of the background (a simple gradient)
///
/// Linearly blends white and blue depending on the height of the y coordinate.
fn ray_color(ray: &Ray) -> Color {
    // Scale the ray direction to unit length -1 <= direction.y <= 1
    let direction = ray.direction().normalized();

    // Scale direction into 0 <= t <= 1
    let t = 0.5 * (direction.y() + 1.0);
    
    // linear blend / linear interpolation / lerp
    // blended = (1 - t) * start + t * end
    let blue = Color::new(0.5, 0.7, 1.0);
    (1.0 - t) * Color::white() + t * blue
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Image

    // Use 16:9 aspect ratio
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 255;
    const IMAGE_HEIGHT: u64 = 255;

    const COLOR_MAX: u8 = 255;

    // Camera (-1 to 1, -1 to 1, -1 to 0)
    let camera = Camera::new(2.0, ASPECT_RATIO, 1.0);

    let mut file = BufWriter::new(fs::File::create("image.ppm")?);

    writeln!(file, "P3")?;
    writeln!(file, "{} {}", IMAGE_WIDTH, IMAGE_HEIGHT)?;
    writeln!(file, "{}", COLOR_MAX)?;

    let bar = ProgressBar::new(IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        bar.set_length(j);
        for i in 0..IMAGE_WIDTH {
            let (i, j) = (i as f64, j as f64);
            let (width, height) = (IMAGE_WIDTH as f64, IMAGE_HEIGHT as f64);

            // u: 0.0 -> 1.0
            // v: 0.0 ^ 1.0
            let (u, v) = (i / (width - 1.0), j / (height - 1.0));

            let direction = camera.lower_left_corner + u * camera.horizontal + v * camera.vertical
                - camera.origin;
            let ray = Ray::new(camera.origin, direction);

            let pixel = ray_color(&ray);
            writeln!(file, "{}", pixel.format_color())?;
        }
    }

    bar.finish();

    Ok(())
}
