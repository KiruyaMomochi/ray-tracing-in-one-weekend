use crate::{Vec3, Point3};

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
