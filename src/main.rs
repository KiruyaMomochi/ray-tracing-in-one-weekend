use indicatif::ProgressBar;
use ray_tracing_in_one_weekend::{Color, Point3, Ray, Sphere, Camera};
use std::{
    fs,
    io::{BufWriter, Write},
};

/// Returns the color of the ray-tracing
///
/// When our ray hits a sphere, the color is red.
///
/// Background color is a simple gradient, which
/// linearly blends white and blue depending on the height of the y coordinate.
pub fn ray_color(ray: &Ray) -> Color {
    let sphere = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);
    if let Some(hit) = ray.hit(sphere, f64::NEG_INFINITY, f64::INFINITY) {
        // Obtain the unit normal vector: -1 <= . <= 1
        let normal = hit.normal.normalized();
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
