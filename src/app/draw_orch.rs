use crate::vulkan::{ComputePipeline, DescriptorSetLayout, Image};

pub struct ShaderPass {
    _image: Image,
    _compute_pipeline: ComputePipeline,
    _descriptor_set_layout: DescriptorSetLayout,
}

pub struct DrawOrchestrator {
    _passes: Vec<ShaderPass>,
}

impl DrawOrchestrator {
    pub fn new() -> DrawOrchestrator {
        DrawOrchestrator {
            _passes: Vec::new(),
        }
    }
}