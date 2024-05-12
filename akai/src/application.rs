use std::sync::Arc;
use ash::vk;
use ash::vk::{PhysicalDevice, Queue};
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::platform::run_on_demand::EventLoopExtRunOnDemand;
use crate::vulkan::{Device, GraphicsPipeline, Instance, Surface, Swapchain, RenderPass, Framebuffer, CommandPool, CommandBuffer};
use crate::window::Window;

/// Generative art runtime.
/// Manages the window and graphics recording.
pub struct Application {
    pub swapchain: Arc<Swapchain>,
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
    pub queue: Queue,
    pub render_pass: Arc<RenderPass>,
}

impl Application {


    pub fn new(event_loop: &EventLoop<()>, window_title: &str, width: u32, height: u32) -> Application {
    let window = Window::create(&event_loop, window_title, width, height);

        let entry = Arc::new(ash::Entry::linked());
        let instance = Arc::new(Instance::new(entry.clone(), window.display_handle()));
        let surface = Arc::new(Surface::new(instance.clone(), &window));
        let (physical_device, queue_family_index) = instance.create_physical_device(surface.clone());
        let device = Arc::new(Device::new(instance.clone(), physical_device, queue_family_index));
        let queue = device.get_queue(0);

        let swapchain = Arc::new(Swapchain::new(instance.clone(), &physical_device, device.clone(), &window, surface.clone()));

        let render_pass = Arc::new(RenderPass::new(device.clone(), vk::Format::R8G8B8A8_UNORM));

        let framebuffers = swapchain.clone().get_image_views().iter().map(|image_view| {
            Arc::new(Framebuffer::new(device.clone(), swapchain.get_extent(), render_pass.clone(), vec![image_view.clone()]))
        }).collect::<Vec<Arc<Framebuffer>>>();

        let command_pool = Arc::new(CommandPool::new(device.clone(), queue_family_index));
        let command_buffer = Arc::new(CommandBuffer::new(device.clone(), command_pool.clone()));

        let graphics_pipeline = Arc::new(GraphicsPipeline::new(device.clone(), render_pass.clone()));

        Application::record_command_buffer(command_buffer.clone(), render_pass.clone(), &framebuffers.clone());

        Self {
            window,
            entry,
            instance,
            surface,
            physical_device,
            queue,
            device,
            swapchain,
            render_pass,
            framebuffers,
            command_pool,
            command_buffer,
            graphics_pipeline,
        }
    }

    fn record_command_buffer(command_buffer: Arc<CommandBuffer>, render_pass: Arc<RenderPass>, framebuffers: &Vec<Arc<Framebuffer>>) {

        command_buffer.begin();
        command_buffer.begin_render_pass(render_pass.clone(), framebuffers[0].clone());
        command_buffer.end_render_pass(command_buffer.clone());
        command_buffer.end();

        // Swapchain update
    }


    fn draw_frame(&mut self) {

        let index = self.swapchain.acquire_next_image();

        let fence_create_info = vk::FenceCreateInfo::default();
        let fence = unsafe { self.device.get_vk_device().create_fence(&fence_create_info, None) }.unwrap();
        self.device.submit_command_buffer(self.queue, fence, self.command_buffer.get_vk_command_buffer());

        self.swapchain.queue_present(self.queue);

    }

    pub fn run(mut self, event_loop: EventLoop<()>) {
        event_loop
            .run(move |event, elwt| {

                match event {
                    | Event::WindowEvent { event, .. } => {
                        self.window.window_event( event.clone(), elwt );

                        match event {
                            WindowEvent::RedrawRequested => {
                                self.draw_frame();
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
