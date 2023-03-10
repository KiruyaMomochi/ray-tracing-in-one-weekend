use std::sync::Arc;
use crate::{Material, Point3, Ray, Vec3, Color};

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
    /// Surface coordinates of the hit point
    pub u: f64,
    /// Surface coordinates of the hit point
    pub v: f64,
    /// Color of emitted light from the object at hit point.
    /// This may larger than 1.0, which means the object is brighter.
    pub emitted: Color,
}

impl OutwardHitRecord {
    pub fn new(
        point: Point3,
        ray: &Ray,
        normal_outward: Vec3<f64>,
        t: f64,
        material: Arc<dyn Material>,
        (u, v): (f64, f64),
    ) -> Self {
        assert!(point.is_valid_point());
        let emitted = material.emit(point, u, v);
        let front_face = ray.direction().dot(normal_outward) < crate::vec3::Float::EPSILON;
        Self {
            point,
            normal_outward,
            t,
            material,
            front_face,
            u,
            v,
            emitted,
        }
    }

    pub fn is_front(&self) -> bool {
        self.front_face
    }

    pub fn normal_against_ray(&self) -> Vec3<f64> {
        if self.is_front() {
            self.normal_outward
        } else {
            -self.normal_outward
        }
    }

    pub fn into_against_ray(self) -> AgainstRayHitRecord {
        let front_face = self.is_front();
        let normal_against_ray = self.normal_against_ray();

        AgainstRayHitRecord {
            point: self.point,
            t: self.t,
            material: self.material,
            normal_against_ray,
            front_face,
            u: self.u,
            v: self.v,
            emitted: self.emitted,
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
    /// Surface coordinates of the hit point
    pub u: f64,
    /// Surface coordinates of the hit point
    pub v: f64,
    /// Color of emitted light from the object at hit point.
    /// This may larger than 1.0, which means the object is brighter.
    pub emitted: Color,
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
