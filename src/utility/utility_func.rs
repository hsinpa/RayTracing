use cgmath::{InnerSpace, Vector3};

pub struct UtilityFunc {
}

impl UtilityFunc {

    pub fn get_random_float() -> f32 {
        fastrand::f32()
    }

    pub fn get_random_range(min: f32, max: f32) -> f32 {
        min + (max - min) * UtilityFunc::get_random_float()
    }

    pub fn random_vector3() -> Vector3<f32> {
        return Vector3::new(UtilityFunc::get_random_float(),UtilityFunc::get_random_float(), UtilityFunc::get_random_float());
    }

    pub fn random_vector3_rangge(min: f32, max: f32) -> Vector3<f32> {
        return Vector3::new(UtilityFunc::get_random_range(min, max),UtilityFunc::get_random_range(min, max), UtilityFunc::get_random_range(min, max));
    }

    pub fn random_in_unit_sphere() -> Vector3<f32> {
        loop {
            let p = UtilityFunc::random_vector3_rangge(-1.0, 1.0);
            if UtilityFunc::length_squared(&p) >= 1.0 {
                continue;
            }
            return p.normalize();
        }
    }

    pub fn random_in_hemisphere(normal: &Vector3<f32>) -> Vector3<f32>{
        let in_unit_sphere =  UtilityFunc::random_in_unit_sphere();
        if Vector3::dot(in_unit_sphere, *normal) > 0.0 {
            return in_unit_sphere;
        }

        return in_unit_sphere * -1.0;
    }

    pub fn reflect(v: &Vector3<f32>, n : &Vector3<f32>) -> Vector3<f32> {
        return v - 2.0 * Vector3::dot(*v, *n) * n;
    }

    pub fn length_squared(vec: &Vector3<f32>) -> f32 {
        (vec.x * vec.x) + (vec.y * vec.y) + (vec.z * vec.z)
    }

    pub fn vector_near_zero(vec: &Vector3<f32>) -> bool {
        let s:f32 = 0.00001;
        return vec.x.abs() < s && vec.y.abs() < s && vec.z.abs() < s;

    }

    pub fn degrees_to_radians(degrees: f32) -> f32 {
        return degrees * std::f32::consts::PI / 180.0;
    }
}