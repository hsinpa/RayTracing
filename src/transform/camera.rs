use cgmath::Vector3;
use crate::utility::ray::Ray;

pub struct Camera {
    pub origin: Vector3<f32>,
    pub lower_left_corner: Vector3<f32>,
    pub horizontal: Vector3<f32>,
    pub vertical: Vector3<f32>,
    pub sampler: i32,
}

impl Camera {

    pub fn new(position: Vector3<f32>) -> Self {
        let aspect_ratio:f32 = 16.0 / 9.0;

        let viewport_height:f32 = 2.0;
        let viewport_width:f32 = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let horizontal: Vector3<f32> = Vector3::new(viewport_width, 0.0, 0.0);
        let vertical: Vector3<f32> = Vector3::new(0.0, viewport_height, 0.0);
        let lower_left_corner = position - (horizontal / 2.0) - (vertical / 2.0) - Vector3::new(0.0, 0.0, focal_length);
        Self {
            origin: position, lower_left_corner, horizontal, vertical,
            sampler: 1,
        }
    }

    pub fn set_sampler(&mut self, sampler: i32) {
        self.sampler = sampler;
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let direction = self.lower_left_corner + (u * self.horizontal) + (v * self.vertical) - self.origin;
        return Ray::new(self.origin.clone(), direction);
    }
}
