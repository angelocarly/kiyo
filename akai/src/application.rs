use std::sync::Arc;
use ash::vk;
use ash::vk::{FenceCreateFlags, PhysicalDevice, Queue};
use winit::event::{Event, StartCause, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::run_on_demand::EventLoopExtRunOnDemand;
use crate::vulkan::{Device, Instance, Surface, Swapchain, RenderPass, Framebuffer, CommandPool, CommandBuffer};
use crate::window::Window;

pub trait GameHandler {
    fn render(&mut self, render_context: &RenderContext);
}

pub struct GraphicsContext {
    pub render_pass: Arc<RenderPass>,
    pub device: Arc<Device>,
    pub physical_device: PhysicalDevice,
    pub instance: Arc<Instance>,
}

pub struct RenderContext {
    render_pass: Arc<RenderPass>,
    framebuffer: Arc<Framebuffer>,
    pub command_buffer: Arc<CommandBuffer>,
}

impl RenderContext {
    pub fn begin_root_render_pass(&self) {
        self.command_buffer.begin_render_pass(
            self.render_pass.clone(),
            self.framebuffer.clone()
        );
    }
}

/// Generative art runtime.
/// Manages the window and graphics recording.
pub struct Application {
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
    pub graphics_context: Arc<GraphicsContext>,
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

        // Per frame resources

        let framebuffers = swapchain.clone().get_image_views().iter().map(|image_view| {
            Arc::new(Framebuffer::new(device.clone(), swapchain.get_extent(), render_pass.clone(), vec![image_view.clone()]))
        }).collect::<Vec<Arc<Framebuffer>>>();

        let command_buffers = (0..swapchain.clone().get_image_count()).map(|_| {
            Arc::new(CommandBuffer::new(device.clone(), command_pool.clone()))
        }).collect::<Vec<Arc<CommandBuffer>>>();

        let image_available_semaphores = (0..swapchain.clone().get_image_count()).map(|_| unsafe {
            let semaphore_create_info = vk::SemaphoreCreateInfo::default();
            device.get_vk_device().create_semaphore(&semaphore_create_info, None)
                .expect("Failed to create semaphore")
        }).collect::<Vec<vk::Semaphore>>();

        let render_finished_semaphores = (0..swapchain.clone().get_image_count()).map(|_| unsafe {
            let semaphore_create_info = vk::SemaphoreCreateInfo::default();
            device.get_vk_device().create_semaphore(&semaphore_create_info, None)
                .expect("Failed to create semaphore")
        }).collect::<Vec<vk::Semaphore>>();

        let in_flight_fences = (0..swapchain.clone().get_image_count()).map(|_| {
            unsafe {
                let fence_create_info = vk::FenceCreateInfo::default()
                    .flags(FenceCreateFlags::SIGNALED);
                device.get_vk_device().create_fence(&fence_create_info, None)
                    .expect("Failed to create fence")
            }
        }).collect::<Vec<vk::Fence>>();

        let graphics_context = Arc::new(GraphicsContext {
            render_pass: render_pass.clone(),
            device: device.clone(),
            physical_device,
            instance: instance.clone(),
        });

        Self {
            window,
            entry,
            graphics_context,
            surface,
            queue,
            swapchain,
            framebuffers,
            render_finished_semaphores,
            image_available_semaphores,
            in_flight_fences,
            command_pool,
            command_buffers,
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

    fn record_command_buffer(&mut self, command_buffer: Arc<CommandBuffer>, framebuffer: Arc<Framebuffer>, game_handler: &mut dyn GameHandler) {

        command_buffer.begin();

        // Dynamic state
        command_buffer.set_scissor(
            vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: self.swapchain.get_extent(),
            }
        );
        command_buffer.set_viewport(
            vk::Viewport {
                x: 0.0,
                y: 0.0,
                width: self.swapchain.get_extent().width as f32,
                height: self.swapchain.get_extent().height as f32,
                min_depth: 0.0,
                max_depth: 1.0,
            }
        );

        let render_context = RenderContext {
            render_pass: self.graphics_context.render_pass.clone(),
            framebuffer: framebuffer.clone(),
            command_buffer: command_buffer.clone(),
        };
        game_handler.render(&render_context);

        command_buffer.end();
    }

    fn draw_frame(&mut self, game_handler: &mut dyn GameHandler) {

        // Wait for the corresponding command buffer to finish executing.
        self.graphics_context.device.wait_for_fence(self.in_flight_fences[self.frame_index]);

        let index = self.swapchain.acquire_next_image(self.image_available_semaphores[self.frame_index]) as usize;

        self.record_command_buffer(self.command_buffers[self.frame_index].clone(), self.framebuffers[index].clone(), game_handler);

        self.graphics_context.device.reset_fence(self.in_flight_fences[self.frame_index]);
        self.graphics_context.device.submit_command_buffer(
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

    pub fn get_graphics_context(&self) -> Arc<GraphicsContext> {
        self.graphics_context.clone()
    }

    pub fn run(mut self, mut event_loop: EventLoop<()>, game_handler: &mut dyn GameHandler) {
        event_loop
            .run_on_demand(move |event, elwt| {
                elwt.set_control_flow(ControlFlow::Poll);

                match event {
                    | Event::NewEvents(StartCause::Poll) => {
                        self.draw_frame(game_handler);
                    }
                    | Event::WindowEvent { event, .. } => {
                        self.window.window_event( event.clone(), elwt );

                        match event {
                            WindowEvent::RedrawRequested => {
                                self.draw_frame(game_handler);
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
            self.graphics_context.device.get_vk_device().device_wait_idle().unwrap();
            for semaphore in &self.render_finished_semaphores {
                self.graphics_context.device.get_vk_device().destroy_semaphore(*semaphore, None);
            }
            for semaphore in &self.image_available_semaphores {
                self.graphics_context.device.get_vk_device().destroy_semaphore(*semaphore, None);
            }
            for fence in &self.in_flight_fences {
                self.graphics_context.device.get_vk_device().destroy_fence(*fence, None);
            }
        }
    }
}
