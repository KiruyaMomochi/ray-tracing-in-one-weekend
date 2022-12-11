use crate::{Hit, hit::{AABB, OutwardHitRecord}, Ray};

// Vec<Box<dyn trait>> has an implict 'static lifetime
// https://stackoverflow.com/questions/70717050/why-do-i-need-static-lifetime-here-and-how-to-fix-it
// https://users.rust-lang.org/t/box-with-a-trait-object-requires-static-lifetime/35261/2
pub struct World(Vec<Box<dyn Hit>>);

impl World {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn from_vec(hits: Vec<Box<dyn Hit>>) -> Self {
        Self(hits)
    }

    pub fn add<T: Hit + 'static>(&mut self, object: T) {
        self.0.push(Box::new(object));
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

impl Hit for World {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<OutwardHitRecord> {
        self.0.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, time_from: f64, time_to: f64) -> Option<AABB> {
        self.0.bounding_box(time_from, time_to)
    }
}
