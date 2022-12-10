mod dielectric;
mod lambertian;
mod metal;

pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use metal::Metal;

use crate::{Color, HitRecord, Point3, Ray};
use std::fmt::Debug;

/// A material that can be hit by a ray
pub trait Material: Debug + Sync + Send {
    /// Scatter a ray, returning the ray scattered and the attenuation of the ray.
    ///
    /// For details, see [Volume Scattering Process](https://www.pbr-book.org/3ed-2018/Volume_Scattering/Volume_Scattering_Processes)
    /// in the Physically Based Rendering book.
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)>;

    /// Return the emitted color of material. For non-emissive materials, this
    /// is always black.
    fn emit(&self, _u: f64, _v: f64, _point: Point3) -> Color {
        Color::black()
    }
}
