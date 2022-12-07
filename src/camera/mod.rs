mod camera_builder;
pub use camera_builder::CameraBuilder;
use rand::Rng;

use crate::{Point3, Ray, Vec3};
use std::{fmt::Display, ops::Range};

/// Ray-tracing camera
///
/// ## Viewport
///
/// ![Camera view up direction](https://raytracing.github.io/images/fig-1.15-cam-view-dir.jpg)
///
/// As shown in the figure, camera faces `look_at`, or `-w`.
/// Rays are cast from the origin to a projection plane `|w| = -1`.
/// Viewport height is `2h` and viewport width is `2h * aspect_ratio`.
///
/// ## Lens
///
/// A real camera has a complicated lens, but we use a thin lens approximation.
///
/// ![Camera lens model](https://raytracing.github.io/images/fig-1.17-cam-lens.jpg)
///
/// We don't need to simulate any of the inside of the camera. For the purpose of
/// rendering an image outside the camera, we start the rays from the lens, and send
/// them toward the focus plane (`focus_dist` away from the lens), where everything
/// on that plane is in perfect focus.
///
/// ![Camera focus plane](https://raytracing.github.io/images/fig-1.18-cam-film-plane.jpg)
#[derive(Debug, Clone)]
pub struct Camera {
    origin: Point3,
    /// Lower left corner of the viewport
    lower_left_corner: Point3,
    /// Horizontal vector of the viewport
    horizontal: Vec3<f64>,
    /// Vertical vector of the viewport
    vertical: Vec3<f64>,
    u: Vec3<f64>,
    v: Vec3<f64>,
    lens_radius: f64,
    /// Shutter open and close times
    time_range: Range<f64>,
}

impl Camera {
    /// Returns a ray that starts at the camera's origin and
    /// passes through the point `(u, v)` on the viewport.
    ///
    /// This function is used to cast rays through the scene.
    /// `u` and `v` are the coordinates of the point on the
    /// viewport, in the range of [0.0, 1.0].
    pub fn cast(&self, u: f64, v: f64) -> Ray {
        let random = Vec3::random_in_disk(self.lens_radius);
        let offset = self.u * random.x() + self.v * random.y();
        let time = rand::thread_rng().gen_range(self.time_range.clone());

        let origin = self.origin + offset;
        let destination = self.lower_left_corner + u * self.horizontal + v * self.vertical;
        let direction = destination - origin;

        Ray::new(origin, direction, time)
    }

    pub fn aspect_ratio(&self) -> f64 {
        self.horizontal.norm() / self.vertical.norm()
    }

    pub fn builder() -> CameraBuilder {
        CameraBuilder::new()
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
