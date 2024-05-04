use std::sync::Arc;
use ash::vk;
use ash::vk::{PhysicalDevice};
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use crate::vulkan::{Device, GraphicsPipeline, Instance, Surface, Swapchain, RenderPass, Framebuffer, CommandPool, CommandBuffer};
use crate::window::Window;

/// Generative art runtime.
/// Manages the window and graphics recording.
pub struct Application {
    pub swapchain: Arc<Swapchain>,

    pub event_loop: EventLoop<()>,
    pub window: Window,
    pub entry: Arc<ash::Entry>,
    pub surface: Arc<Surface>,
    pub device: Arc<Device>,
    pub physical_device: PhysicalDevice,
    pub instance: Arc<Instance>,
    pub framebuffers: Vec<Arc<Framebuffer>>,
    pub command_pool: Arc<CommandPool>,
    pub command_buffer: Arc<CommandBuffer>,
    pub graphics_pipeline: Arc<GraphicsPipeline>,
}

impl Application {
    pub fn new(window_title: &str, width: u32, height: u32) -> Application {
        let event_loop = EventLoop::new().expect("Failed to create event loop.");
        let window = Window::create(&event_loop, window_title, width, height);

        let entry = Arc::new(ash::Entry::linked());
        let instance = Arc::new(Instance::new(entry.clone(), window.display_handle()));
        let surface = Arc::new(Surface::new(instance.clone(), &window));
        let (physical_device, queue_family_index) = instance.create_physical_device(surface.clone());
        let device = Arc::new(Device::new(instance.clone(), physical_device, queue_family_index));

        let swapchain = Arc::new(Swapchain::new(instance.clone(), &physical_device, device.clone(), &window, surface.clone()));

        let render_pass = Arc::new(RenderPass::new(device.clone(), vk::Format::R8G8B8A8_UNORM));

        let framebuffers = swapchain.clone().get_image_views().iter().map(|image_view| {
            Arc::new(Framebuffer::new(device.clone(), swapchain.get_extent(), render_pass.clone(), vec![image_view.clone()]))
        }).collect::<Vec<Arc<Framebuffer>>>();

        let command_pool = Arc::new(CommandPool::new(device.clone(), queue_family_index));
        let command_buffer = Arc::new(CommandBuffer::new(device.clone(), command_pool.clone()));

        let graphics_pipeline = Arc::new(GraphicsPipeline::new(device.clone(), render_pass.clone()));

        Self {
            event_loop,
            window,
            entry,
            instance,
            surface,
            physical_device,
            device,
            swapchain,
            framebuffers,
            command_pool,
            command_buffer,
            graphics_pipeline,
        }
    }

    fn draw_frame() {

    }

    pub fn run(mut self) {
        self.event_loop
            .run(|event, elwt| {

                match event {
                    | Event::WindowEvent { event, .. } => {
                        self.window.window_event( event.clone(), elwt );

                        match event {
                            WindowEvent::RedrawRequested => {
                                Self::draw_frame();
                            },
                            _ => (),
                        }
                    }
                    _ => (),
                }

            })
            .unwrap()
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::new("Akai engine", 800, 600)
    }
}