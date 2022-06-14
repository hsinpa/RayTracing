use cgmath::{Array, InnerSpace, Vector3, Vector4, Zero};
use crate::material::material_interface::{MaterialTrait, ScatterStruct};
use crate::transform::hittable::HitRecord;
use crate::utility::ray::Ray;
use crate::utility::utility_func::UtilityFunc;

pub struct DielectricMat {
    pub index_of_fraction: f32
}

impl MaterialTrait for DielectricMat {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> ScatterStruct {

        let refraction_ratio = if rec.front_face { 1.0 / self.index_of_fraction } else { self.index_of_fraction };

        let unit_direction = r_in.get_direction();
        //let refracted = UtilityFunc::refract(&unit_direction, &rec.normal, refraction_ratio);

        let cos_theta = Vector3::dot(-(unit_direction.clone()), rec.normal.clone()).min(1.0);
        let sin_theta = (1.0 - (cos_theta * cos_theta)).sqrt();

        let connot_refract = refraction_ratio * sin_theta > 1.0;
        let mut direction = Vector3::zero();

        if connot_refract || UtilityFunc::reflectance(cos_theta, refraction_ratio) > UtilityFunc::get_random_float() {
            direction = UtilityFunc::reflect(&unit_direction, &rec.normal);
        } else {
            direction = UtilityFunc::refract(&unit_direction, &rec.normal, refraction_ratio);
        }

        ScatterStruct  {
            attenuation: Vector4::new(1.0, 1.0, 1.0, 1.0),
            scattered: Ray::new(rec.p.clone(), direction),
            hit: true
        }
    }
}