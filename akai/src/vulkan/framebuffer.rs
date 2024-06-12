use std::sync::Arc;
use ash::vk;
use ash::vk::Extent2D;
use crate::vulkan::{Device, RenderPass};

pub struct Framebuffer {
    pub framebuffer: vk::Framebuffer,
    pub device: Arc<Device>,
    pub extent: Extent2D,
}

impl Framebuffer {
    pub fn new(device: Arc<Device>, extent: vk::Extent2D, render_pass: &RenderPass, attachments: Vec<vk::ImageView>) -> Self {

        let framebuffer_create_info = vk::FramebufferCreateInfo::default()
            .render_pass(render_pass.handle())
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
            framebuffer,
            extent
        }
    }

    pub fn get_vk_framebuffer(&self) -> vk::Framebuffer {
        self.framebuffer
    }

    pub fn get_extent(&self) -> vk::Extent2D {
        self.extent
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe {
            self.device.device.destroy_framebuffer(self.framebuffer, None);
        }
    }
}