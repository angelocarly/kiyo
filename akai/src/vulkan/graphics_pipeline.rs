use std::ffi::CString;
use std::sync::Arc;
use ash::vk;
use ash::vk::{ShaderModule};
use crate::vulkan::{Device, RenderPass};

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

    pub fn new(device: Arc<Device>, render_pass: Arc<RenderPass>) -> GraphicsPipeline {

        // TODO: Make graphics shaders configurable

        // Shaders
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

        // Multisample
        let multisample_state_create_info = vk::PipelineMultisampleStateCreateInfo::default()
            .rasterization_samples(vk::SampleCountFlags::TYPE_1);

        // Viewport
        let viewports = [vk::Viewport::default()
            .width(512f32)
            .height(512f32)
            .x(0f32)
            .y(0f32)
        ];

        let scissors = [vk::Rect2D::default()
            .offset(vk::Offset2D::default())
            .extent(vk::Extent2D::default().width(512).height(512))
        ];

        let viewport_state_create_info = vk::PipelineViewportStateCreateInfo::default()
            .viewports(&viewports)
            .scissors(&scissors);

        // Vertex input
        let vertex_input_state_create_info = vk::PipelineVertexInputStateCreateInfo::default();

        // Input assembly
        let input_assembly_state_create_info = vk::PipelineInputAssemblyStateCreateInfo::default()
            .primitive_restart_enable(false)
            .topology(vk::PrimitiveTopology::TRIANGLE_LIST);

        // Rasterization
        let rasterization_state = vk::PipelineRasterizationStateCreateInfo::default()
            .polygon_mode(vk::PolygonMode::FILL)
            .cull_mode(vk::CullModeFlags::BACK)
            .front_face(vk::FrontFace::COUNTER_CLOCKWISE)
            .line_width(1.0);

        // Color blending
        let color_blend_attachment_state = vk::PipelineColorBlendAttachmentState::default()
            .blend_enable(false)
            .color_write_mask(vk::ColorComponentFlags::RGBA)
            .src_color_blend_factor(vk::BlendFactor::ONE)
            .dst_color_blend_factor(vk::BlendFactor::ZERO)
            .color_blend_op(vk::BlendOp::ADD)
            .src_alpha_blend_factor(vk::BlendFactor::ONE)
            .dst_alpha_blend_factor(vk::BlendFactor::ZERO)
            .alpha_blend_op(vk::BlendOp::ADD);
        let color_blend_attachment_states = [color_blend_attachment_state];

        let color_blend_state = vk::PipelineColorBlendStateCreateInfo::default()
            .logic_op_enable(false)
            .logic_op(vk::LogicOp::COPY)
            .blend_constants([0.0, 0.0, 0.0, 0.0])
            .attachments(&color_blend_attachment_states);

        // Layout
        let create_info = vk::PipelineLayoutCreateInfo::default();
        let pipeline_layout = unsafe {
            device.get_vk_device()
                .create_pipeline_layout(&create_info, None)
                .expect("Failed to create pipeline layout")
        };

        // pipeline
        let graphics_pipeline_create_info = vk::GraphicsPipelineCreateInfo::default()
            .stages(&shader_stages)
            .render_pass(render_pass.get_vk_render_pass())
            .multisample_state(&multisample_state_create_info)
            .viewport_state(&viewport_state_create_info)
            .vertex_input_state(&vertex_input_state_create_info)
            .input_assembly_state(&input_assembly_state_create_info)
            .color_blend_state(&color_blend_state)
            .rasterization_state(&rasterization_state)
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