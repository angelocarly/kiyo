use std::sync::Arc;
use ash::vk::PhysicalDevice;
use winit::event::Event;
use winit::event_loop::EventLoop;

use crate::{vulkan, window};
use crate::vulkan::{Device, Instance, Surface};
use crate::window::Window;

pub struct Application {
    pub event_loop: EventLoop<()>,
    pub window: Window,
    pub entry: Arc<ash::Entry>,
    pub surface: Surface,
    pub device: Arc<Device>,
    pub physical_device: PhysicalDevice,
    pub instance: Arc<Instance>,
}

impl Application {
    pub fn new() -> Application {

        let event_loop = EventLoop::new().expect("Failed to create event loop.");
        let window = window::Window::create(&event_loop);

        let entry = Arc::new(ash::Entry::linked());
        let instance = Arc::new( vulkan::Instance::new(entry.clone(), window.display_handle()) );
        let surface = vulkan::Surface::new(instance.clone(), &window);
        let (physical_device, queue_family_index) = instance.create_physical_device(&surface);
        let device = Arc::new(Device::new(instance.clone(), physical_device, queue_family_index));

        Self {
            event_loop,
            window,
            entry,
            instance,
            surface,
            physical_device,
            device
        }
    }

    pub fn run(mut self) {
        self.event_loop
            .run(|event, elwt| if let Event::WindowEvent { event, .. } = event {
                self.window.window_event(event, elwt);
            })
            .unwrap()
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::new()
    }
}