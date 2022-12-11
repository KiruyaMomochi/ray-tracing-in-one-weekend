mod dielectric;
mod lambertian;
mod metal;
mod diffuse_light;

pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use metal::Metal;
pub use diffuse_light::DiffuseLight;

use crate::{Color, Point3, Ray, hit::AgainstRayHitRecord};
use std::fmt::Debug;

/// A material that can be hit by a ray
pub trait Material: Debug + Sync + Send {
    /// Scatter a ray, returning the ray scattered and the attenuation of the ray.
    ///
    /// For details, see [Volume Scattering Process](https://www.pbr-book.org/3ed-2018/Volume_Scattering/Volume_Scattering_Processes)
    /// in the Physically Based Rendering book.
    fn scatter(&self, ray: &Ray, hit_record: &AgainstRayHitRecord) -> Option<(Ray, Color)>;

    /// Return the emitted color of material. For non-emissive materials, this
    /// is always black.
    ///
    /// # Arguments
    ///
    /// * `point` - The point on the surface of the object.
    /// * `u`, `v` - The texture coordinates corresponding to the point.
    #[allow(unused_variables)] // This is a default implementation, so the arguments may not be used.
    fn emit(&self, point: Point3, u: f64, v: f64) -> Color {
        Color::BLACK
    }
}
