use std::rc::Rc;
use crate::{vulkan, window};
use winit::event::Event;
use winit::event_loop::EventLoop;

pub struct Application {}

impl Application {
    pub fn new() -> Application {
        Application {}
    }

    pub fn run(&mut self) {
        let event_loop = EventLoop::new().expect("Failed to create event loop.");

        let mut window = window::Window::create(&event_loop);

        let entry = Rc::new(ash::Entry::linked());
        let instance = Rc::new( vulkan::Instance::new(entry.clone(), window.display_handle()) );
        let surface = vulkan::Surface::new(instance.clone(), &window);
        let (physical_device, queue_family_index) = instance.create_physical_device(&surface);

        println!("Running application");
        event_loop
            .run(|event, elwt| match event {
                Event::WindowEvent { event, .. } => {
                    window.window_event(event, elwt);
                }
                _ => {}
            })
            .unwrap()
    }
}
