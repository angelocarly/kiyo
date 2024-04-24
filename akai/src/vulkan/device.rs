use std::sync::Arc;
use ash::khr::swapchain;
use ash::vk;
use crate::vulkan::Instance;

/// A connection to a physical GPU.
pub struct Device {
    pub device: ash::Device,
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
            device
        }
    }

    pub fn get_vk_device(&self) -> &ash::Device {
        &self.device
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