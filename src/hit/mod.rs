mod aabb;
mod hit_record;
pub mod translation;
pub mod rotation;

pub use aabb::AABB;

use crate::Ray;
pub use hit_record::AgainstRayHitRecord;
pub use hit_record::OutwardHitRecord;

/// Trait for objects that can be hit by a ray
pub trait Hit: Sync + Send {
    /// Returns the hit record for the ray if it hits the object, otherwise None
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<OutwardHitRecord>;

    /// Returns the bounding box of the object
    ///
    /// This function returns a option because some objects do not have a bounding box,
    /// such as infinite planes. Moving objects will have a bounding box that encloses
    /// the object at all times.
    fn bounding_box(&self, time_from: f64, time_to: f64) -> Option<AABB>;
}

impl<H: Hit> Hit for Box<H> {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<OutwardHitRecord> {
        self.as_ref().hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, time_from: f64, time_to: f64) -> Option<AABB> {
        self.as_ref().bounding_box(time_from, time_to)
    }
}

impl Hit for Box<dyn Hit> {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<OutwardHitRecord> {
        self.as_ref().hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, time_from: f64, time_to: f64) -> Option<AABB> {
        self.as_ref().bounding_box(time_from, time_to)
    }
}

impl<H: Hit> Hit for [H] {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<OutwardHitRecord> {
        // https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by
        self.iter()
            .filter_map(|obj| obj.hit(ray.clone(), t_min, t_max))
            .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap()) // unwarp: t != NaN
    }

    fn bounding_box(&self, time_from: f64, time_to: f64) -> Option<AABB> {
        self.iter()
            .filter_map(|obj| obj.bounding_box(time_from, time_to))
            .reduce(|a, b| a.merge(&b))
    }
}

impl<H: Hit> Hit for Vec<H> {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<OutwardHitRecord> {
        self.as_slice().hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, time_from: f64, time_to: f64) -> Option<AABB> {
        self.as_slice().bounding_box(time_from, time_to)
    }
}
