use std::sync::Arc;
use ash::{vk};
use crate::vulkan::{Device};
use crate::vulkan::device::DeviceInner;

pub struct RenderPassInner {
    pub renderpass: vk::RenderPass,
    pub device_dep: Arc<DeviceInner>,
}

impl Drop for RenderPassInner {
    fn drop(&mut self) {
        unsafe {
            self.device_dep.device.destroy_render_pass(self.renderpass, None);
        }
    }
}

pub struct RenderPass {
    pub inner: Arc<RenderPassInner>,
}

impl RenderPass {
    pub fn new(device: &Device, surface_format: vk::Format) -> RenderPass {
        let color_attachment = vk::AttachmentDescription::default()
            .format(surface_format)
            .samples(vk::SampleCountFlags::TYPE_1)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .final_layout(vk::ImageLayout::PRESENT_SRC_KHR);

        let render_pass_attachments = [color_attachment];

        let color_attachment_ref = vk::AttachmentReference::default()
            .attachment( 0 )
            .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL);

        let subpass_description = vk::SubpassDescription::default()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(std::slice::from_ref(&color_attachment_ref));

        let subpass_descriptions = [subpass_description];

        let renderpass_create_info = vk::RenderPassCreateInfo::default()
            .attachments(&render_pass_attachments)
            .subpasses(&subpass_descriptions);

        let renderpass = unsafe {
            device.handle()
                .create_render_pass(&renderpass_create_info, None)
                .expect("Failed to create render pass")
        };

        let renderpass_inner = RenderPassInner {
            renderpass,
            device_dep: device.inner.clone()
        };

        RenderPass {
            inner: Arc::new(renderpass_inner),
        }
    }

    pub fn handle(&self) -> vk::RenderPass {
        self.inner.renderpass
    }

}