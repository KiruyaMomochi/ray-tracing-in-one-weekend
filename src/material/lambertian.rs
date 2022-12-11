use crate::Vec3;
use crate::hit::AgainstRayHitRecord;
use crate::texture::SolidColor;
use crate::{Material, Ray, Color, texture::Texture};

/// Diffuse material, which can either scatter always and attenuate by its
/// reflectance R, or it can scatter with no attenuation but absorb the
/// fraction 1-R of the rays, or it could be a mixture of the two.
#[derive(Debug)]
pub struct Lambertian<T: Texture> {
    /// The texture of the material
    albedo: T,
}

impl<T: Texture> Lambertian<T> {
    pub fn new(albedo: T) -> Self {
        Self { albedo }
    }
}

impl Lambertian<SolidColor> {
    pub fn new_solid(albedo: Color) -> Self {
        Self::new(SolidColor::new(albedo))
    }
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, ray: &Ray, hit_record: &AgainstRayHitRecord) -> Option<(Ray, Color)> {
        let scatter_direction =
            hit_record.normal_against_ray + Vec3::random_in_unit_sphere().normalized();

        // scatter_direction near zero may leads to infinite or NaNs, which
        // may cause problems later on. So we need to handle this case.
        let direction = if scatter_direction.is_near_zero() {
            hit_record.normal_against_ray
        } else {
            scatter_direction
        };
        let scattered = Ray::new(hit_record.point, direction, ray.time());
        let attenuation = self.albedo.color(hit_record.point, hit_record.u, hit_record.v);

        Some((scattered, attenuation))
    }
}
