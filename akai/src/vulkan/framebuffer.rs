use std::sync::Arc;
use ash::vk;
use ash::vk::Extent2D;
use crate::vulkan::{Device, RenderPass};

pub struct FramebufferInner {
    pub framebuffer: vk::Framebuffer,
    pub device: Arc<Device>,
    pub extent: Extent2D,
}

impl Drop for FramebufferInner {
    fn drop(&mut self) {
        unsafe {
            self.device.device.destroy_framebuffer(self.framebuffer, None);
        }
    }
}

pub struct Framebuffer {
    pub inner: Arc<FramebufferInner>,
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

        let framebuffer_inner = FramebufferInner {
            device,
            framebuffer,
            extent
        };

        Framebuffer {
            inner: Arc::new(framebuffer_inner)
        }
    }

    pub fn handle(&self) -> vk::Framebuffer {
        self.inner.framebuffer
    }

    pub fn get_extent(&self) -> vk::Extent2D {
        self.inner.extent
    }
}
