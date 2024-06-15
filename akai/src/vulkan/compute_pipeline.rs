use std::ffi::CString;
use std::fs;
use std::sync::Arc;
use ash::vk;
use ash::vk::{ShaderModule};
use crate::vulkan::{Device, Pipeline};
use crate::vulkan::device::DeviceInner;

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
}

impl ComputePipeline {

    fn create_shader_module(device: &ash::Device, code: Vec<u32>) -> ShaderModule {
        let shader_module_create_info = vk::ShaderModuleCreateInfo::default()
            .code(unsafe { std::slice::from_raw_parts(code.as_ptr(), code.len()) });

        unsafe {
            device
                .create_shader_module(&shader_module_create_info, None)
                .expect("Failed to create shader module")
        }
    }

    fn load_from_file(source_file: String) -> Vec<u32>
    {
        use shaderc;

        let shader_kind = match source_file.split(".").last() {
            Some("vert") => shaderc::ShaderKind::Vertex,
            Some("frag") => shaderc::ShaderKind::Fragment,
            Some("comp") => shaderc::ShaderKind::Compute,
            _ => panic!("Unknown shader type")
        };

        let source = fs::read_to_string(source_file.clone()).expect(format!("Failed to read file: {}", source_file).as_str());

        let compiler = shaderc::Compiler::new().unwrap();
        let mut options = shaderc::CompileOptions::new().unwrap();
        options.add_macro_definition("EP", Some("main"));
        let binary_result = compiler.compile_into_spirv(
            source.as_str(),
            shader_kind,
            source_file.as_str(),
            "main",
            Some(&options)
        ).unwrap();

        binary_result.as_binary().to_vec()
    }

    pub fn new(device: &Device, shader_source: String) -> Self {

        let shader_code = Self::load_from_file(shader_source);
        let shader_module = Self::create_shader_module(device.handle(), shader_code.to_vec());

        let binding = CString::new("main").unwrap();
        let shader_stages = [
            // Vertex shader
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlags::COMPUTE)
                .module(shader_module)
                .name(binding.as_c_str()),
        ];

        // Layout
        let create_info = vk::PipelineLayoutCreateInfo::default();
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
