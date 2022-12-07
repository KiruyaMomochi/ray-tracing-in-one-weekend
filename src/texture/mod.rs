use std::fmt::Debug;

use crate::{Color, Point3};

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

/// A checker texture that forms a 3D checker pattern
#[derive(Debug, Clone)]
pub struct Checker<O, E>
where
    O: Texture,
    E: Texture,
{
    odd: O,
    even: E,
}

impl<O, E> Checker<O, E>
where
    O: Texture,
    E: Texture,
{
    pub fn new(odd: O, even: E) -> Self {
        Self { odd, even }
    }
}

impl Checker<SolidColor, SolidColor> {
    pub fn new_solids(odd: Color, even: Color) -> Self {
        Self {
            odd: SolidColor::new(odd),
            even: SolidColor::new(even),
        }
    }
}

fn sines(point: &Point3) -> f64 {
    let sines = (10.0 * point).apply(|x| x.sin());
    sines.x() * sines.y() * sines.z()
}

impl<O, E> Texture for Checker<O, E>
where
    O: Texture,
    E: Texture,
{
    fn color(&self, point: Point3, u: f64, v: f64) -> Color {
        if sines(&point).is_sign_negative() {
            self.odd.color(point, u, v)
        } else {
            self.even.color(point, u, v)
        }
    }
}
