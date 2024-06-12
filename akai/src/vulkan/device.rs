use std::sync::Arc;
use ash::khr::swapchain;
use ash::vk;
use ash::vk::{PipelineStageFlags, Queue};
use crate::vulkan::{CommandBuffer, Instance};

/// A connection to a physical GPU.
pub struct DeviceInner {
    pub device: ash::Device,
    pub queue_family_index: u32,
}

impl Drop for DeviceInner {
    fn drop(&mut self) {
        unsafe {
            self.device.device_wait_idle().unwrap();
            self.device.destroy_device(None);
        }
    }
}

pub struct Device {
    pub inner: Arc<DeviceInner>,
}

impl Device {
    pub fn new(instance: Arc<Instance>, physical_device: vk::PhysicalDevice, queue_family_index: u32) -> Device {
        let priorities = [1.0];

        let queue_info = vk::DeviceQueueCreateInfo::default()
            .queue_family_index(queue_family_index)
            .queue_priorities(&priorities);

        let device_extension_names_raw = [
            swapchain::NAME.as_ptr(),
            #[cfg(target_os = "macos")]
                ash::khr::portability_subset::NAME.as_ptr(),
        ];

        let features = vk::PhysicalDeviceFeatures {
            shader_clip_distance: 1,
            ..Default::default()
        };

        let device_create_info = vk::DeviceCreateInfo::default()
            .queue_create_infos(std::slice::from_ref(&queue_info))
            .enabled_extension_names(&device_extension_names_raw)
            .enabled_features(&features);

        let device = unsafe {
            instance.get_vk_instance()
                .create_device(physical_device, &device_create_info, None)
        }.unwrap();

        let device_inner = DeviceInner {
            device,
            queue_family_index,
        };

        Self {
            inner: Arc::new(device_inner),
        }
    }

    pub fn handle(&self) -> &ash::Device {
        &self.inner.device
    }

    pub fn get_queue(&self, queue_index: u32) -> Queue {
        unsafe { self.handle().get_device_queue(self.inner.queue_family_index, queue_index) }
    }

    pub fn wait_for_fence(&self, fence: vk::Fence) {
        unsafe {
            let fences = [fence];
            self.handle()
                .wait_for_fences(&fences, true, u64::MAX)
                .expect("Failed to destroy fence");
        }
    }

    pub fn reset_fence(&self, fence: vk::Fence) {
        unsafe {
            let fences = [fence];
            self.handle()
                .reset_fences(&fences)
                .unwrap()
        }
    }

    pub fn submit_single_time_command(
        &self,
        queue: Queue,
        command_buffer: Arc<CommandBuffer>
    ) {
        unsafe {
            let command_buffers = [command_buffer.handle()];
            let submit_info = vk::SubmitInfo::default()
                .command_buffers(&command_buffers);

            let fence_create_info = vk::FenceCreateInfo::default();
            let fence = self.handle()
                    .create_fence(&fence_create_info, None)
                    .expect("Failed to create fence");

            let submits = [submit_info];
            self.handle().queue_submit(queue, &submits, fence).unwrap();

            self.wait_for_fence(fence);

            self.handle()
                .destroy_fence(fence, None);
        }
    }

    /// Submit a command buffer for execution
    ///
    /// - `wait_semaphore` - A semaphore to wait on before execution.
    /// - `signal_semaphore` - A semaphore to signal after execution.
    /// - `fence` - A fence to signal once the commandbuffer has finished execution.
    ///
    /// https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueueSubmit.html
    pub fn submit_command_buffer(
        &self,
        queue: &Queue,
        fence: vk::Fence,
        wait_semaphore: vk::Semaphore,
        signal_semaphore: vk::Semaphore,
        command_buffer: &CommandBuffer
    ) {
        let command_buffers = [command_buffer.handle()];
        let mut submit_info = vk::SubmitInfo::default()
            .command_buffers(&command_buffers);

        let wait_semaphores = [wait_semaphore];
        let signal_semaphores = [signal_semaphore];
        let wait_dst_stage_masks = [PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];

        submit_info = submit_info
            .wait_semaphores(&wait_semaphores)
            .signal_semaphores(&signal_semaphores)
            .wait_dst_stage_mask(&wait_dst_stage_masks);

        let submits = [submit_info];
        unsafe { self.handle().queue_submit(*queue, &submits, fence).unwrap(); }
    }
}
