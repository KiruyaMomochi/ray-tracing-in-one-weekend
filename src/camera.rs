use std::fmt::Display;

use crate::{Point3, Ray, Vec3};

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
}

impl Camera {
    /// Creates a new [`Camera`] with the given aspect ratio.
    ///
    /// ![Camera view up direction](https://raytracing.github.io/images/fig-1.15-cam-view-dir.jpg)
    ///
    /// As shown in the figure, camera faces `look_at`, or `-w`.
    /// Rays are cast from the origin to a projection plane `|w| = -1`.
    /// Viewport height is `2h` and viewport width is `2h * aspect_ratio`.
    ///
    /// # Arguments
    /// * `look_from` - Camera origin
    /// * `look_at` - Point camera is looking at
    /// * `view_up` - The up direction of the camera. We will project it onto the plane
    /// perpendicular to `look_at` and normalize it.
    /// * `vertical_fov` - Vertical field of view in degrees
    /// * `aspect_ratio` - Aspect ratio of the viewport
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        view_up: Vec3<f64>,
        vertical_field_of_view: f64,
        aspect_ratio: f64,
    ) -> Self {
        // convert vertical fov to radians
        let theta = vertical_field_of_view.to_radians();
        let h = (theta / 2.0).tan();

        // virtual viewport to pass scene rays
        let viewport_height = 2.0 * h;
        let viewport_width = viewport_height * aspect_ratio;

        // orthonormal basis (u, v, w) to define the camera coordinate system
        let camera_w = (look_from - look_at).normalized();
        // project view_up onto the plane orthogonal to camera_w
        let camera_u = view_up.cross(camera_w).normalized();
        let camera_v = camera_w.cross(camera_u);

        let horizontal = viewport_width * camera_u;
        let vertical = viewport_height * camera_v;
        let lower_left_corner =
            look_from - horizontal / 2.0 - vertical / 2.0 - camera_w;

        Self {
            origin: look_from,
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
    origin: [{}],
    horizontal: [{}]
    vertical: {},
    lower left corner: {}
}}",
            self.origin, self.horizontal, self.vertical, self.lower_left_corner
        )
    }
}
