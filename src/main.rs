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
        let viewport_width = viewport_height * aspect_ratio;
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        Self {
            viewport_height,
            viewport_width,
            focal_length,
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }
}

#[derive(Debug, Clone)]
struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }

    fn center(&self) -> Vec3<f64> {
        self.center
    }

    fn radius(&self) -> f64 {
        self.radius
    }
}

/// Check if ray hits the sphere
///
/// The equation of the sphere in vector form is
///
///     (P - C) . (P - C) = r^2
///
/// where C is the vector from sphere center, and P is the point.
///
/// When P is the ray P(t) = A + tb for some t,  the equation expands to
///
///     (A + t b - C) . (A + t b - C) = r^2
///
/// and in quadratic form
///
///     (b.b) t^2 + (2b.(A-C)) t + ((A-C).(A-C) - r^2) = 0
///
fn ray_hit_sphere(ray: &Ray, sphere: &Sphere) -> bool {
    // oc is (A - C)
    let oc = ray.origin() - sphere.center();

    // a x^2 + b x + c = y
    let a = ray.direction().dot(ray.direction());
    let b = 2.0 * ray.direction().dot(oc);
    let c = oc.dot(oc) - sphere.radius() * sphere.radius();

    // b^2 - 4 ac > 0 => equations has 2 roots
    let discriminant = b * b - 4.0 * a * c;

    discriminant > 0.0
}

/// Returns the color of the ray-tracing
///
/// When our ray hits a sphere, the color is red.
///
/// Background color is a simple gradient, which
/// linearly blends white and blue depending on the height of the y coordinate.
fn ray_color(ray: &Ray) -> Color {
    let sphere = &&Sphere {
        center: Point3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    };
    if ray_hit_sphere(ray, sphere) {
        return Color::red();
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Image

    // Use 16:9 aspect ratio
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_HEIGHT: u64 = 255;
    const IMAGE_WIDTH: u64 = (ASPECT_RATIO * IMAGE_HEIGHT as f64) as u64;

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

            // u: left 0.0 -> 1.0 right
            // v: botm 0.0 -> 1.0 up
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
