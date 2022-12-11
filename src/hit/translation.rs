use crate::{Hit, Vec3, Ray};

use super::OutwardHitRecord;

/// Instance translation, i.e. moving an object in space
///
/// Instead of moving the object, we move the ray in the opposite direction.
#[derive(Debug, Clone)]
pub struct Translate<H: Hit> {
    object: H,
    offset: Vec3<f64>,
}

impl<H: Hit> Translate<H> {
    pub fn new(object: H, offset: Vec3<f64>) -> Self {
        Self { object, offset }
    }
}

impl<H: Hit> Hit for Translate<H> {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<OutwardHitRecord> {
        let ray = ray.move_origin_by(-self.offset);
        ray.hit(&self.object, t_min, t_max).map(|mut hit| {
            hit.point += self.offset;
            hit
        })
    }

    fn bounding_box(&self, time_from: f64, time_to: f64) -> Option<super::AABB> {
        self.object.bounding_box(time_from, time_to).map(|aabb| {
            aabb.move_by(self.offset)
        })
    }
}
