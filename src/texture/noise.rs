use crate::Color;

use super::{perlin::Perlin, Texture};

/// A noise texture which uses Perlin noise to generate a color.
#[derive(Debug, Clone)]
pub struct Noise {
    perlin: Perlin,
    /// The scale of the noise
    scale: f64,
}

impl Noise {
    pub fn new(scale: f64) -> Self {
        Self {
            perlin: Perlin::new(),
            scale,
        }
    }
}

impl Texture for Noise {
    fn color(&self, point: crate::Point3, _u: f64, _v: f64) -> Color {
        // marble-like
        let phase = 10.0 * self.perlin.turbulence(&point, 7);
        let value = (self.scale * point.z() + phase).sin();
        let color = 0.5 * (1.0 + value);
        Color::constant(color)
    }
}
