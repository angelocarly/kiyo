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

        let entry = ash::Entry::linked();
        let instance = vulkan::Instance::new(&entry, window.window_handle());
        let (physical_device, queue_family_index) = instance.create_physical_device(window.window_handle());
        println!("Queue family index: {}", queue_family_index);

        println!("Destroyed instance?");


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
