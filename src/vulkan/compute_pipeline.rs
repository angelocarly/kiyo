use std::ffi::CString;
use std::sync::Arc;
use ash::vk;
use ash::vk::PushConstantRange;
use crate::vulkan::{DescriptorSetLayout, Device, Pipeline};
use crate::vulkan::device::DeviceInner;
use crate::vulkan::pipeline::{create_shader_module, load_from_file};

pub struct ComputePipelineInner {
    pub pipeline_layout: vk::PipelineLayout,
    pub compute_pipeline: vk::Pipeline,
    pub device_dep: Arc<DeviceInner>,
}

impl Drop for ComputePipelineInner {
    fn drop(&mut self) {
        unsafe {
            self.device_dep.device.destroy_pipeline(self.compute_pipeline, None);
            self.device_dep.device.destroy_pipeline_layout(self.pipeline_layout, None);
        }
    }
}

pub struct ComputePipeline {
    inner: Arc<ComputePipelineInner>
}

impl Pipeline for ComputePipeline {
    fn handle(&self) -> vk::Pipeline {
        self.inner.compute_pipeline
    }

    fn bind_point(&self) -> vk::PipelineBindPoint {
        vk::PipelineBindPoint::COMPUTE
    }

    fn layout(&self) -> vk::PipelineLayout {
        self.inner.pipeline_layout
    }
}

impl ComputePipeline {

pub fn new(device: &Device, shader_source: String, layouts: &[&DescriptorSetLayout], push_constant_ranges: &[PushConstantRange]) -> Self {

        let shader_code = load_from_file(shader_source);
        let shader_module = create_shader_module(device.handle(), shader_code.to_vec());

        let binding = CString::new("main").unwrap();
        let shader_stages = [
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlags::COMPUTE)
                .module(shader_module)
                .name(binding.as_c_str()),
        ];

        // Layout
        let desc_layouts = layouts
            .iter().map(|layout| layout.handle()).collect::<Vec<_>>();
        let create_info = vk::PipelineLayoutCreateInfo::default()
            .set_layouts(&*desc_layouts)
            .push_constant_ranges(&push_constant_ranges);
        let pipeline_layout = unsafe {
            device.handle()
                .create_pipeline_layout(&create_info, None)
                .expect("Failed to create pipeline layout")
        };

        // pipeline
        let compute_pipeline_create_info = vk::ComputePipelineCreateInfo::default()
            .stage(shader_stages[0])
            .layout(pipeline_layout);

        let compute_pipeline = unsafe {
            device.handle()
                .create_compute_pipelines(vk::PipelineCache::null(), &[compute_pipeline_create_info], None)
                .expect("Failed to create graphics pipeline")[0]
        };

        unsafe { device.handle().destroy_shader_module(shader_module, None); }

        let pipeline_inner = ComputePipelineInner {
            pipeline_layout,
            compute_pipeline,
            device_dep: device.inner.clone()
        };

        Self {
            inner: Arc::new(pipeline_inner)
        }
    }
}
