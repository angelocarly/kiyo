use std::sync::Arc;
use ash::vk;
use crate::vulkan::{CommandPool, Device, Framebuffer, RenderPass};

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

    pub fn begin(&self) {
        let command_buffer_begin_info = vk::CommandBufferBeginInfo::default();
        unsafe {
            self.device.get_vk_device()
                .begin_command_buffer(self.command_buffer, &command_buffer_begin_info)
                .expect("Failed to begin command buffer");
        }
    }

    pub fn end(&self) {
        unsafe {
            self.device.get_vk_device()
                .end_command_buffer(self.command_buffer)
                .expect("Failed to end command buffer");
        }
    }

    pub fn begin_render_pass(&self, render_pass: Arc<RenderPass>, framebuffer: Arc<Framebuffer>) {
        let render_pass_begin_info = vk::RenderPassBeginInfo::default()
            .render_area(vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: framebuffer.get_extent(),
            })
            .clear_values(&[vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: [0.0, 0.0, 0.0, 1.0],
                },
            }])
            .render_pass(render_pass.get_vk_render_pass())
            .framebuffer(framebuffer.get_vk_framebuffer());
        unsafe {
            self.device.get_vk_device()
                .cmd_begin_render_pass(self.command_buffer, &render_pass_begin_info, vk::SubpassContents::INLINE);
        }
    }

    pub fn end_render_pass(&self, command_buffer: Arc<CommandBuffer>) {
        unsafe {
            self.device.get_vk_device()
                .cmd_end_render_pass(command_buffer.get_vk_command_buffer());
        }
    }
    pub fn get_vk_command_buffer(&self) -> vk::CommandBuffer {
        self.command_buffer
    }
}