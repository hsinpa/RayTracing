use cgmath::Vector3;
use crate::PixelCanvas;

pub struct Scene {
    pixel_canvas : PixelCanvas,
    viewport_height: f32,
    viewport_width: f32,
    focal_legth: f32,
    aspect_ratio: f32,
    camera: Vector3<f32>
}