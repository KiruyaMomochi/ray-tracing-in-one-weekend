use crate::{Ray, Point3, Vec3};

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3<f64>,
    pub t: f64
}

pub trait Hit {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
