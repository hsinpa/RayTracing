use cgmath::{InnerSpace, Vector3, Vector4};
use crate::material::material_interface::{MaterialTrait, ScatterStruct};
use crate::transform::hittable::HitRecord;
use crate::utility::ray::Ray;
use crate::utility::utility_func::UtilityFunc;

pub struct MetalMat {
    pub albedo: Vector4<f32>,
    pub fuzz: f32,
}

impl MetalMat {
    pub fn new(color: Vector4<f32>, fuzz: f32) -> Self {
        Self {
            albedo: color,
            fuzz
        }
    }
}

impl MaterialTrait for MetalMat {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> ScatterStruct {
        let mut scatter_direction = rec.normal + UtilityFunc::random_in_unit_sphere();

        let reflected = UtilityFunc::reflect( &r_in.get_direction(), &rec.normal);

        ScatterStruct {
            scattered: Ray::new(rec.p, reflected + (self.fuzz * UtilityFunc::random_in_unit_sphere())),
            attenuation: self.albedo.clone(),
            hit: Vector3::dot(scatter_direction, rec.normal) > 0.0,
        }
    }
}
