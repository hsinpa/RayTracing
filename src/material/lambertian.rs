use cgmath::{InnerSpace, Vector4};
use crate::material::material_interface::{MaterialTrait, ScatterStruct};
use crate::transform::hittable::HitRecord;
use crate::utility::ray::Ray;
use crate::utility::utility_func::UtilityFunc;
#[derive(Debug, Copy, Clone)]
pub struct LambertianMat {
    pub albedo: Vector4<f32>,
}

impl LambertianMat {
    pub fn new(color: Vector4<f32>) -> Self {
        Self {
            albedo: color
        }
    }
}

impl MaterialTrait for LambertianMat {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> ScatterStruct {
        let mut scatter_direction = rec.normal + UtilityFunc::random_in_unit_sphere();

        if UtilityFunc::vector_near_zero(&scatter_direction) {
            scatter_direction = rec.normal.clone();
        }

        ScatterStruct {
            scattered: Ray::new(rec.p, scatter_direction),
            attenuation: self.albedo.clone(),
            hit: true,
        }
    }
}
