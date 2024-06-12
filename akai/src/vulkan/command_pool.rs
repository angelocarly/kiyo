use std::sync::Arc;
use ash::vk;
use crate::vulkan::Device;
use crate::vulkan::device::DeviceInner;

pub struct CommandPool {
    pub device_dep: Arc<DeviceInner>,
    pub command_pool: vk::CommandPool,
}

impl CommandPool {

    pub fn new(device: &Device, queue_family_index: u32) -> CommandPool {

        let command_pool_create_info = vk::CommandPoolCreateInfo::default()
            .queue_family_index(queue_family_index)
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER);

        let command_pool = unsafe {
            device.handle()
                .create_command_pool(&command_pool_create_info, None)
                .expect("Failed to create command pool")
        };

        Self {
            device_dep: device.inner.clone(),
            command_pool
        }
    }

    pub fn get_vk_command_pool(&self) -> vk::CommandPool {
        self.command_pool
    }

}

impl Drop for CommandPool {
    fn drop(&mut self) {
        unsafe {
            self.device_dep.device.destroy_command_pool(self.command_pool, None);
        }
    }
}