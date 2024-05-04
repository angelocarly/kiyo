use std::sync::Arc;
use ash::vk;
use crate::vulkan::{Device, RenderPass};

pub struct Framebuffer {
    pub framebuffer: vk::Framebuffer,
    pub device: Arc<Device>,
}

impl Framebuffer {
    pub fn new(device: Arc<Device>, extent: vk::Extent2D, render_pass: Arc<RenderPass>, attachments: Vec<vk::ImageView>) -> Self {

        let framebuffer_create_info = vk::FramebufferCreateInfo::default()
            .render_pass(render_pass.get_vk_render_pass())
            .attachments(&attachments)
            .width(extent.width)
            .height(extent.height)
            .layers(1);

        let framebuffer = unsafe {
            device.get_vk_device()
                .create_framebuffer(&framebuffer_create_info, None)
                .expect("Failed to create framebuffer")
        };

        Framebuffer {
            device,
            framebuffer
        }
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe {
            self.device.device.destroy_framebuffer(self.framebuffer, None);
        }
    }
}