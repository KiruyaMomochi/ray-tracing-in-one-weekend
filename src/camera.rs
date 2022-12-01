use std::fmt::Display;

use crate::{Point3, Ray, Vec3};

/// Simple camera
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
}

impl Camera {
    /// Creates a new [`Camera`].
    ///
    /// # Arguments
    /// * `look_from` - Camera origin
    /// * `look_at` - Point camera is looking at
    /// * `view_up` - The up direction of the camera. We will project it onto the plane
    ///               perpendicular to `look_at` and normalize it.
    /// * `vertical_fov` - Vertical field of view in degrees
    /// * `aspect_ratio` - Aspect ratio of the viewport
    /// * `aperture` - The aperture of the camera. This is the diameter of the lens.
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        view_up: Vec3<f64>,
        vertical_field_of_view: f64,
        aspect_ratio: f64,
        aperture: f64,
    ) -> Self {
        let focus_distance = (look_from - look_at).len();
        Self::new_focused(
            look_from,
            look_at,
            view_up,
            vertical_field_of_view,
            aspect_ratio,
            aperture,
            focus_distance,
        )
    }

    /// Creates a new [`Camera`] with a specific focus distance.
    ///
    /// # Arguments
    /// * `look_from` - Camera origin
    /// * `look_at` - Point camera is looking at
    /// * `view_up` - The up direction of the camera. We will project it onto the plane
    ///               perpendicular to `look_at` and normalize it.
    /// * `vertical_fov` - Vertical field of view in degrees
    /// * `aspect_ratio` - Aspect ratio of the viewport
    /// * `aperture` - The aperture of the camera. This is the diameter of the lens.
    /// * `focus_distance` - The distance between the projection point and the plane
    ///                      where everything is in perfect focus.
    ///
    /// # Details
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
    pub fn new_focused(
        look_from: Point3,
        look_at: Point3,
        view_up: Vec3<f64>,
        vertical_field_of_view: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_distance: f64,
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

        // note: camera faces -w
        let focus_plane_center = look_from + focus_distance * -camera_w;
        let horizontal = focus_distance * viewport_width * camera_u;
        let vertical = focus_distance * viewport_height * camera_v;
        let lower_left_corner = focus_plane_center - horizontal / 2.0 - vertical / 2.0;

        Self {
            origin: look_from,
            horizontal,
            vertical,
            lower_left_corner,
            u: camera_u,
            v: camera_v,
            lens_radius: aperture / 2.0,
        }
    }

    /// Returns a ray that starts at the camera's origin and
    /// passes through the point `(u, v)` on the viewport.
    ///
    /// This function is used to cast rays through the scene.
    /// `u` and `v` are the coordinates of the point on the
    /// viewport, in the range of [0.0, 1.0].
    pub fn cast(&self, u: f64, v: f64) -> Ray {
        let random = Vec3::random_in_disk(self.lens_radius);
        let offset = self.u * random.x() + self.v * random.y();

        let origin = self.origin + offset;
        let destination = self.lower_left_corner + u * self.horizontal + v * self.vertical;
        let direction = destination - origin;

        Ray::new(origin, direction)
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
