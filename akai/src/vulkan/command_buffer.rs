use std::sync::Arc;
use ash::vk;
use ash::vk::PipelineBindPoint;
use crate::vulkan::{CommandPool, Device, Framebuffer, GraphicsPipeline, RenderPass};
use crate::vulkan::device::DeviceInner;

pub struct CommandBuffer {
    device_dep: Arc<DeviceInner>,
    command_buffer: vk::CommandBuffer,
}

impl CommandBuffer {
    pub fn new(device: &Device, command_pool: &CommandPool) -> CommandBuffer {
        let command_buffer_allocate_info = vk::CommandBufferAllocateInfo::default()
            .command_pool(command_pool.handle())
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(1);

        let command_buffer = unsafe {
            device.handle()
                .allocate_command_buffers(&command_buffer_allocate_info)
                .map(|command_buffers| command_buffers[0])
                .expect("Failed to allocate command buffers")
        };

        CommandBuffer {
            device_dep: device.inner.clone(),
            command_buffer
        }
    }

    pub fn begin(&self) {
        let command_buffer_begin_info = vk::CommandBufferBeginInfo::default();
        unsafe {
            self.device_dep.device
                .begin_command_buffer(self.command_buffer, &command_buffer_begin_info)
                .expect("Failed to begin command buffer");
        }
    }

    pub fn end(&self) {
        unsafe {
            self.device_dep.device
                .end_command_buffer(self.command_buffer)
                .expect("Failed to end command buffer");
        }
    }

    pub fn begin_render_pass(&self, render_pass: &RenderPass, framebuffer: &Framebuffer) {
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
            .render_pass(render_pass.handle())
            .framebuffer(framebuffer.handle());
        unsafe {
            self.device_dep.device
                .cmd_begin_render_pass(self.command_buffer, &render_pass_begin_info, vk::SubpassContents::INLINE);
        }
    }

    pub fn end_render_pass(&self) {
        unsafe {
            self.device_dep.device
                .cmd_end_render_pass(self.command_buffer);
        }
    }

    pub fn set_viewport(&self, viewport: vk::Viewport) {
        unsafe {
            self.device_dep.device
                .cmd_set_viewport(self.command_buffer, 0, &[viewport]);
        }
    }

    pub fn set_scissor(&self, scissor: vk::Rect2D) {
        unsafe {
            self.device_dep.device
                .cmd_set_scissor(self.command_buffer, 0, &[scissor]);
        }
    }

    pub fn bind_pipeline(&self, pipeline: &GraphicsPipeline) {
        unsafe {
            self.device_dep.device
                .cmd_bind_pipeline(self.command_buffer, PipelineBindPoint::GRAPHICS, pipeline.handle());
        }
    }
    pub fn get_vk_command_buffer(&self) -> vk::CommandBuffer {
        self.command_buffer
    }
}