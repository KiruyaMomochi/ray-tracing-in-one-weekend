use std::fmt::Display;

use crate::{Point3, Vec3, Ray};

const FOCAL_LENGTH: f64 = 1.0;

/// Simple camera
#[derive(Debug, Clone)]
pub struct Camera {
    origin: Point3,
    /// Lower left corner of the viewport
    lower_left_corner: Point3,
    /// x-asis
    horizontal: Vec3<f64>,
    /// y-axis
    vertical: Vec3<f64>,
    /// Distance between projection plane and projection point
    focal_length: f64,
}

impl Camera {
    /// Creates a new [`Camera`] with the given aspect ratio.
    /// 
    /// ![Camera viewing geometry](https://raytracing.github.io/images/fig-1.14-cam-view-geom.jpg)
    /// 
    /// As shown in the figure, rays are cast from the origin to a projection plane `z = -1`.
    /// Viewport height is `2h` and viewport width is `2h * aspect_ratio`.
    /// 
    /// # Arguments
    /// * `vertical_fov` - Vertical field of view in degrees
    /// * `aspect_ratio` - Aspect ratio of the viewport
    pub fn new(vertical_fov: f64, aspect_ratio: f64) -> Self {
        // convert vertical fov to radians
        let theta = vertical_fov.to_radians();
        let h = (theta / 2.0).tan();
        
        // virtual viewport to pass scene rays
        let viewport_height = 2.0 * h;
        let viewport_width = viewport_height * aspect_ratio;

        // the focal length is the distance between the projection point and the image plane
        // this may not the same as the distance between the projection point and the viewport
        let focal_length = FOCAL_LENGTH;

        // origin point defaults to be (0, 0, 0)
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
