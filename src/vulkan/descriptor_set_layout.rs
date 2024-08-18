use std::sync::Arc;
use ash::vk;
use ash::vk::DescriptorSetLayoutBinding;
use log::trace;
use crate::vulkan::{Device, LOG_TARGET};
use crate::vulkan::device::DeviceInner;

pub struct DescriptorSetLayout {
    device_dep: Arc<DeviceInner>,
    layout: vk::DescriptorSetLayout,
}

impl Drop for DescriptorSetLayout {
    fn drop(&mut self) {
        unsafe {
            let layout_addr = format!("{:?}", self.layout);
            self.device_dep.device.destroy_descriptor_set_layout(self.layout, None);
            trace!(target: LOG_TARGET, "Destroyed descriptor set layout: [{}]", layout_addr);
        }
    }
}

impl DescriptorSetLayout {

    fn create(device: &Device, flags: vk::DescriptorSetLayoutCreateFlags, layout_bindings: &[DescriptorSetLayoutBinding]) -> DescriptorSetLayout {

        let layout_create_info = vk::DescriptorSetLayoutCreateInfo::default()
            .flags(flags)
            .bindings(layout_bindings);

        let layout = unsafe {
            device.handle()
                .create_descriptor_set_layout(&layout_create_info, None)
                .expect("Failed to create descriptor set layout")
        };

        trace!(target: LOG_TARGET, "Created descriptor set layout: {:?}", layout);

        DescriptorSetLayout {
            device_dep: device.inner.clone(),
            layout,
        }
    }

    pub fn new(device: &Device, layout_bindings: &[vk::DescriptorSetLayoutBinding]) -> DescriptorSetLayout {
        DescriptorSetLayout::create(device, vk::DescriptorSetLayoutCreateFlags::empty(), layout_bindings)
    }

    pub fn new_push_descriptor(device: &Device, layout_bindings: &[DescriptorSetLayoutBinding]) -> DescriptorSetLayout {
        DescriptorSetLayout::create(device, vk::DescriptorSetLayoutCreateFlags::PUSH_DESCRIPTOR_KHR, layout_bindings)
    }

    pub(crate) fn handle(&self) -> vk::DescriptorSetLayout {
        self.layout
    }
}