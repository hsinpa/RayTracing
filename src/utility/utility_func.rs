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
            return p;

        }
    }

    pub fn length_squared(vec: &Vector3<f32>) -> f32 {
        (vec.x * vec.x) + (vec.y * vec.y) + (vec.z * vec.z)
    }

    pub fn degrees_to_radians(degrees: f32) -> f32 {
        return degrees * std::f32::consts::PI / 180.0;
    }
}