use cgmath::Vector3;

pub struct Ray {
    origin : Vector3<f32>,
    direction: Vector3<f32>
}

impl Ray {
    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>) -> Self {
        Self {
            origin: origin,
            direction: direction
        }
    }

    pub fn get_origin(&self) -> Vector3<f32> {
        self.origin.clone()
    }

    pub fn get_direction(&self) -> Vector3<f32> {
        self.direction.clone()
    }

    pub fn at(&self, t: f32) -> Vector3<f32> {
        self.origin + (t * self.direction)
    }
}