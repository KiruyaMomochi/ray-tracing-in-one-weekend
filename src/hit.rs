use crate::{Point3, Ray, Vec3};

pub struct HitRecord {
    /// Point of intersection
    pub point: Point3,
    /// Normal vector at point of intersection, pointing outward
    pub normal_outward: Vec3<f64>,
    /// Distance from ray origin to hit point
    pub t: f64,
}

impl HitRecord {
    pub fn front(&self, ray: &Ray) -> bool {
        ray.direction().dot(self.normal_outward) < 0.0
    }

    pub fn into_rayward(self, ray: &Ray) -> RaywardHitRecord {
        let front_face = self.front(ray);
        let normal_rayward = if front_face {
            self.normal_outward
        } else {
            -self.normal_outward
        };

        RaywardHitRecord {
            point: self.point,
            t: self.t,
            normal_rayward,
            front_face,
        }
    }
}

pub struct RaywardHitRecord {
    /// Point of intersection
    pub point: Point3,
    /// Normal vector at point of intersection, pointing against the ray
    pub normal_rayward: Vec3<f64>,
    /// Distance from ray origin to hit point
    pub t: f64,
    /// True if ray is outside the object
    front_face: bool,
}

pub trait Hit {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
