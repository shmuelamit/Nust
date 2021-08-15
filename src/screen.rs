use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

pub fn create_window(width: u32, height: u32, title: &str) -> (EventLoop<()>, Window, Pixels) {
    let event_loop = EventLoop::new();

    let size = LogicalSize::new(width, height);
    let window = WindowBuilder::new()
        .with_title(title)
        .with_inner_size(size)
        .with_min_inner_size(size)
        .build(&event_loop)
        .unwrap();

    let mut pixels = Pixels::new(
        width,
        height,
        SurfaceTexture::new(
            window.inner_size().width,
            window.inner_size().height,
            &window,
        ),
    )
    .unwrap();

    (event_loop, window, pixels)
}
