use std::sync::Arc;
use ash::vk;
use ash::vk::{Extent3D, FenceCreateFlags, ImageAspectFlags, ImageSubresourceLayers, Offset3D, PhysicalDevice, Queue};
use gpu_allocator::vulkan::{AllocatorCreateDesc};
use crate::app::{DrawOrchestrator, Window};
use crate::vulkan::{Allocator, CommandBuffer, CommandPool, Device, Instance, Surface, Swapchain};

pub struct Renderer {
    pub render_finished_semaphores: Vec<vk::Semaphore>,
    pub image_available_semaphores: Vec<vk::Semaphore>,
    pub command_buffers: Vec<CommandBuffer>,
    pub command_pool: CommandPool,
    pub queue: Queue,
    pub swapchain: Swapchain,
    pub entry: ash::Entry,
    pub surface: Surface,
    pub frame_index: usize,
    pub in_flight_fences: Vec<vk::Fence>,
    pub allocator: Allocator,
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

        let allocator = Allocator::new(&AllocatorCreateDesc {
            instance: instance.handle().clone(),
            device: device.handle().clone(),
            physical_device,
            debug_settings: Default::default(),
            buffer_device_address: false,  // Ideally, check the BufferDeviceAddressFeatures struct.
            allocation_sizes: Default::default(),
        });

        let swapchain = Swapchain::new(&instance, &physical_device, &device, &window, &surface);
        Self::transition_swapchain_images(&device, &command_pool, &queue, &swapchain);

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
            allocator,
            surface,
            queue,
            swapchain,
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

