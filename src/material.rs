use crate::{hit::HitRecord, Color, Ray, Vec3};
use std::fmt::Debug;

/// A material that can be hit by a ray
pub trait Material
where
    Self: Debug,
{
    /// Scatter a ray, returning the ray scattered and the attenuation of the ray.
    ///
    /// For details, see [Volume Scattering Process](https://www.pbr-book.org/3ed-2018/Volume_Scattering/Volume_Scattering_Processes)
    /// in the Physically Based Rendering book.
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)>;
}

/// Diffuse material, which can either scatter always and attenuate by its
/// reflectance R, or it can scatter with no attenuation but absorb the
/// fraction 1-R of the rays, or it could be a mixture of the two.
#[derive(Debug, Clone)]
pub struct Lambertian {
    /// The color reflected by the surface
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let scatter_direction = hit_record.normal_outward + Vec3::random_in_sphere().normalized();

        // scatter_direction near zero may leads to infinite or NaNs, which
        // may cause problems later on. So we need to handle this case.
        let direction = if scatter_direction.near_zero() {
            hit_record.normal_outward
        } else {
            scatter_direction
        };
        let scattered = Ray::new(hit_record.point, direction);

        Some((scattered, self.albedo))
    }
}

#[derive(Debug, Clone)]
pub struct Metal {
    /// The color reflected by the surface
    albedo: Color,
    /// Fuzziness of the material, zero means no perturbation.
    fuzziness: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzziness: f64) -> Self {
        Self { albedo, fuzziness }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = ray
            .direction()
            .reflect(hit_record.normal_outward)
            .normalized();
        let direction = reflected + self.fuzziness * Vec3::random_in_sphere();
        let scattered = Ray::new(hit_record.point, direction);

        // if the ray is reflected towards the surface, then we scatter it
        if scattered.direction().dot(hit_record.normal_outward) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}
