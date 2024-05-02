use std::ffi::CString;
use std::sync::Arc;
use ash::vk;
use ash::vk::ShaderModule;
use crate::vulkan::Device;

pub struct GraphicsPipeline {
    pub pipeline_layout: vk::PipelineLayout,
    pub graphics_pipeline: vk::Pipeline,
    pub device: Arc<Device>,
}

impl GraphicsPipeline {

    fn create_shader_module(device: &ash::Device, code: Vec<u8>) -> ShaderModule {
        let shader_module_create_info = vk::ShaderModuleCreateInfo::default()
            .code(unsafe { std::slice::from_raw_parts(code.as_ptr() as *const u32, code.len() / 4) });

        unsafe {
            device
                .create_shader_module(&shader_module_create_info, None)
                .expect("Failed to create shader module")
        }
    }

    pub fn new(device: Arc<Device>) -> GraphicsPipeline {

        // TODO: Make graphics shaders configurable

        let vertex_shader_code = include_bytes!("../../shaders/test_shader.vert.spv");
        let vertex_shader_module = Self::create_shader_module(device.get_vk_device(), vertex_shader_code.to_vec());

        let fragment_shader_code = include_bytes!("../../shaders/test_shader.frag.spv");
        let fragment_shader_module = Self::create_shader_module(device.get_vk_device(), fragment_shader_code.to_vec());

        let binding = CString::new("main").unwrap();
        let shader_stages = [
            // Vertex shader
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlags::VERTEX)
                .module(vertex_shader_module)
                .name(binding.as_c_str()),
            // Fragment shader
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlags::FRAGMENT)
                .module(fragment_shader_module)
                .name(binding.as_c_str())
        ];

        let create_info = vk::PipelineLayoutCreateInfo::default();
        let pipeline_layout = unsafe {
            device.get_vk_device()
                .create_pipeline_layout(&create_info, None)
                .expect("Failed to create pipeline layout")
        };

        let graphics_pipeline_create_info = vk::GraphicsPipelineCreateInfo::default()
            .stages(&shader_stages)
            .layout(pipeline_layout);

        let graphics_pipeline = unsafe {
            device.get_vk_device()
                .create_graphics_pipelines(vk::PipelineCache::null(), &[graphics_pipeline_create_info], None)
                .expect("Failed to create graphics pipeline")[0]
        };

        Self {
            device,
            pipeline_layout,
            graphics_pipeline
        }
    }
}

impl Drop for GraphicsPipeline {
    fn drop(&mut self) {
        unsafe {
            self.device.get_vk_device().destroy_pipeline(self.graphics_pipeline, None);
            self.device.get_vk_device().destroy_pipeline_layout(self.pipeline_layout, None);
        }
    }
}