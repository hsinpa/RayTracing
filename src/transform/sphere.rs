use std::cell::{RefCell};
use std::rc::Rc;
use cgmath::{InnerSpace, Vector3};
use crate::material::material_interface::MaterialTrait;
use crate::Scene;
use crate::transform::hittable::{HitRecord, Hittable};
use crate::utility::ray::Ray;
use crate::utility::utility_func::UtilityFunc;

pub struct Sphere{
    pub center: Vector3<f32>,
    pub radius: f32,
    pub mat_ptr: Rc<RefCell<dyn MaterialTrait>>
}

impl Sphere{
    pub fn new(center: Vector3<f32>, radius: f32, material: Rc<RefCell<dyn MaterialTrait>>) -> Self {
        Self {
            center, radius,
            mat_ptr: material
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = r.get_origin() - self.center;
        let a = UtilityFunc::length_squared(&r.get_direction());
        let half_b = Vector3::dot(oc, r.get_direction());
        let c = UtilityFunc::length_squared(&oc) - (self.radius * self.radius);

        let discriminant = (half_b * half_b) - (a * c);
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        rec.normal = (rec.p - self.center) / self.radius;

        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.set_mat_prt(self.mat_ptr.clone());

        return true;
    }
}