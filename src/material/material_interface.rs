use cgmath::Vector4;
use crate::transform::hittable::HitRecord;
use crate::utility::ray::Ray;

pub trait  MaterialTrait {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> ScatterStruct;
}

pub struct ScatterStruct {
    pub attenuation: Vector4<f32>,
    pub scattered: Ray,
    pub hit: bool,
}