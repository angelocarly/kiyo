use std::sync::Arc;
use ash::vk;
use ash::vk::{FenceCreateFlags, PhysicalDevice, Queue};
use crate::application::{GameHandler, RenderContext};
use crate::vulkan::{CommandBuffer, CommandPool, Device, Framebuffer, Instance, RenderPass, Surface, Swapchain};
use crate::window::Window;

pub struct Renderer {
    pub render_finished_semaphores: Vec<vk::Semaphore>,
    pub image_available_semaphores: Vec<vk::Semaphore>,
    pub command_buffers: Vec<CommandBuffer>,
    pub command_pool: CommandPool,
    pub queue: Queue,
    pub framebuffers: Vec<Framebuffer>,
    pub swapchain: Swapchain,
    pub entry: ash::Entry,
    pub surface: Surface,
    pub frame_index: usize,
    pub in_flight_fences: Vec<vk::Fence>,
    pub render_pass: RenderPass,
    pub device: Device,
    pub physical_device: PhysicalDevice,
    pub instance: Instance,
}

impl Renderer {
    pub fn new(window: &Window) -> Renderer {
        let entry = ash::Entry::linked();
        let instance = Instance::new(&entry, window.display_handle());
        let surface = Surface::new(&entry, &instance, &window);
        let (physical_device, queue_family_index) = instance.create_physical_device(&entry, &surface);
        let device = Device::new(&instance, physical_device, queue_family_index);
        let queue = device.get_queue(0);
        let command_pool = CommandPool::new(&device, queue_family_index);

        let swapchain = Swapchain::new(&instance, &physical_device, &device, &window, &surface);
        Self::transition_swapchain_images(&device, &command_pool, &queue, &swapchain);

        let render_pass = RenderPass::new(&device, swapchain.get_format().format);

        // Per frame resources

        let framebuffers = swapchain.get_image_views().iter().map(|image_view| {
            Framebuffer::new(&device, swapchain.get_extent(), &render_pass, vec![image_view.clone()])
        }).collect::<Vec<Framebuffer>>();

        let command_buffers = (0..swapchain.get_image_count()).map(|_| {
            CommandBuffer::new(&device, &command_pool)
        }).collect::<Vec<CommandBuffer>>();

        let image_available_semaphores = (0..swapchain.get_image_count()).map(|_| unsafe {
            let semaphore_create_info = vk::SemaphoreCreateInfo::default();
            device.handle().create_semaphore(&semaphore_create_info, None)
                .expect("Failed to create semaphore")
        }).collect::<Vec<vk::Semaphore>>();

        let render_finished_semaphores = (0..swapchain.get_image_count()).map(|_| unsafe {
            let semaphore_create_info = vk::SemaphoreCreateInfo::default();
            device.handle().create_semaphore(&semaphore_create_info, None)
                .expect("Failed to create semaphore")
        }).collect::<Vec<vk::Semaphore>>();

        let in_flight_fences = (0..swapchain.get_image_count()).map(|_| {
            unsafe {
                let fence_create_info = vk::FenceCreateInfo::default()
                    .flags(FenceCreateFlags::SIGNALED);
                device.handle().create_fence(&fence_create_info, None)
                    .expect("Failed to create fence")
            }
        }).collect::<Vec<vk::Fence>>();

        Self {
            entry,
            device,
            physical_device,
            instance,
            render_pass,
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

    fn transition_swapchain_images(device: &Device, command_pool: &CommandPool, queue: &Queue, swapchain: &Swapchain) {
        let image_command_buffer = Arc::new(CommandBuffer::new(device, command_pool));
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
                device.handle().cmd_pipeline_barrier(
                    image_command_buffer.handle(),
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

    fn record_command_buffer(&mut self, frame_index: usize, game_handler: &mut dyn GameHandler) {

        let command_buffer = &self.command_buffers[frame_index];

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
            device: &self.device,
            render_pass: &self.render_pass,
            framebuffer: &self.framebuffers[frame_index],
            command_buffer: command_buffer,
        };
        game_handler.render(&render_context);

        command_buffer.end();
    }

    pub fn draw_frame(&mut self, game_handler: &mut dyn GameHandler) {

        // Wait for the corresponding command buffer to finish executing.
        self.device.wait_for_fence(self.in_flight_fences[self.frame_index]);

        let index = self.swapchain.acquire_next_image(self.image_available_semaphores[self.frame_index]) as usize;

        self.record_command_buffer(self.frame_index, game_handler);

        self.device.reset_fence(self.in_flight_fences[self.frame_index]);
        self.device.submit_command_buffer(
            &self.queue,
            self.in_flight_fences[self.frame_index],
            self.image_available_semaphores[self.frame_index],
            self.render_finished_semaphores[self.frame_index],
            &self.command_buffers[self.frame_index]
        );

        self.swapchain.queue_present(
            self.queue,
            self.render_finished_semaphores[self.frame_index],
            index as u32
        );

        self.frame_index = ( self.frame_index + 1 ) % self.swapchain.get_image_views().len();
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            self.device.handle().device_wait_idle().unwrap();
            for semaphore in &self.render_finished_semaphores {
                self.device.handle().destroy_semaphore(*semaphore, None);
            }
            for semaphore in &self.image_available_semaphores {
                self.device.handle().destroy_semaphore(*semaphore, None);
            }
            for fence in &self.in_flight_fences {
                self.device.handle().destroy_fence(*fence, None);
            }
        }
    }
}
