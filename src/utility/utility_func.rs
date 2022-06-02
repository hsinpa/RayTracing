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

    pub fn degrees_to_radians(degrees: f32) -> f32 {
        return degrees * 3.1415926 / 180.0;
    }
}