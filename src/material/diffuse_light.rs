use std::sync::Arc;

use crate::{
    texture::{SolidColor, Texture},
    Color, Material, Point3,
};

/// A material which emits light with color from a texture.
#[derive(Debug, Clone)]
pub struct DiffuseLight {
    texture: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(texture: Arc<dyn Texture>) -> Self {
        Self { texture }
    }
    pub fn solid(color: Color) -> Self {
        Self::new(Arc::new(SolidColor::new(color)))
    }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        _ray: &crate::Ray,
        _hit_record: &crate::HitRecord,
    ) -> Option<(crate::Ray, crate::Color)> {
        None
    }

    fn emit(&self, point: Point3, u: f64, v: f64) -> crate::Color {
        self.texture.color(point, u, v)
    }
}
