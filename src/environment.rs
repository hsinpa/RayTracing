use std::cell::RefCell;
use std::f32::consts::PI;
use std::ops::Mul;
use std::rc::Rc;
use cgmath::{ElementWise, InnerSpace, Vector3, Vector4, Zero};
use cgmath::num_traits::Float;
use pixels::wgpu::Color;
use crate::material::dielectric::DielectricMat;
use crate::material::lambertian::LambertianMat;
use crate::material::materials::Materials;
use crate::material::metal::MetalMat;
use crate::PixelCanvas;
use crate::transform::hittable::{HitRecord, Hittable};
use crate::transform::hittable_list::HittableList;
use crate::transform::sphere::Sphere;
use crate::utility::ray::Ray;
use crate::transform::camera::Camera;
use crate::utility::utility_func::UtilityFunc;
use crate::utility::static_flags::{LAMBERTIAN_MAT_ID};

pub struct Scene {
    pixel_canvas : PixelCanvas,
    camera: Camera,
    world : HittableList,
    image_width: u32,
    image_height: u32,
}

impl Scene {
    pub fn new(canvas: PixelCanvas) -> Self {
        //Image
        let aspect_ratio:f32 = 16.0 / 9.0;

        let image_width = canvas.get_size().x;
        let image_height = (image_width as f32 / aspect_ratio) as u32;
        let material_ground= Rc::new(RefCell::new(LambertianMat::new(Vector4::new(0.8, 0.8, 0.0, 1.0))));
        let material_center= Rc::new(RefCell::new(LambertianMat::new(Vector4::new(0.1, 0.2, 0.5, 1.0))));
        let material_left= Rc::new(RefCell::new(DielectricMat{ index_of_fraction: 1.5}));
        let material_right= Rc::new(RefCell::new(MetalMat::new(Vector4::new(0.8, 0.6, 0.2, 1.0), 0.0)));

        //World
        let mut world: HittableList = HittableList::new();
        world.add(Box::new( Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0, material_ground) ));
        world.add(Box::new( Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5, material_center )));
        world.add(Box::new( Sphere::new(Vector3::new(-1.0, 0.0, -1.0), 0.5, material_left.clone() )));
        world.add(Box::new( Sphere::new(Vector3::new(-1.0, 0.0, -1.0), -0.45, material_left)));
        world.add(Box::new( Sphere::new(Vector3::new(1.0, 0.0, -1.0), 0.5, material_right) ));

        //Camera
        let origin:Vector3<f32> = Vector3::new(-2.0,2.0,1.0);
        let lookat:Vector3<f32> = Vector3::new(0.0,0.0, -1.0);
        let vup:Vector3<f32> = Vector3::new(0.0,1.0, 0.0);
        let dist_to_focus = (origin - lookat).magnitude();
        let aperture = 2.0;

        let mut camera = Camera::new(origin,lookat, vup, PI * 0.1,  aspect_ratio, aperture, dist_to_focus);
                camera.set_sampler(20);

        Self {
            pixel_canvas: canvas,
            image_width: image_width,
            image_height: image_height,
            camera: camera,
            world: world,
        }
    }

    pub fn process(&mut self) {
        println!("Image width {}, image_height {}", self.image_width, self.image_height);
        for j in (0..self.image_height).rev() {
            println!("Scanline remaining {}", j);

            for i in 0..self.image_width {

                let mut pixel_color: Vector4<f32> = Vector4::zero();
                for sample_index in 0..self.camera.sampler {
                    let u = (i as f32 + UtilityFunc::get_random_float())/ (self.image_width - 1) as f32;
                    let v = (j as f32  + UtilityFunc::get_random_float())/ (self.image_height - 1) as f32;
                    let ray = self.camera.get_ray(u,v);

                    let hit_color = Scene::ray_color(&ray, &self.world, 10);
                    pixel_color += hit_color;
                }

                // let x = (u * (self.image_width as f32)) as u32;
                // let y = (v * (self.image_height as f32)) as u32;
                let x = i;
                let y =  j;

                self.pixel_canvas.set_color( x, y, Scene::convert_8bit_color(&pixel_color, self.camera.sampler) );
            }
        }

        self.pixel_canvas.render();
    }

    pub fn ray_color(ray: &Ray, world: &dyn Hittable, depth: i32) -> Vector4<f32> {

        let mut rec = HitRecord::new();

        if depth <= 0 {
            return Vector4::zero();
        }

        if world.hit(ray, 0.0001, f32::MAX, &mut rec) {

            let hit_mat_option = rec.mat_ptr.clone();
            if hit_mat_option.is_some() {
                let mut hit_mat = hit_mat_option.unwrap();
                let scatter_result = hit_mat.borrow_mut().scatter(&ray, &rec);

                if scatter_result.hit {
                    let color_return = Scene::ray_color(&scatter_result.scattered, world, depth - 1);

                    return scatter_result.attenuation.mul_element_wise(color_return);
                }
            }
            return Vector4::zero();


            // let target = rec.p + UtilityFunc::random_in_hemisphere(&rec.normal);
            // let reflect_ray = Ray::new(rec.p, target - rec.p);
            // let hit_color = Scene::ray_color(&reflect_ray, world, depth - 1);
            // return 0.5 * hit_color;
        }

        let unit_direction = ray.get_direction().normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        let mut lerp_color =  (1.0-t)* Vector4::new(1.0, 1.0, 1.0, 1.0) + t * Vector4::new(0.5, 0.7, 1.0, 1.0);
            lerp_color.w = 1.0;
        return lerp_color;
    }

    pub fn convert_8bit_color(color: &Vector4<f32>, samples_per_pixel: i32) -> Vector4<u8> {
        let scale = 1.0 / samples_per_pixel as f32;

        let target_color: Vector4<f32> = color * scale;
        let mut r = target_color.x.sqrt().clamp(0.0, 0.999) * 255.0;
        let mut g = target_color.y.sqrt().clamp(0.0, 0.999) * 255.0;
        let mut b = target_color.z.sqrt().clamp(0.0, 0.999) * 255.0;
        let mut a = 255.0;

        return Vector4::new(r as u8, g as u8, b as u8 ,a as u8);
    }

    pub fn vec3_to_vec4(c: &Vector3<f32>) -> Vector4<f32> {
        return Vector4::new(c.x, c.y, c.z ,1.0);
    }
}
