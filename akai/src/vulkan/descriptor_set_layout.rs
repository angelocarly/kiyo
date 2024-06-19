use std::sync::Arc;
use ash::vk;
use crate::vulkan::Device;
use crate::vulkan::device::DeviceInner;

pub struct DescriptorSetLayout {
    device_dep: Arc<DeviceInner>,
    layout: vk::DescriptorSetLayout,
}

impl Drop for DescriptorSetLayout {
    fn drop(&mut self) {
        unsafe {
            self.device_dep.device.destroy_descriptor_set_layout(self.layout, None);
        }
    }
}

impl DescriptorSetLayout {

    fn create(device: &Device, flags: vk::DescriptorSetLayoutCreateFlags ) -> DescriptorSetLayout {

        // TODO: Pass this through somehow, I'd rather keep ash code inside of akai
        let layout_bindings = &[
            vk::DescriptorSetLayoutBinding::default()
                .binding(0)
                .descriptor_type(vk::DescriptorType::STORAGE_IMAGE)
                .descriptor_count(1)
                .stage_flags(vk::ShaderStageFlags::COMPUTE)
        ];

        let layout_create_info = vk::DescriptorSetLayoutCreateInfo::default()
            .flags(vk::DescriptorSetLayoutCreateFlags::PUSH_DESCRIPTOR_KHR)
            .bindings(layout_bindings);

        let layout = unsafe {
            device.handle()
                .create_descriptor_set_layout(&layout_create_info, None)
                .expect("Failed to create descriptor set layout")
        };

        DescriptorSetLayout {
            device_dep: device.inner.clone(),
            layout,
        }
    }

    pub fn new(device: &Device) -> DescriptorSetLayout {
        DescriptorSetLayout::create(device, vk::DescriptorSetLayoutCreateFlags::empty())
    }

    pub fn new_push_descriptor(device: &Device) -> DescriptorSetLayout {
        DescriptorSetLayout::create(device, vk::DescriptorSetLayoutCreateFlags::PUSH_DESCRIPTOR_KHR)
    }

    pub(crate) fn handle(&self) -> vk::DescriptorSetLayout {
        self.layout
    }
}