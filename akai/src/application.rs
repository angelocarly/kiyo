use std::sync::Arc;
use ash::vk;
use ash::vk::{FenceCreateFlags, PhysicalDevice, Queue};
use winit::event::{Event, StartCause, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use crate::vulkan::{Device, GraphicsPipeline, Instance, Surface, Swapchain, RenderPass, Framebuffer, CommandPool, CommandBuffer};
use crate::window::Window;

/// Generative art runtime.
/// Manages the window and graphics recording.
pub struct Application {
    pub graphics_pipeline: Arc<GraphicsPipeline>,
    pub render_pass: Arc<RenderPass>,
    pub render_finished_semaphores: Vec<vk::Semaphore>,
    pub image_available_semaphores: Vec<vk::Semaphore>,
    pub command_buffers: Vec<Arc<CommandBuffer>>,
    pub command_pool: Arc<CommandPool>,
    pub queue: Queue,
    pub framebuffers: Vec<Arc<Framebuffer>>,
    pub swapchain: Arc<Swapchain>,
    pub window: Window,
    pub entry: Arc<ash::Entry>,
    pub surface: Arc<Surface>,
    pub device: Arc<Device>,
    pub physical_device: PhysicalDevice,
    pub instance: Arc<Instance>,
    pub frame_index: usize,
    pub in_flight_fences: Vec<vk::Fence>,
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
        let command_pool = Arc::new(CommandPool::new(device.clone(), queue_family_index));

        let swapchain = Arc::new(Swapchain::new(instance.clone(), &physical_device, device.clone(), &window, surface.clone()));
        Self::transition_swapchain_images(device.clone(), command_pool.clone(), &queue, swapchain.clone());

        let render_pass = Arc::new(RenderPass::new(device.clone(), swapchain.get_format().format));

        let framebuffers = swapchain.clone().get_image_views().iter().map(|image_view| {
            Arc::new(Framebuffer::new(device.clone(), swapchain.get_extent(), render_pass.clone(), vec![image_view.clone()]))
        }).collect::<Vec<Arc<Framebuffer>>>();

        let command_buffers = swapchain.clone().get_image_views().iter().map(|_| {
            Arc::new(CommandBuffer::new(device.clone(), command_pool.clone()))
        }).collect::<Vec<Arc<CommandBuffer>>>();

        let image_available_semaphores = swapchain.clone().get_image_views().iter().map(|_| unsafe {
            let semaphore_create_info = vk::SemaphoreCreateInfo::default();
            device.get_vk_device().create_semaphore(&semaphore_create_info, None)
                .expect("Failed to create semaphore")
        }).collect::<Vec<vk::Semaphore>>();
        let render_finished_semaphores = swapchain.clone().get_image_views().iter().map(|_| unsafe {
            let semaphore_create_info = vk::SemaphoreCreateInfo::default();
            device.get_vk_device().create_semaphore(&semaphore_create_info, None)
                .expect("Failed to create semaphore")
        }).collect::<Vec<vk::Semaphore>>();
        let in_flight_fences = swapchain.clone().get_image_views().iter().map(|_| {
            unsafe {
                let fence_create_info = vk::FenceCreateInfo::default()
                    .flags(FenceCreateFlags::SIGNALED);
                device.get_vk_device().create_fence(&fence_create_info, None)
                    .expect("Failed to create fence")
            }
        }).collect::<Vec<vk::Fence>>();

        let graphics_pipeline = Arc::new(GraphicsPipeline::new(device.clone(), render_pass.clone()));

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
            render_finished_semaphores,
            image_available_semaphores,
            in_flight_fences,
            command_pool,
            command_buffers,
            graphics_pipeline,
            frame_index: 0,
        }
    }

    fn transition_swapchain_images(device: Arc<Device>, command_pool: Arc<CommandPool>, queue: &Queue, swapchain: Arc<Swapchain>) {
        let image_command_buffer = Arc::new(CommandBuffer::new(device.clone(), command_pool.clone()));
        image_command_buffer.begin();
        swapchain.get_images().iter().for_each(|image| {
            let image_memory_barrier = vk::ImageMemoryBarrier::default()
                .old_layout(vk::ImageLayout::UNDEFINED)
                .new_layout(vk::ImageLayout::PRESENT_SRC_KHR)
                .src_access_mask(vk::AccessFlags::empty())
                .dst_access_mask(vk::AccessFlags::empty())
                .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .image(*image)
                .subresource_range(vk::ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,
                });
            unsafe {
                device.get_vk_device().cmd_pipeline_barrier(
                    image_command_buffer.get_vk_command_buffer(),
                    vk::PipelineStageFlags::TOP_OF_PIPE,
                    vk::PipelineStageFlags::BOTTOM_OF_PIPE,
                    vk::DependencyFlags::empty(),
                    &[],
                    &[],
                    &[image_memory_barrier]
                )
            }
        });
        image_command_buffer.end();
        device.submit_single_time_command(*queue, image_command_buffer);
    }

    fn record_command_buffer(&mut self, command_buffer: Arc<CommandBuffer>, framebuffer: Arc<Framebuffer>) {

        command_buffer.begin();
        command_buffer.begin_render_pass(self.render_pass.clone(), framebuffer.clone());
        command_buffer.end_render_pass(command_buffer.clone());
        command_buffer.end();
    }

    fn draw_frame(&mut self) {

        // Wait for the corresponding command buffer to finish executing.
        self.device.wait_for_fence(self.in_flight_fences[self.frame_index]);

        let index = self.swapchain.acquire_next_image(self.image_available_semaphores[self.frame_index]) as usize;

        self.record_command_buffer(self.command_buffers[self.frame_index].clone(), self.framebuffers[index].clone());

        self.device.reset_fence(self.in_flight_fences[self.frame_index]);
        self.device.submit_command_buffer(
            &self.queue,
            self.in_flight_fences[self.frame_index],
            self.image_available_semaphores[self.frame_index],
            self.render_finished_semaphores[self.frame_index],
            self.command_buffers[self.frame_index].clone()
        );

        self.swapchain.queue_present(
            self.queue,
            self.render_finished_semaphores[self.frame_index],
            index as u32
        );

        self.frame_index = ( self.frame_index + 1 ) % self.swapchain.get_image_views().len();
    }

    pub fn run(mut self, event_loop: EventLoop<()>) {
        event_loop
            .run(move |event, elwt| {
                elwt.set_control_flow(ControlFlow::Poll);

                match event {
                    | Event::NewEvents(StartCause::Poll) => {
                        self.draw_frame();
                    }
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

impl Drop for Application {
    fn drop(&mut self) {
        unsafe {
            self.device.get_vk_device().device_wait_idle().unwrap();
            for semaphore in &self.render_finished_semaphores {
                self.device.get_vk_device().destroy_semaphore(*semaphore, None);
            }
            for semaphore in &self.image_available_semaphores {
                self.device.get_vk_device().destroy_semaphore(*semaphore, None);
            }
            for fence in &self.in_flight_fences {
                self.device.get_vk_device().destroy_fence(*fence, None);
            }
        }
    }
}
