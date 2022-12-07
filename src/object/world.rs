use crate::{Hit, HitRecord, hit::AABB};

// Vec<Box<dyn trait>> has an implict 'static lifetime
// https://stackoverflow.com/questions/70717050/why-do-i-need-static-lifetime-here-and-how-to-fix-it
// https://users.rust-lang.org/t/box-with-a-trait-object-requires-static-lifetime/35261/2
pub struct World(Vec<Box<dyn Hit>>);

impl World {
    pub fn new() -> Self {
        Self(Vec::new())
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
    fn hit(&self, ray: &crate::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by
        self.0
            .iter()
            .filter_map(|object| object.hit(ray, t_min, t_max))
            .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap()) // unwarp: t != NaN
    }

    fn bounding_box(&self, time_from: f64, time_to: f64) -> Option<AABB> {
        self.0.iter()
            .filter_map(|object| object.bounding_box(time_from, time_to))
            .reduce(|a, b| a.merge(b))
    }
}
