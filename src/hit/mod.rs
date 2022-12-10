mod aabb;
mod hit_record;

pub use aabb::AABB;

use crate::Ray;
pub use hit_record::OutwardHitRecord;
pub use hit_record::AgainstRayHitRecord;

/// Trait for objects that can be hit by a ray
pub trait Hit: Sync + Send {
    /// Returns the hit record for the ray if it hits the object, otherwise None
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<crate::HitRecord>;

    /// Returns the bounding box of the object
    ///
    /// This function returns a option because some objects do not have a bounding box,
    /// such as infinite planes. Moving objects will have a bounding box that encloses
    /// the object at all times.
    fn bounding_box(&self, time_from: f64, time_to: f64) -> Option<AABB>;
}
