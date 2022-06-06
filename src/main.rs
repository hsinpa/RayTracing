mod canvas;
mod utility;
mod environment;
mod transform;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use pixels::{Error, Pixels, SurfaceTexture};
use canvas::PixelCanvas;
use environment::Scene;
const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;
const BOX_SIZE: i16 = 64;



fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut canvas = PixelCanvas::new(&window);
    let mut scene = Scene::new(canvas);
    scene.process();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,

            Event::RedrawRequested(_) => {

            },
            _ => (),


        }
    });
}
