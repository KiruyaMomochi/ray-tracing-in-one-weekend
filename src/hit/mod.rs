mod aabb;
mod bvh;

use std::sync::Arc;
pub use bvh::BVH;
pub use aabb::AABB;

use crate::{Material, Point3, Ray, Vec3};

#[derive(Debug, Clone)]
pub struct OutwardHitRecord {
    /// Point of intersection
    pub point: Point3,
    /// Normal vector at point of intersection, pointing outward
    pub normal_outward: Vec3<f64>,
    /// Distance from ray origin to hit point
    pub t: f64,
    /// Material of the object hit
    pub material: Arc<dyn Material>,
    /// True if ray is outside the object
    pub front_face: bool,
}

impl OutwardHitRecord {
    pub fn new(
        point: Point3,
        ray: &Ray,
        normal_outward: Vec3<f64>,
        t: f64,
        material: Arc<dyn Material>,
    ) -> Self {
        let front_face = ray.direction().dot(normal_outward) < crate::vec3::EPSILON;
        Self {
            point,
            normal_outward,
            t,
            material,
            front_face,
        }
    }

    pub fn front(&self) -> bool {
        self.front_face
    }

    pub fn normal_against_ray(&self) -> Vec3<f64> {
        if self.front() {
            self.normal_outward
        } else {
            -self.normal_outward
        }
    }

    pub fn into_against_ray(self) -> AgainstRayHitRecord {
        let front_face = self.front();
        let normal_against_ray = self.normal_against_ray();

        AgainstRayHitRecord {
            point: self.point,
            t: self.t,
            material: self.material,
            normal_against_ray,
            front_face,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AgainstRayHitRecord {
    /// Point of intersection
    pub point: Point3,
    /// Normal vector at point of intersection, pointing against the ray
    pub normal_against_ray: Vec3<f64>,
    /// Distance from ray origin to hit point
    pub t: f64,
    /// Material of the object hit
    pub material: Arc<dyn Material>,
    /// True if ray is outside the object
    pub front_face: bool,
}

impl AgainstRayHitRecord {
    pub fn is_front(&self) -> bool {
        self.front_face
    }

    pub fn normal_outward(&self) -> Vec3<f64> {
        if self.front_face {
            self.normal_against_ray
        } else {
            -self.normal_against_ray
        }
    }
}

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
