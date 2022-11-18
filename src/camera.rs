use std::fmt::Display;

use crate::{Point3, Vec3, Ray};

/// Simple camera
#[derive(Debug, Clone)]
pub struct Camera {
    origin: Point3,
    /// lower left corner of the viewport
    lower_left_corner: Point3,
    /// x-asis
    horizontal: Vec3<f64>,
    /// y-axis
    vertical: Vec3<f64>,
    /// Distance between projection plane and projection point
    focal_length: f64,
}

impl Camera {
    pub fn new(viewport_height: f64, aspect_ratio: f64, focal_length: f64) -> Self {
        // Virtual viewport to pass scene rays
        let viewport_width = viewport_height * aspect_ratio;

        // Origin point defaults to be (0, 0, 0)
        let origin = Point3::zero();

        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        Self {
            focal_length,
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    /// Returns a ray that starts at the camera's origin and
    /// passes through the point `(u, v)` on the viewport.
    /// 
    /// This function is used to cast rays through the scene.
    /// `u` and `v` are the coordinates of the point on the
    /// viewport, in the range of [0.0, 1.0].
    pub fn cast(&self, u: f64, v: f64) -> Ray {
        let direction =
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin;

        Ray::new(self.origin, direction)
    }

    pub fn aspect_ratio(&self) -> f64 {
        self.horizontal.len() / self.vertical.len()
    }
}

impl Display for Camera {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Camera {{
    focal: {:.2},
    origin: [{}],
    horizontal: [{}]
    vertical: {},
    lower left corner: {}
}}",
            self.focal_length, self.origin, self.horizontal, self.vertical, self.lower_left_corner
        )
    }
}
