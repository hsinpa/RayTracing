use cgmath::{InnerSpace, Vector3};
use crate::utility::ray::Ray;
use crate::utility::utility_func::UtilityFunc;

pub struct Camera {
    pub origin: Vector3<f32>,
    pub lower_left_corner: Vector3<f32>,
    pub horizontal: Vector3<f32>,
    pub vertical: Vector3<f32>,
    pub sampler: i32,
    lens_radius: f32,
    u: Vector3<f32>,
    v: Vector3<f32>,
    w: Vector3<f32>,
}

impl Camera {

    pub fn new(lookfrom: Vector3<f32>, lookat: Vector3<f32>, vup: Vector3<f32>, vfov : f32, aspect_ratio: f32, aperture: f32, focus_dist: f32) -> Self {
        let h = (vfov / 2.0).tan();
        let viewport_height:f32 = 2.0 * h;
        let viewport_width:f32 = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).normalize();
        let u = Vector3::cross(vup, w).normalize();
        let v = Vector3::cross(w, u);

        let horizontal: Vector3<f32> = focus_dist * viewport_width * u;
        let vertical: Vector3<f32> = focus_dist * viewport_height * v;
        let lower_left_corner = lookfrom - (horizontal / 2.0) - (vertical / 2.0) - (focus_dist * w);
        Self {
            origin: lookfrom, lower_left_corner, horizontal, vertical,
            lens_radius: aperture / 2.0,
            sampler: 1,
            u,v,w
        }
    }

    pub fn set_sampler(&mut self, sampler: i32) {
        self.sampler = sampler;
    }

    pub fn get_ray(&self, x: f32, y: f32) -> Ray {
        let rd = self.lens_radius * UtilityFunc::random_in_unit_disk();
        let offset = (self.u * rd.x) + (self.v * rd.y);

        let direction = self.lower_left_corner + (x * self.horizontal) + (y * self.vertical) - self.origin - offset;
        return Ray::new(self.origin + offset, direction);
    }
}
