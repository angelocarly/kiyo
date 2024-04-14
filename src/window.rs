use winit::event::{ElementState, KeyEvent};
use winit::event::WindowEvent;
use winit::event_loop::EventLoop;
use winit::keyboard::{Key, NamedKey};

const WINDOW_TITLE: &'static str = "Lov";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

pub(crate) struct Window
{
    window: winit::window::Window,
    close_requested: bool,
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
            close_requested: false,
            redraw_requested: false,
            window,
        }
    }

    pub fn window_event(&mut self, event: winit::event::WindowEvent) {
        match event {
            | WindowEvent::CloseRequested => {
                self.close_requested = true;
            }
            | WindowEvent::RedrawRequested => {
                self.redraw_requested = true;
            }
            | WindowEvent::KeyboardInput {
                event:
                KeyEvent {
                    logical_key: key,
                    state: ElementState::Pressed,
                    ..
                },
                ..
            } => {
                match key.as_ref() {
                    Key::Named(NamedKey::Escape) => {
                        self.close_requested = true;
                    }
                    _ => {}
                }
            }
            | _ => {}
        }
    }
}
