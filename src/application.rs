use winit::event::Event;
use winit::event_loop::EventLoop;
use crate::{vulkan, window};

pub(crate) struct Application {
}

impl Application {
    pub fn new() -> Application {
        Application {
        }
    }

    pub fn run(&mut self) {
        let event_loop = EventLoop::new().expect("Failed to create event loop.");

        let mut window = window::Window::create(&event_loop);

        let entry = ash::Entry::linked();
        let _instance = vulkan::Instance::new(&entry, window.window_handle());

        println!("Running application");
        event_loop.run(|event, _| {
            match event {
                Event::WindowEvent { event, .. } => {
                    println!("Window event: {:?}", event);
                    window.window_event(event);
                },
                _ => {}
            }
        }).unwrap()
    }
}