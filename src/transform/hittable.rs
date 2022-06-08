use std::cell::{Ref, RefCell};
use std::rc::Rc;
use cgmath::{dot, InnerSpace, Vector3, Zero};
use crate::material::material_interface::MaterialTrait;
use crate::utility::ray::Ray;

#[derive(Debug, Copy, Clone)]
pub struct HitRecord {
    pub p: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub t: f32,
    pub front_face: bool,
    pub mat_ptr: u32,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            p: Vector3::zero(),
            normal: Vector3::zero(),
            t: 0.0,
            front_face: false,
            mat_ptr : 0,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vector3<f32>) {
        self.front_face = dot(r.get_direction(), outward_normal.normalize()) < 0.0;
        self.normal = if self.front_face {outward_normal} else {-outward_normal};
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}