    fn record_command_buffer(&mut self, frame_index: usize, draw_orchestrator: &mut DrawOrchestrator) {

        let command_buffer = &self.command_buffers[frame_index];

        command_buffer.begin();

        // Setup dynamic state
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

        // Clear the swapchain image
        let swapchain_image = self.swapchain.get_images()[frame_index];
        self.transition_image(
            command_buffer,
            &swapchain_image,
            vk::ImageLayout::PRESENT_SRC_KHR,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            vk::PipelineStageFlags::TOP_OF_PIPE,
            vk::PipelineStageFlags::TRANSFER,
            vk::AccessFlags::NONE,
            vk::AccessFlags::TRANSFER_WRITE
        );
        unsafe {
            self.device.handle()
                .cmd_clear_color_image(
                    command_buffer.handle(),
                    swapchain_image,
                    vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                    &vk::ClearColorValue {
                           float32: [0.0, 1.0, 0.0, 1.0]
                    },
                    &[vk::ImageSubresourceRange {
                        aspect_mask: ImageAspectFlags::COLOR,
                        base_mip_level: 0,
                        level_count: 1,
                        base_array_layer: 0,
                        layer_count: 1,
                    }]
                );
        }

        // Clear the draw image
        let draw_image = &draw_orchestrator.image;
        unsafe {
            self.device.handle()
                .cmd_clear_color_image(
                    command_buffer.handle(),
                    *draw_image.handle(),
                    vk::ImageLayout::GENERAL,
                    &vk::ClearColorValue {
                            float32: [0.0, 0.0, 1.0, 1.0]
                    },
                    &[vk::ImageSubresourceRange {
                        aspect_mask: ImageAspectFlags::COLOR,
                        base_mip_level: 0,
                        level_count: 1,
                        base_array_layer: 0,
                        layer_count: 1,
                    }]
                );
        }

        // Synchronize between clear and the compute
        self.transition_image(
            command_buffer,
            &draw_image.handle(),
            vk::ImageLayout::GENERAL,
            vk::ImageLayout::GENERAL,
            vk::PipelineStageFlags::TRANSFER,
            vk::PipelineStageFlags::COMPUTE_SHADER,
            vk::AccessFlags::TRANSFER_WRITE,
            vk::AccessFlags::SHADER_READ
        );

        // Render to the draw image
        draw_orchestrator.passes.iter().for_each(|p| {
            command_buffer.bind_pipeline(&p.compute_pipeline);
            command_buffer.bind_push_descriptor_image(&p.compute_pipeline, &draw_image);
            command_buffer.dispatch(p.dispatches.x, p.dispatches.y, p.dispatches.z);

            // Synchronize between compute and compute
            self.transition_image(
                command_buffer,
                &draw_image.handle(),
                vk::ImageLayout::GENERAL,
                vk::ImageLayout::GENERAL,
                vk::PipelineStageFlags::COMPUTE_SHADER,
                vk::PipelineStageFlags::COMPUTE_SHADER,
                vk::AccessFlags::SHADER_WRITE,
                vk::AccessFlags::SHADER_READ
            );
        });

        // Synchronize between compute and transfer
        self.transition_image(
            command_buffer,
            &draw_image.handle(),
            vk::ImageLayout::GENERAL,
            vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
            vk::PipelineStageFlags::COMPUTE_SHADER,
            vk::PipelineStageFlags::TRANSFER,
            vk::AccessFlags::SHADER_WRITE,
            vk::AccessFlags::TRANSFER_READ
        );

        // Copy draw image to the swapchain
        unsafe {
            self.device.handle()
                .cmd_copy_image(
                    command_buffer.handle(),
                    *draw_image.handle(),
                    vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                    swapchain_image,
                    vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                    &[
                        vk::ImageCopy::default()
                            .extent(
                                Extent3D::default()
                                    .width(draw_orchestrator.image.width)
                                    .height(draw_orchestrator.image.height)
                                    .depth(1)
                            )
                            .dst_offset(Offset3D::default())
                            .src_offset(Offset3D::default())
                            .src_subresource(
                                ImageSubresourceLayers::default()
                                    .aspect_mask(ImageAspectFlags::COLOR)
                                    .base_array_layer(0)
                                    .layer_count(1)
                                    .mip_level(0)
                            )
                            .dst_subresource(
                                ImageSubresourceLayers::default()
                                    .aspect_mask(ImageAspectFlags::COLOR)
                                    .base_array_layer(0)
                                    .layer_count(1)
                                    .mip_level(0)
                            )
                    ]
                )
        }

        // Transfer back to default states
        self.transition_image(
            command_buffer,
            &swapchain_image,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            vk::ImageLayout::PRESENT_SRC_KHR,
            vk::PipelineStageFlags::TRANSFER,
            vk::PipelineStageFlags::BOTTOM_OF_PIPE,
            vk::AccessFlags::TRANSFER_WRITE,
            vk::AccessFlags::NONE
        );
        self.transition_image(
            command_buffer,
            &draw_image.handle(),
            vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
            vk::ImageLayout::GENERAL,
            vk::PipelineStageFlags::TRANSFER,
            vk::PipelineStageFlags::BOTTOM_OF_PIPE,
            vk::AccessFlags::TRANSFER_READ,
            vk::AccessFlags::NONE
        );

        command_buffer.end();
    }

    pub fn transition_image(
        &self,
        command_buffer: &CommandBuffer,
        image: &vk::Image,
        old_layout: vk::ImageLayout,
        new_layout: vk::ImageLayout,
        src_stage_mask: vk::PipelineStageFlags,
        dst_stage_mask: vk::PipelineStageFlags,
        src_access_flags: vk::AccessFlags,
        dst_access_flags: vk::AccessFlags,
    ) {
        let image_memory_barrier = vk::ImageMemoryBarrier::default()
            .old_layout(old_layout)
            .new_layout(new_layout)
            .src_access_mask(src_access_flags)
            .dst_access_mask(dst_access_flags)
            .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .image(*image)
            .subresource_range(vk::ImageSubresourceRange {
                aspect_mask: ImageAspectFlags::COLOR,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            });
        unsafe {
            self.device.handle().cmd_pipeline_barrier(
                command_buffer.handle(),
                src_stage_mask,
                dst_stage_mask,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &[image_memory_barrier]
            )
        }
    }


    pub fn draw_frame(&mut self, draw_orchestrator: &mut DrawOrchestrator) {

        // Wait for the current frame's command buffer to finish executing.
        self.device.wait_for_fence(self.in_flight_fences[self.frame_index]);

        let index = self.swapchain.acquire_next_image(self.image_available_semaphores[self.frame_index]) as usize;

        self.record_command_buffer(self.frame_index, draw_orchestrator);

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
