use slotmap::{DefaultKey, SlotMap};
use crate::vulkan::ComputePipeline;

pub(crate) struct PipelineStore {
    _pipelines: SlotMap<DefaultKey, ComputePipeline>,
}

impl PipelineStore {
    pub fn new() -> PipelineStore {
        PipelineStore {
            _pipelines: SlotMap::new(),
        }
    }
}