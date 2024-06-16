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
    pub fn new(device: &Device) -> DescriptorSetLayout {

        // TODO: Pass this through somehow, I'd rather keep ash code inside of akai
        let layout_bindings = &[
            vk::DescriptorSetLayoutBinding::default()
                .binding(0)
                .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
                .descriptor_count(1)
                .stage_flags(vk::ShaderStageFlags::VERTEX),
        ];

        let layout_create_info = vk::DescriptorSetLayoutCreateInfo::default()
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
}