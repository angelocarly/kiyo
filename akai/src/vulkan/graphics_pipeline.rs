use std::ffi::CString;
use std::fs;
use std::sync::Arc;
use ash::vk;
use ash::vk::{ShaderModule};
use crate::vulkan::{Device, Pipeline, RenderPass};
use crate::vulkan::device::DeviceInner;

pub struct GraphicsPipelineInner {
    pub pipeline_layout: vk::PipelineLayout,
    pub graphics_pipeline: vk::Pipeline,
    pub device_dep: Arc<DeviceInner>,
}

impl Drop for GraphicsPipelineInner {
    fn drop(&mut self) {
        unsafe {
            self.device_dep.device.destroy_pipeline(self.graphics_pipeline, None);
            self.device_dep.device.destroy_pipeline_layout(self.pipeline_layout, None);
        }
    }
}

pub struct GraphicsPipeline {
    inner: Arc<GraphicsPipelineInner>
}

impl Pipeline for GraphicsPipeline {
    fn handle(&self) -> vk::Pipeline {
        self.inner.graphics_pipeline
    }

    fn bind_point(&self) -> vk::PipelineBindPoint {
        vk::PipelineBindPoint::GRAPHICS
    }
}

impl GraphicsPipeline {

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

    pub fn new(device: &Device, render_pass: &RenderPass, vertex_shader_source: String, fragment_shader_source: String) -> Self {

        let vertex_shader_code = Self::load_from_file(vertex_shader_source);
        let fragment_shader_code = Self::load_from_file(fragment_shader_source);

        // Shaders
        let vertex_shader_module = Self::create_shader_module(device.handle(), vertex_shader_code.to_vec());
        let fragment_shader_module = Self::create_shader_module(device.handle(), fragment_shader_code.to_vec());

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
            .front_face(vk::FrontFace::CLOCKWISE)
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

        // Depth stencil
        let depth_stencil_state_create_info = vk::PipelineDepthStencilStateCreateInfo::default()
            .depth_test_enable(false)
            .depth_write_enable(false)
            .depth_compare_op(vk::CompareOp::ALWAYS)
            .depth_bounds_test_enable(false)
            .stencil_test_enable(false);

        let dynamic_state_create_info = vk::PipelineDynamicStateCreateInfo::default()
            .dynamic_states(&[vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR]);

        // Layout
        let create_info = vk::PipelineLayoutCreateInfo::default();
        let pipeline_layout = unsafe {
            device.handle()
                .create_pipeline_layout(&create_info, None)
                .expect("Failed to create pipeline layout")
        };

        // pipeline
        let graphics_pipeline_create_info = vk::GraphicsPipelineCreateInfo::default()
            .stages(&shader_stages)
            .render_pass(render_pass.handle())
            .multisample_state(&multisample_state_create_info)
            .viewport_state(&viewport_state_create_info)
            .vertex_input_state(&vertex_input_state_create_info)
            .input_assembly_state(&input_assembly_state_create_info)
            .color_blend_state(&color_blend_state)
            .rasterization_state(&rasterization_state)
            .depth_stencil_state(&depth_stencil_state_create_info)
            .dynamic_state(&dynamic_state_create_info)
            .layout(pipeline_layout);

        let graphics_pipeline = unsafe {
            device.handle()
                .create_graphics_pipelines(vk::PipelineCache::null(), &[graphics_pipeline_create_info], None)
                .expect("Failed to create graphics pipeline")[0]
        };

        unsafe { device.handle().destroy_shader_module(fragment_shader_module, None); }
        unsafe { device.handle().destroy_shader_module(vertex_shader_module, None); }

        let pipeline_inner = GraphicsPipelineInner {
            pipeline_layout,
            graphics_pipeline,
            device_dep: device.inner.clone()
        };

        Self {
            inner: Arc::new(pipeline_inner)
        }
    }
}
