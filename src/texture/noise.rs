use crate::{Color, Vec3};

use super::{perlin::Perlin, Texture};

/// A noise texture which uses Perlin noise to generate a color.
#[derive(Debug, Clone)]
pub struct Noise {
    perlin: Perlin,
}

impl Noise {
    pub fn new() -> Self {
        Self {
            perlin: Perlin::new(),
        }
    }
}

impl Default for Noise {
    fn default() -> Self {
        Self::new()
    }
}

impl Texture for Noise {
    fn color(&self, point: crate::Point3, _u: f64, _v: f64) -> Color {
        Vec3::constant(self.perlin.noise(&point))
    }
}
