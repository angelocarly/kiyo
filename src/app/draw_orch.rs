use std::sync::Arc;
use ash::vk;
use glam::UVec2;
use crate::app::Renderer;
use crate::vulkan::{CommandBuffer, ComputePipeline, DescriptorSetLayout, Image};

pub struct Pass {
    pub shader: String,
    pub dispatches: glam::UVec3,
}

pub struct DrawConfig {
    pub passes: Vec<Pass>,
    pub resolution: UVec2,
}

impl DrawConfig {
    pub fn new() -> DrawConfig {
        DrawConfig {
            passes: Vec::new(),
            resolution: UVec2::new(1000, 1000)
        }
    }
}

pub struct ShaderPass {
    pub compute_pipeline: ComputePipeline,
    pub dispatches: glam::UVec3,
}

pub struct DrawOrchestrator {
    pub compute_descriptor_set_layout: DescriptorSetLayout,
    pub image: Image,
    pub passes: Vec<ShaderPass>,
}

impl DrawOrchestrator {
    pub fn new(renderer: &mut Renderer, draw_config: DrawConfig) -> DrawOrchestrator {

        // Layout
        let layout_bindings = &[
            vk::DescriptorSetLayoutBinding::default()
                .binding(0)
                .descriptor_type(vk::DescriptorType::STORAGE_IMAGE)
                .descriptor_count(1)
                .stage_flags(vk::ShaderStageFlags::COMPUTE | vk::ShaderStageFlags::FRAGMENT)
        ];
        let compute_descriptor_set_layout = DescriptorSetLayout::new_push_descriptor(
            &renderer.device,
            layout_bindings
        );

        // Image
        let image = Image::new(
            &renderer.device,
            &mut renderer.allocator,
            draw_config.resolution.x,
            draw_config.resolution.y,
            vk::ImageUsageFlags::STORAGE | vk::ImageUsageFlags::TRANSFER_SRC | vk::ImageUsageFlags::TRANSFER_DST
        );

        // Transition image
        let image_command_buffer = Arc::new(CommandBuffer::new(&renderer.device, &renderer.command_pool));
        image_command_buffer.begin();
        {
            renderer.transition_image(&image_command_buffer, &image.handle(), vk::ImageLayout::UNDEFINED, vk::ImageLayout::GENERAL, vk::PipelineStageFlags::TOP_OF_PIPE, vk::PipelineStageFlags::BOTTOM_OF_PIPE, vk::AccessFlags::empty(), vk::AccessFlags::empty());
        }
        image_command_buffer.end();
        renderer.device.submit_single_time_command(renderer.queue, image_command_buffer);

        // Passes
        let passes = draw_config.passes
            .iter()
            .map(|c| {
                let compute_pipeline = ComputePipeline::new(
                    &renderer.device,
                    c.shader.to_string(),
                    &[&compute_descriptor_set_layout],
                    &[]
                );
                ShaderPass {
                    compute_pipeline,
                    dispatches: c.dispatches,
                }
            })
            .collect();

        DrawOrchestrator {
            compute_descriptor_set_layout,
            image,
            passes
        }
    }
}