use crate::{
    texture::{SolidColor, Texture},
    Color, Material, Point3,
};

/// A material which emits light with color from a texture.
#[derive(Debug, Clone)]
pub struct DiffuseLight<T: Texture> {
    texture: T,
}

impl<T: Texture> DiffuseLight<T> {
    pub fn new(texture: T) -> Self {
        Self { texture }
    }
}

impl DiffuseLight<SolidColor> {
    pub fn new_solid(color: Color) -> Self {
        Self::new(SolidColor::new(color))
    }
}

impl<T: Texture> Material for DiffuseLight<T> {
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
