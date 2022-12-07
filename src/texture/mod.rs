use crate::{Point3, Color};

/// A texture usually means a function that makes the colors on a surface procedural.
/// This procedure can be synthesis code, or it could be an image lookup, or a
/// combination of both.
pub trait Texture {
    /// The color of the texture at a given point.
    /// 
    /// # Arguments
    /// 
    /// * `point` - The point on the surface of the object.
    /// * `u`, `v` - The texture coordinates of the point.
    fn value(&self, point: Point3, u: f64, v: f64) -> Color;
}

pub struct SolidColor {
    color: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Texture for SolidColor {
    fn value(&self, _point: Point3, _u: f64, _v: f64) -> Color {
        self.color
    }
}
