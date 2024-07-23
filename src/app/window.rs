use ash::vk::Extent2D;
use winit::event::WindowEvent;
use winit::event::{ElementState, KeyEvent};
use winit::event_loop::{EventLoop, EventLoopWindowTarget};
use winit::keyboard::{Key, NamedKey};
use winit::raw_window_handle::{HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle};

/// System window wrapper.
/// Handles window events i.e. close, redraw, keyboard input.
pub struct Window {
    window: winit::window::Window,
}


impl Window {
    pub fn create(event_loop: &EventLoop<()>, window_title: &str, width: u32, height: u32) -> Window {
        let window = winit::window::WindowBuilder::new()
            .with_title(window_title)
            .with_inner_size(winit::dpi::LogicalSize::new(width, height))
            .build(event_loop)
            .expect("Failed to create window.");

        Window {
            window,
        }
    }

    pub fn window_handle(&self) -> RawWindowHandle {
        self.window.window_handle().unwrap().as_raw()
    }

    pub fn display_handle(&self) -> RawDisplayHandle {
        self.window.display_handle().unwrap().as_raw()
    }

    pub fn get_extent(&self) -> Extent2D {
        let width = self.window.inner_size().width;
        let height = self.window.inner_size().height;
        Extent2D{ width, height }
    }

    pub fn window_event(&mut self, event: WindowEvent, elwt: &EventLoopWindowTarget<()>) {
        match event {
            WindowEvent::CloseRequested => {
                elwt.exit();
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
                },
                Key::Character("q") => {
                    elwt.exit();
                }
                _ => {}
            },
            _ => {}
        }
    }
}
