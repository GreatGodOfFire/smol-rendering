use render::Renderer;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

pub mod render;

pub struct App {
    renderer: Renderer,
    event_loop: EventLoop<()>,
    window: Window,
}

impl App {
    pub fn new(title: &str) -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().with_title(title).build(&event_loop).unwrap();
        let renderer = pollster::block_on(Renderer::new(&window));

        Self {
            renderer,
            event_loop,
            window,
        }
    }

    pub fn run(mut self) {
        self.event_loop
            .run(move |event, _, control_flow| match event {
                Event::RedrawRequested(window_id) if window_id == self.window.id() => {
                    match self.renderer.render() {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost) => self.renderer.resize(self.renderer.size),
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
                Event::MainEventsCleared => {
                    self.window.request_redraw();
                }
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.window.id() => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        self.renderer.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        self.renderer.resize(**new_inner_size);
                    }
                    _ => {}
                },
                _ => {}
            });
    }

    pub fn renderer(&self) -> &Renderer {
        &self.renderer
    }

    pub fn renderer_mut(&mut self) -> &mut Renderer {
        &mut self.renderer
    }
}
