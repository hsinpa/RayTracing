use cgmath::{Vector4, Zero};
use pixels::{Error, Pixels, SurfaceTexture};
use winit::window::Window;
use rayon::prelude::*;

pub struct PixelCanvas {
    canvas: Pixels,
    width: u32,
    height: u32,
    pixels: Vec<Vector4<u8>>,
}

impl PixelCanvas {
    pub fn new(window : &Window) -> Self{
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, window);
        println!("Window size {}", window_size.width * window_size.height);

        Self {
            width : window_size.width,
            height : window_size.height,
            canvas: Pixels::new(window_size.width, window_size.height, surface_texture).unwrap(),
            pixels: vec![Vector4::zero(); (window_size.width * window_size.height) as usize],
        }
    }

    pub fn set_color(&mut self, x: u32, y: u32, color: Vector4<u8>) {
        let index = ((y * self.width) + x) as usize;
        self.pixels[index] = color;
    }

    pub fn render(&mut self) {
        let frame = self.canvas.get_frame();
        let chunks = frame.chunks_exact_mut(4);

        for (index, pixel) in chunks.enumerate() {
            pixel[0] = self.pixels[index].x; // R
            pixel[1] = self.pixels[index].y; // G
            pixel[2] = self.pixels[index].z; // B
            pixel[3] = self.pixels[index].w; // A
        }
        self.canvas.render().unwrap();
    }

    pub fn clear(&mut self) {
        self.pixels.par_iter_mut().for_each(|color| color.set_zero());
    }
}