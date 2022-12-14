use std::sync::Arc;

use log::debug;

use crate::{
    hit::OutwardHitRecord, material::Isotropic, texture::{Texture, SolidColor}, Hit, Vec3, Color,
};

/// A volume of constant density.
///
/// When the ray hits the boundary and passes through the medium,
/// it may scatter at any point. The denser the medium, the more likely
/// the ray will scatter. The probability of scattering is
/// ```ignore
/// probability = density * distance
/// ```
/// where `density` is a constant. If the distance is outside the range
/// `[t_min, t_max]`, the ray will not scatter.
///
/// In this material, the place where the ray scatters is sampled by
/// `t = -density * ln(random) + t_min`, where `random` is a random number
/// in the range `[0, 1)`.
#[derive(Debug, Clone)]
pub struct ConstantMedium<H: Hit, T: Texture> {
    /// Object to be filled with the medium.
    boundary: H,
    /// Material of the medium.
    material: Arc<Isotropic<T>>,
    /// Negative reciprocal of the density of the medium
    negative_reciprocal_density: f64,
}

impl<H: Hit, T: Texture> ConstantMedium<H, T> {
    /// Create a new volume of constant density.
    ///
    /// # Arguments
    ///
    /// * `boundary` - Object to be filled with the medium.
    /// * `texture` - Texture of the medium.
    /// * `density` - Optional density of the medium.
    ///
    pub fn new(boundary: H, texture: T, density: f64) -> Self {
        Self {
            boundary,
            material: Arc::new(Isotropic::new(texture)),
            negative_reciprocal_density: -1.0 / density,
        }
    }
}

impl<H: Hit> ConstantMedium<H, SolidColor> {
    pub fn new_solid(boundary: H, color: Color, density: f64) -> Self {
        Self::new(boundary, SolidColor::new(color), density)
    }
}

impl<H: Hit, T: Texture + 'static> Hit for ConstantMedium<H, T> {
    fn hit(&self, ray: crate::Ray, t_min: f64, t_max: f64) -> Option<super::OutwardHitRecord> {
        // ray doesn't hit the boundary -> ray does not hit the medium.
        // ray does hit the boundary    -> hit point is the first point
        // NOTE: we should not limit the range of t, because the ray origin
        // may be inside the medium.
        let hit_record = ray.clone().hit(&self.boundary, f64::NEG_INFINITY, f64::INFINITY)?;

        // update t_min
        let t_min = t_min.max(hit_record.t);

        // ray doesn't hit the other side -> does not hit the medium.
        // ray does hit the other side    -> hit point is the second point
        // skip a small amount of distance to avoid self-intersection or a tiny plane.
        let hit_record = ray.clone().hit(&self.boundary, t_min + 1e-5, t_max)?;

        // update t_max
        let t_max = t_max.min(hit_record.t);

        // if t_min >= t_max, the ray will not hit the medium, because it travels
        // in the opposite direction.
        if t_min >= t_max {
            return None;
        };

        // t_min should be positive?
        assert!(t_min >= -f64::EPSILON);

        // find the distance the ray travels through the medium
        let ray_length = ray.direction().norm();
        let distance_traveled = (t_max - t_min) * ray_length;
        // generate random distance the ray should scatter
        let distance_to_scatter = self.negative_reciprocal_density * rand::random::<f64>().ln();
        // if distance to scatter is greater than the distance traveled,
        // the ray will not scatter.
        debug!("       distance traveled {} to scatter {}", distance_to_scatter, distance_traveled);
        if distance_to_scatter > distance_traveled {
            return None;
        };

        // find the point where our ray really scatters
        let t = t_min + distance_to_scatter / ray_length;
        let point = ray.at(t);

        // as in the isotropic material, the ray will scatter in a random direction,
        // hence the normal vector is not used, we use a NaN vector instead.
        let normal_outward = Vec3::constant(f64::NAN);
        // uv is not used too
        let uv = (f64::NAN, f64::NAN);

        Some(OutwardHitRecord::new(
            point,
            &ray,
            normal_outward,
            t,
            self.material.clone(),
            uv,
        ))
    }

    fn bounding_box(&self, time_from: f64, time_to: f64) -> Option<super::AABB> {
        self.boundary.bounding_box(time_from, time_to)
    }
}
