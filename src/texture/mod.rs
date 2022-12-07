use std::fmt::Debug;

use crate::{Point3, Color};

/// A texture usually means a function that makes the colors on a surface procedural.
/// This procedure can be synthesis code, or it could be an image lookup, or a
/// combination of both.
pub trait Texture: Sync + Send + Debug {
    /// The color of the texture at a given point.
    /// 
    /// # Arguments
    /// 
    /// * `point` - The point on the surface of the object.
    /// * `u`, `v` - The texture coordinates of the point.
    fn color(&self, point: Point3, u: f64, v: f64) -> Color;
}

/// A solid color texture.
#[derive(Debug, Clone)]
pub struct SolidColor {
    color: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> Self {
        Self { color }
    }

    pub fn new_rgb(r: f64, g: f64, b: f64) -> Self {
        Self::new(Color::new(r, g, b))
    }

    pub fn color(&self) -> Color {
        self.color
    }
}

impl Texture for SolidColor {
    fn color(&self, _point: Point3, _u: f64, _v: f64) -> Color {
        self.color
    }
}
