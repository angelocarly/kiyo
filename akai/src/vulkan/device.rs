use std::sync::Arc;
use ash::khr::swapchain;
use ash::vk;
use ash::vk::Queue;
use crate::vulkan::Instance;

/// A connection to a physical GPU.
pub struct Device {
    pub device: ash::Device,
    pub queue_family_index: u32,
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

        Self {
            device,
            queue_family_index
        }
    }

    pub fn get_vk_device(&self) -> &ash::Device {
        &self.device
    }

    pub fn get_queue(&self, queue_index: u32) -> Queue {
        unsafe { self.device.get_device_queue(self.queue_family_index, queue_index) }
    }

    pub fn submit_command_buffer(&self, queue: &Queue, fence: vk::Fence, semaphore: Option<vk::Semaphore>, command_buffer: vk::CommandBuffer) {
        let command_buffers = [command_buffer];
        let mut submit_info = vk::SubmitInfo::default()
            .command_buffers(&command_buffers);

        if let Some(s) = semaphore {
            let semaphores = [s];
            submit_info = submit_info.signal_semaphores(&semaphores);

            let submits = [submit_info];
            unsafe { self.device.queue_submit(*queue, &submits, fence).unwrap(); }
        }
        else {
            let submits = [submit_info];
            unsafe { self.device.queue_submit(*queue, &submits, fence).unwrap(); }
        }

    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            self.device.device_wait_idle().unwrap();
            self.device.destroy_device(None);
        }
    }
}