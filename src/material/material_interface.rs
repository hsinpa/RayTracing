use cgmath::Vector4;
use crate::transform::hittable::HitRecord;
use crate::utility::ray::Ray;

pub trait  MaterialTrait {
    fn get_id(&self) -> u32;
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> ScatterStruct;
}

pub struct  ScatterStruct {
    pub attenuation: Vector4<f32>,
    pub scattered: Ray,
    pub result: bool,
}