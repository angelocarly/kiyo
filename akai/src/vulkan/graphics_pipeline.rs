use std::sync::Arc;
use ash::vk;
use crate::vulkan::Device;

pub struct GraphicsPipeline {
    pipeline_layout: vk::PipelineLayout,
}

impl GraphicsPipeline {
    pub fn new(device: Arc<Device>) -> GraphicsPipeline {
        let create_info = vk::PipelineLayoutCreateInfo::default();
        let pipeline_layout = unsafe { device.get_vk_device().create_pipeline_layout(&create_info, None).expect("Failed to create pipeline layout") };

        Self {
            pipeline_layout
        }
    }
}