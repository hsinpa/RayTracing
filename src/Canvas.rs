use cgmath::Vector4;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::window::Window;

pub struct PixelCanvas {
    pixels: Pixels,
    width: u32,
    height: u32,
}

impl PixelCanvas {
    pub fn new(window : &Window) -> Self{
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, window);

        Self {
            width : window_size.width,
            height : window_size.height,
            pixels: Pixels::new(window_size.width, window_size.height, surface_texture).unwrap()
        }
    }

    pub fn set_color(&mut self, x: u32, y: u32, color: Vector4<u8>) {
        let frame = self.pixels.get_frame();
        let frame_chunks = frame.chunks_exact_mut(4);
        let index = ((y * self.width) + x) as u8;
        frame_chunks[index][0] = color.x; // R
        frame_chunks[index][1] = color.y; // G
        frame_chunks[index][2] = color.z; // B
        frame_chunks[index][3] = color.w; // A

        self.pixels.render().unwrap();
    }

    pub fn render(&mut self) {
        let frame = self.pixels.get_frame();
        for pixel in frame.chunks_exact_mut(4) {
            pixel[0] = 0; // R
            pixel[1] = 0; // G
            pixel[2] = 255; // B
            pixel[3] = 255; // A
        }
        self.pixels.render().unwrap();
    }

}