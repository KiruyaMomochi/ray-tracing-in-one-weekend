use crate::{Camera, Point3, Vec3};
use std::ops::Range;

macro_rules! builder_methods {
    ($($name:ident: $type:ty$(as $extra:tt)?),*) => {
        $(
            builder_methods!(@impl $name: $type$(as $extra)?);
        )*
    };
    (@impl $name:ident: $type:ty as vec3) => {
        pub fn $name(mut self, x: f64, y: f64, z: f64) -> Self {
            self.$name = <$type>::new(x, y, z);
            self
        }
    };
    (@impl $name:ident: $type:ty as range) => {
        pub fn $name(mut self, start: f64, end: f64) -> Self {
            self.$name = start..end;
            self
        }
    };
    (@impl $name:ident: $type:ty) => {
        pub fn $name(mut self, $name: $type) -> Self {
            self.$name = $name;
            self
        }
    }
}

#[derive(Debug, Clone)]
pub struct CameraBuilder {
    /// Camera origin
    look_from: Point3,
    /// Point the camera is looking at
    look_at: Point3,
    /// The up direction of the camera. We will project it onto the plane
    /// perpendicular to `look_at` and normalize it.
    view_up: Vec3<f64>,
    /// Vertical field of view in degrees
    vertical_field_of_view: f64,
    /// Aspect ratio of the viewport
    aspect_ratio: f64,
    /// The aperture of the camera. This is the diameter of the lens.
    aperture: f64,
    /// The distance between the projection point and the plane
    /// where everything is in perfect focus.
    focus_distance: Option<f64>,
    /// Range of time values for the camera to generate rays
    time_range: Range<f64>,
}

impl CameraBuilder {
    pub fn new() -> Self {
        Self {
            look_from: Point3::new(0.0, 0.0, 0.0),
            look_at: Point3::new(0.0, 0.0, -1.0),
            view_up: Vec3::new(0.0, 1.0, 0.0),
            vertical_field_of_view: 90.0,
            aspect_ratio: 16.0 / 9.0,
            aperture: 0.0,
            focus_distance: None,
            time_range: 0.0..1.0,
        }
    }

    builder_methods! {
        look_from: Point3 as vec3,
        look_at: Point3 as vec3,
        view_up: Vec3<f64> as vec3,
        vertical_field_of_view: f64,
        aspect_ratio: f64,
        aperture: f64,
        time_range: Range<f64> as range
    }

    pub fn focus_distance(mut self, focus_distance: f64) -> Self {
        self.focus_distance = Some(focus_distance);
        self
    }

    pub fn lens_radius(mut self, lens_radius: f64) -> Self {
        self.aperture = 2.0 * lens_radius;
        self
    }

    pub fn build(self) -> Camera {
        let Self {
            look_from,
            look_at,
            view_up,
            vertical_field_of_view,
            aspect_ratio,
            aperture,
            focus_distance,
            time_range,
        } = self;

        let focus_distance = focus_distance.unwrap_or_else(|| (look_at - look_from).norm());

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

        Camera {
            origin: look_from,
            horizontal,
            vertical,
            lower_left_corner,
            u: camera_u,
            v: camera_v,
            lens_radius: aperture / 2.0,
            time_range,
        }
    }
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self::new()
    }
}
