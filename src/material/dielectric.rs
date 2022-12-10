use crate::{Material, HitRecord, Ray, Color};

#[derive(Debug, Clone)]
pub struct Dielectric {
    index_of_refraction: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            index_of_refraction,
        }
    }

    /// [Schlick's approximation](https://en.wikipedia.org/wiki/Schlick%27s_approximation) for reflectance
    /// of a dielectric material.
    ///
    /// The specular reflection coefficient $R$ is given by:
    /// ```math
    /// R(\theta) = R_0 + (1 - R_0)(1 - \cos \theta)^5
    /// ```
    /// where
    /// ```
    /// R_0 = \frac{(n_1 - n_2)^2}{(n_1 + n_2)^2}
    /// ```
    /// and $\theta$ is the angle between the incident ray and the normal.
    /// And $n_1$ and $n_2$ are the indices of refraction of the two media.
    /// In our case one of the interfaces is air, so $n_1 = 1$.
    ///
    /// # Arguments
    /// * `cosine` - cosine of the angle between the incident ray and the normal
    /// * `index_of_refraction` - index of refraction of the material
    fn reflectance(cosine: f64, index_of_refraction: f64) -> f64 {
        let r0 = ((1.0 - index_of_refraction) / (1.0 + index_of_refraction)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let refraction_ratio = if hit_record.is_front() {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction / 1.0
        };

        let unit_direction = ray.direction().normalized();
        // `theta` is the angle from the normal
        // TODO: normal_outward or normal_against_ray
        let cos_theta = (-unit_direction)
            .dot(hit_record.normal_against_ray)
            .min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        // TODO: or refraction_ratio?
        let reflectance = Self::reflectance(cos_theta, self.index_of_refraction);
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let will_reflect = reflectance > rand::random::<f64>();

        let direction = if cannot_refract || will_reflect {
            // Refraction is not possible, must reflect
            unit_direction.reflect(hit_record.normal_against_ray)
        } else {
            unit_direction.refract(hit_record.normal_against_ray, refraction_ratio)
        };

        let scattered = Ray::new(hit_record.point, direction, ray.time());

        // attenuation is always 1 as the glass surface absorbs nothing
        Some((scattered, Color::white()))
    }
}
