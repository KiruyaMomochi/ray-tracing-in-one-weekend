mod ray;
mod vec3;
mod sphere;

pub use vec3::{Vec3, Color, Point3};
pub use ray::Ray;
pub use sphere::Sphere;

/// Simple camera
#[derive(Debug, Clone)]
pub struct Camera {
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

/// Returns a hit point if ray hits sphere, otherwise `None`.
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
/// where b: ray.direction, A: ray.origin, C: sphere.center.
/// 
/// In quadratic form
///
///     (b.b) t^2 + (2b.(A-C)) t + ((A-C).(A-C) - r^2) = 0
///
pub fn ray_hit_sphere(ray: &Ray, sphere: &Sphere) -> Option<Point3> {
    // oc is (A - C)
    let oc = ray.origin() - sphere.center();

    // a x^2 + b x + c = y

    // let b = 2 * half because
    // - discriminant has a factor of 2 and,
    // - b part also has a factor of 2

    let a = ray.direction().len_squared();
    let h = ray.direction().dot(oc); // b / 2
    let c = oc.len_squared() - sphere.radius() * sphere.radius();

    // b^2 - 4 ac > 0 => h^2 - ac > 0 => equations has 2 roots
    let discriminant_h = h * h - a * c;
    
    if discriminant_h < 0.0 {
        // Does not hit any point
        None
    } else {
        // Compute the first root:
        // (-b - sqrt(dis)) / (2a) => (-h - sqrt(dis_h)) / a
        let root = (-h - discriminant_h.sqrt()) / a;
        Some(ray.at(root))
    }
}
