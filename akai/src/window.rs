use winit::event::WindowEvent;
use winit::event::{ElementState, KeyEvent};
use winit::event_loop::{EventLoop, EventLoopWindowTarget};
use winit::keyboard::{Key, NamedKey};

const WINDOW_TITLE: &'static str = "Akai";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

pub(crate) struct Window {
    window: winit::window::Window,
    redraw_requested: bool,
}

impl Window {
    pub fn create(event_loop: &EventLoop<()>) -> Window {
        let window = winit::window::WindowBuilder::new()
            .with_title(WINDOW_TITLE)
            .with_inner_size(winit::dpi::LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
            .build(event_loop)
            .expect("Failed to create window.");

        Window {
            redraw_requested: false,
            window,
        }
    }

    pub fn window_handle(&self) -> &winit::window::Window {
        &self.window
    }

    pub fn window_event(&mut self, event: WindowEvent, elwt: &EventLoopWindowTarget<()>) {
        match event {
            WindowEvent::CloseRequested => {
                elwt.exit();
            }
            WindowEvent::RedrawRequested => {
                self.redraw_requested = true;
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key: key,
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => match key.as_ref() {
                Key::Named(NamedKey::Escape) => {
                    elwt.exit();
                }
                _ => {}
            },
            _ => {}
        }
    }
}
