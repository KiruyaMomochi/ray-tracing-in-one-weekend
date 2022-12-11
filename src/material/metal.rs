use crate::{Color, Material, Ray, Vec3, hit::AgainstRayHitRecord};

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
    fn scatter(&self, ray: &Ray, hit_record: &AgainstRayHitRecord) -> Option<(Ray, Color)> {
        let reflected = ray
            .direction()
            .reflect(hit_record.normal_against_ray)
            .normalized();
        let direction = reflected + self.fuzziness * Vec3::random_in_unit_sphere();
        let scattered = Ray::new(hit_record.point, direction, ray.time());

        // if the ray is reflected towards the surface, then we scatter it
        if scattered.direction().dot(hit_record.normal_against_ray) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}
