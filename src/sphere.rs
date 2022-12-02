use std::sync::Arc;

use crate::{hit::OutwardHitRecord, Hit, Material, Point3, Vec3, HitRecord};

#[derive(Debug, Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }

    pub fn center(&self) -> Vec3<f64> {
        self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl Hit for Sphere {
    /// Returns a [`HitRecord`] if `ray` hit to a point in `[t_min, t_max]`,
    /// or `None` if does not hit
    ///
    /// The equation of the sphere in vector form is
    ///
    ///     (P - C) . (P - C) = r^2
    ///
    /// where C is the vector from sphere center, and P is the point.
    ///
    /// When P is the ray P(t) = A + tb for some t,  the equation expands to
    ///
    ///     (A + t b - C) . (A + t b - C) = r^2
    ///
    /// where b: ray.direction, A: ray.origin, C: sphere.center.
    ///
    /// In quadratic form
    ///
    ///     (b.b) t^2 + (2b.(A-C)) t + ((A-C).(A-C) - r^2) = 0
    ///
    fn hit(&self, ray: &crate::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let sphere = self;

        // oc is (A - C)
        let oc = ray.origin() - sphere.center();

        // a x^2 + b x + c = y

        // let b = 2 * half because
        // - discriminant has a factor of 2 and,
        // - b part also has a factor of 2

        let direction = ray.direction();
        let a = direction.len_squared();
        let h = oc.dot(direction); // b / 2
        let c = oc.len_squared() - sphere.radius() * sphere.radius();

        // b^2 - 4 ac > 0 => h^2 - ac > 0 => equations has 2 roots
        let discriminant_h = h * h - a * c;

        if discriminant_h < 0.0 {
            // Does not hit any point
            return None;
        };

        // (-b +- sqrt(dis)) / (2a) => (-h +- sqrt(dis_h)) / a
        let discriminant_s = discriminant_h.sqrt();
        let roots = [(-h - discriminant_s) / a, (-h + discriminant_s) / a];

        // Compute the roots and find acceptable one
        let t = roots
            .into_iter()
            .find(|root| root >= &t_min && root <= &t_max)?;

        let point = ray.at(t);
        let normal_outward = (point - sphere.center()) / sphere.radius();

        // if direction.x() == 0.0 && direction.y() == 0.0 {
        //     println!("ray direction: {}", direction);
        //     println!("roots: {:?}", roots);
        //     println!("point: {}, normal: {}", point, normal);
        // }

        Some(OutwardHitRecord::new(point, ray, normal_outward, t, self.material.clone()).into_against_ray())
    }
}
