use std::sync::Arc;
use ash::vk;
use crate::vulkan::{CommandPool, Device};

pub struct CommandBuffer {
    device: Arc<Device>,
    command_buffer: vk::CommandBuffer,
}

impl CommandBuffer {
    pub fn new(device: Arc<Device>, command_pool: Arc<CommandPool>) -> CommandBuffer {
        let command_buffer_allocate_info = vk::CommandBufferAllocateInfo::default()
            .command_pool(command_pool.get_vk_command_pool())
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(1);

        let command_buffer = unsafe {
            device.get_vk_device()
                .allocate_command_buffers(&command_buffer_allocate_info)
                .map(|command_buffers| command_buffers[0])
                .expect("Failed to allocate command buffers")
        };

        CommandBuffer {
            device,
            command_buffer
        }
    }

    pub fn get_vk_command_buffer(&self) -> vk::CommandBuffer {
        self.command_buffer
    }
}