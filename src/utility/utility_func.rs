use cgmath::{InnerSpace, Vector3};
use rand::Rng;
use rand::rngs::ThreadRng;

pub struct UtilityFunc {
    rand_rng: ThreadRng
}

impl UtilityFunc {

    pub fn new() -> Self{
        Self {
            rand_rng: rand::thread_rng()
        }
    }

    pub fn get_random_float(&mut self) -> f32 {
        self.rand_rng.gen()
    }

    pub fn get_random_range(&mut self, min: f32, max: f32) -> f32 {
        min + (max - min) * self.get_random_float()
    }

    pub fn random_vector3(&mut self) -> Vector3<f32> {
        return Vector3::new(self.get_random_float(),self.get_random_float(), self.get_random_float());
    }

    pub fn random_vector3_rangge(&mut self, min: f32, max: f32) -> Vector3<f32> {
        return Vector3::new(self.get_random_range(min, max),self.get_random_range(min, max), self.get_random_range(min, max));
    }

    pub fn random_in_unit_sphere(&mut self) -> Vector3<f32> {
        loop {
            let p = self.random_vector3_rangge(-1.0, 1.0);

            if (p.magnitude() >= 1.0) {
                continue;
            }
            return p;

        }
    }

    pub fn degrees_to_radians(degrees: f32) -> f32 {
        return degrees * 3.1415926 / 180.0;
    }
}