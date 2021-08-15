mod screen;
mod cpu;
mod nes_parser;
mod bus;
mod ppu;

use crate::screen::create_window;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use rand::Rng;

fn main() {
    let (event_loop, window, mut pixels) = create_window(256, 240, "lol");
    let mut scale = window.scale_factor();
    let mut input = WinitInputHelper::new();

    let mut r = rand::thread_rng();

    for pixel in pixels.get_frame().chunks_exact_mut(4) {
        pixel.copy_from_slice(&[r.gen(), r.gen(), r.gen(), 0xff])
    }

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        match event {
            Event::RedrawRequested(_) => {
                if pixels.render().is_err() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            },
            _ => ()
        }

        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            window.request_redraw();
        }
    });
}
