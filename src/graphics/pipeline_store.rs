use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use ash::vk;

use slotmap::{DefaultKey, SlotMap};

use crate::vulkan::{ComputePipeline, DescriptorSetLayout, Device, PipelineErr};

pub struct PipelineConfig {
    pub shader_path: PathBuf,
    pub descriptor_set_layouts: Vec<DescriptorSetLayout>,
    pub push_constant_ranges: Vec<vk::PushConstantRange>,
    pub macros: HashMap<String, String>,
}

struct PipelineHandle {
    config: PipelineConfig,
    pipeline: ComputePipeline,
}

struct PipelineStoreInner {
    device: Device,
    pipelines: SlotMap<DefaultKey, PipelineHandle>,
}

pub struct PipelineStore {
    inner: Arc<Mutex<PipelineStoreInner>>,
}

impl PipelineStore {
    pub fn new(device: &Device) -> PipelineStore {

        PipelineStore {
            inner: Arc::new(Mutex::new(PipelineStoreInner{
                device: device.clone(),
                pipelines: SlotMap::new(),
            }))
        }
    }

    pub fn insert(&mut self, config: PipelineConfig) -> Result<DefaultKey, PipelineErr> {
        let mut inner = self.inner.lock().unwrap();
        let pipeline = ComputePipeline::new(
            &inner.device,
            config.shader_path.clone(),
            &config.descriptor_set_layouts.as_slice(),
            &config.push_constant_ranges.as_slice(),
            &config.macros
        )?;

        Ok(inner.pipelines.insert(PipelineHandle {
            config,
            pipeline
        }))
    }

    pub fn get(&self, key: DefaultKey) -> Option<ComputePipeline> {
        self.inner.lock().unwrap().pipelines.get(key).map(|p| p.pipeline.clone())
    }

    pub fn reload(&mut self, path: &PathBuf) {
        let mut inner = self.inner.lock().unwrap();
        let device = inner.device.clone();

        // Look through all shaders with the given path and recreate them
        for handle in inner.pipelines.iter_mut() {
            let config = &handle.1.config;
            if path.ends_with(&config.shader_path) {
                let pipeline = ComputePipeline::new(
                    &device,
                    config.shader_path.clone(),
                    &config.descriptor_set_layouts.as_slice(),
                    &config.push_constant_ranges.as_slice(),
                    &config.macros
                ).unwrap();
                handle.1.pipeline = pipeline;
            }
        }
    }
}