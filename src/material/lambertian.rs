use cgmath::{InnerSpace, Vector4};
use crate::material::material_interface::{MaterialTrait, ScatterStruct};
use crate::transform::hittable::HitRecord;
use crate::utility::ray::Ray;
use crate::utility::utility_func::UtilityFunc;
#[derive(Debug, Copy, Clone)]
pub struct LambertianMat {
    pub albedo: Vector4<f32>,
    pub id: u32,
}

impl LambertianMat {
    pub fn new(id:u32, color: Vector4<f32>) -> Self {
        Self {
            id: id,
            albedo: color
        }
    }
}

impl MaterialTrait for LambertianMat {
    fn get_id(&self) -> u32 {
        self.id
    }

    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> ScatterStruct {
        let mut scatter_direction = rec.normal + UtilityFunc::random_in_unit_sphere().normalize();

        if UtilityFunc::vector_near_zero(&scatter_direction) {
            scatter_direction = rec.normal.clone();
        }

        ScatterStruct {
            scattered: Ray::new(rec.p, scatter_direction),
            attenuation: self.albedo.clone(),
            result: true,
        }
    }
}
