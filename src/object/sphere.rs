use std::{f64::consts::PI, ops::Range, sync::Arc};

use crate::{
    hit::{OutwardHitRecord, AABB},
    Hit, Material, Point3, Ray, Vec3,
};

#[derive(Debug, Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Arc<dyn Material>,
}

/// Compute the surface coordinates (u, v) from hitpoint P
fn to_sphere_uv(normal_outward: &Vec3<f64>) -> (f64, f64) {
    let (r, theta, phi) = normal_outward.to_spherical().into_tuple();
    let u = phi / (2.0 * PI) / r;
    let v = theta / PI / r;
    (u, v)
}

impl Sphere {
    pub fn new<P: Into<Point3>>(center: P, radius: f64, material: Arc<dyn Material>) -> Self {
        Self {
            center: center.into(),
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

#[derive(Debug, Clone)]
pub struct MovingSphere {
    time_from: f64,
    center_from: Point3,
    time_to: f64,
    center_to: Point3,
    radius: f64,
    material: Arc<dyn Material>,
}

impl MovingSphere {
    pub fn new(
        time_range: Range<f64>,
        center_from: Point3,
        center_to: Point3,
        radius: f64,
        material: Arc<dyn Material>,
    ) -> Self {
        Self {
            time_from: time_range.start,
            center_from,
            time_to: time_range.end,
            center_to,
            radius,
            material,
        }
    }

    pub fn center(&self, time: f64) -> Vec3<f64> {
        let ratio = (time - self.time_from) / (self.time_to - self.time_from);
        self.center_from.lerp(self.center_to, ratio)
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl Sphere {
    pub fn into_moving(self, time_range: Range<f64>, center_to: Point3) -> MovingSphere {
        MovingSphere::new(
            time_range,
            self.center,
            center_to,
            self.radius,
            self.material,
        )
    }
}

fn hit(
    center: Point3,
    radius: f64,
    material: Arc<dyn Material>,
    ray: &Ray,
    t_min: f64,
    t_max: f64,
) -> Option<OutwardHitRecord> {
    // oc is (A - C)
    let oc = ray.origin() - center;

    // a x^2 + b x + c = y

    // let b = 2 * half because
    // - discriminant has a factor of 2 and,
    // - b part also has a factor of 2

    let direction = ray.direction();
    let a = direction.len_squared();
    let h = oc.dot(direction); // b / 2
    let c = oc.len_squared() - radius * radius;

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
    let normal_outward = (point - center) / radius;
    let uv = to_sphere_uv(&normal_outward);

    Some(OutwardHitRecord::new(
        point,
        ray,
        normal_outward,
        t,
        material,
        uv,
    ))
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
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<OutwardHitRecord> {
        let center = self.center();
        let radius = self.radius();
        let material = self.material.clone();

        hit(center, radius, material, &ray, t_min, t_max)
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<AABB> {
        let center = self.center();
        let offset = Vec3::constant(self.radius());
        Some(AABB::new(center - offset, center + offset))
    }
}

impl Hit for MovingSphere {
    /// Returns a [`HitRecord`] if `ray` hit to a point in `[t_min, t_max]`,
    /// or `None` if does not hit.
    ///
    /// Refer to [`Sphere::hit`] for the equation.
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<OutwardHitRecord> {
        let center = self.center(ray.time());
        let radius = self.radius();
        let material = self.material.clone();

        hit(center, radius, material, &ray, t_min, t_max)
    }

    fn bounding_box(&self, time_from: f64, time_to: f64) -> Option<AABB> {
        let center_from = self.center(time_from);
        let center_to = self.center(time_to);
        let offset = Vec3::constant(self.radius());

        let box_from = AABB::new(center_from - offset, center_from + offset);
        let box_to = AABB::new(center_to - offset, center_to + offset);

        Some(box_from.merge(&box_to))
    }
}
