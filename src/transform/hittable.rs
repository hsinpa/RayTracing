use std::cell::{Ref, RefCell};
use std::rc::Rc;
use cgmath::{dot, InnerSpace, Vector3, Zero};
use crate::material::material_interface::MaterialTrait;
use crate::utility::ray::Ray;

pub struct HitRecord {
    pub p: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub t: f32,
    pub front_face: bool,
    pub mat_ptr: Option<Rc<RefCell<dyn MaterialTrait>>>,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            p: Vector3::zero(),
            normal: Vector3::zero(),
            t: 0.0,
            front_face: false,
            mat_ptr : None,
        }
    }

    pub fn set_mat_prt(&mut self, material: Rc<RefCell<dyn MaterialTrait>>) {
        self.mat_ptr = Some(material);
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vector3<f32>) {
        self.front_face = dot(r.get_direction(), outward_normal.normalize()) < 0.0;
        self.normal = if self.front_face {outward_normal} else {-outward_normal};
    }
}

impl Clone for HitRecord {
    fn clone(&self) -> Self {

        Self {
            p: self.p.clone(),
            normal: self.normal.clone(),
            t : self.t.clone(),
            front_face: self.front_face.clone(),
            mat_ptr: self.mat_ptr.clone(),
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}