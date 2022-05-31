use cgmath::{InnerSpace, Vector3, Vector4};
use cgmath::num_traits::Float;
use crate::PixelCanvas;
use crate::transform::hittable::{HitRecord, Hittable};
use crate::transform::hittable_list::HittableList;
use crate::transform::sphere::Sphere;
use crate::utility::ray::Ray;

pub struct Scene {
    pixel_canvas : PixelCanvas,
    viewport_height: f32,
    viewport_width: f32,

    image_width: u32,
    image_height: u32,

    focal_length: f32,
    aspect_ratio: f32,
    camera: Vector3<f32>,
    world : HittableList,
}

impl Scene {

    pub fn new(canvas: PixelCanvas) -> Self {
        //Image
        let aspect_ratio:f32 = 16.0 / 9.0;

        let image_width = canvas.get_size().x;
        let image_height = (image_width as f32 / aspect_ratio) as u32;


        //World
        let mut world: HittableList = HittableList::new();
        world.add(Box::new( Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5) ));
        world.add(Box::new( Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0) ));

        //Camera
        let viewport_height:f32 = 2.0;
        let viewport_width:f32 = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin:Vector3<f32> = Vector3::new(0.0,0.0,0.0);

        Self {
            pixel_canvas: canvas,
            viewport_height: viewport_height,
            viewport_width : viewport_width,
            image_width: image_width,
            image_height: image_height,
            focal_length: focal_length,
            aspect_ratio: aspect_ratio,
            camera: origin,
            world: world
        }
    }

    pub fn process(&mut self) {
        println!("Image width {}, image_height {}", self.image_width, self.image_height);

        let horizontal: Vector3<f32> = Vector3::new(self.viewport_width, 0.0, 0.0);
        let vertical: Vector3<f32> = Vector3::new(0.0, self.viewport_height, 0.0);
        let lower_left_corner = self.camera - (horizontal / 2.0) - (vertical / 2.0) - Vector3::new(0.0, 0.0, self.focal_length);




        for j in (0..self.image_height-1).rev() {
            for i in 0..self.image_width-1 {
                let u = i as f32 / (self.image_width - 1) as f32;
                let v = j as f32 / (self.image_height - 1) as f32;

                let direction = lower_left_corner + (u * horizontal) + (v * vertical) - self.camera;
                let ray = Ray::new(self.camera.clone(), direction);
                let hit_color = Scene::ray_color(&ray, &self.world);

                let x = (u * (self.image_width as f32)) as u32;
                let y = (v * (self.image_height as f32)) as u32;

                self.pixel_canvas.set_color( x, y, Scene::convert_8bit_color(&hit_color) );
            }
        }

        self.pixel_canvas.render();
    }

    pub fn hit_sphere(center : Vector3<f32>, radius : f32, r : &Ray) -> f32 {
        let oc = r.get_origin() - center;
        let a = r.get_direction().magnitude2();
        let half_b = Vector3::dot(oc, r.get_direction());
        let c = oc.magnitude2() - (radius * radius);
        let discriminant = (half_b * half_b) - (a * c);

        if discriminant < 0.0 {
            return -1.0;
        } else {
         return (-half_b - discriminant.sqrt()) / a;
        }
    }

    pub fn ray_color(ray: &Ray, world: &dyn Hittable) -> Vector4<f32> {

        let mut rec = HitRecord::new();

        if world.hit(ray, 0.0, f32::MAX, &mut rec) {
            return Scene::vec3_to_vec4(&(0.5 * (rec.normal + Vector3::new(1.0, 1.0, 1.0)) ));
        }

        let unit_direction = ray.get_direction().normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        let mut lerp_color =  (1.0-t)* Vector4::new(1.0, 1.0, 1.0, 1.0) + t * Vector4::new(0.5, 0.7, 1.0, 1.0);
            lerp_color.w = 1.0;
        return lerp_color;
    }

    pub fn convert_8bit_color(color: &Vector4<f32>) -> Vector4<u8> {
        let target_color = color * 255.0;
        return Vector4::new(target_color.x as u8, target_color.y as u8, target_color.z as u8 ,target_color.w as u8);
    }

    pub fn vec3_to_vec4(c: &Vector3<f32>) -> Vector4<f32> {
        return Vector4::new(c.x, c.y, c.z ,1.0);
    }
}
