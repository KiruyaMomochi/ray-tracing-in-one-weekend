use crate::{texture::Texture, Material, Ray, Vec3};

/// Isotropic material, which reflects light equally in all directions.
#[derive(Debug, Clone)]
pub struct Isotropic<T: Texture> {
    /// The texture of the material
    albedo: T,
}

impl<T: Texture> Isotropic<T> {
    pub fn new(albedo: T) -> Self { Self { albedo } }
}

impl<T: Texture> Material for Isotropic<T> {
    fn scatter(&self, ray: &crate::Ray, hit_record: &crate::hit::AgainstRayHitRecord) -> Option<(crate::Ray, crate::Color)> {
        let ray = Ray::new(hit_record.point, Vec3::random_in_unit_sphere(), ray.time());
        let attenuation = self.albedo.color(hit_record.point, hit_record.u, hit_record.v);
        Some((ray, attenuation))
    }
}
