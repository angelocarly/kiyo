use crate::vulkan::PipelineErr;
use std::collections::HashMap;
use std::mem::size_of;
use std::sync::Arc;
use ash::vk;
use glam::{UVec2, UVec3};
use slotmap::{DefaultKey, SlotMap};
use crate::app::{Renderer};
use crate::app::renderer::PushConstants;
use crate::vulkan::{CommandBuffer, ComputePipeline, DescriptorSetLayout, Image};

#[derive(Clone)]
pub struct ImageResource {
    pub id: u32,
}

pub enum DispatchConfig
{
    Count( u32, u32, u32 ),
    FullScreen,
}

pub struct Pass {
    pub shader: String,
    pub dispatches: DispatchConfig,
    pub input_resources: Vec<u32>,
    pub output_resources: Vec<u32>,
}

pub struct DrawConfig {
    pub passes: Vec<Pass>,
}

impl DrawConfig {
    pub fn new() -> DrawConfig {
        DrawConfig {
            passes: Vec::new(),
        }
    }
}

pub struct ShaderPass {
    pub dispatches: glam::UVec3,
    pub in_images: Vec<u32>,
    pub out_images: Vec<u32>,
    pub compute_handle: DefaultKey,
}

/**
 *  Contains all render related structures relating to a config.
 */
pub struct DrawOrchestrator {
    pub compute_descriptor_set_layout: DescriptorSetLayout,
    pub images: Vec<Image>,
    pub passes: Vec<ShaderPass>,
    pub pipelines: SlotMap<DefaultKey, ComputePipeline>,
}

impl DrawOrchestrator {
    pub fn new(renderer: &mut Renderer, resolution: UVec2, draw_config: &DrawConfig) -> Result<DrawOrchestrator, PipelineErr> {

        let image_count = draw_config.passes.iter()
            .map(|p| p.output_resources.iter())
            .flatten().max().unwrap() + 1;

        // Layout
        let layout_bindings = &[
            vk::DescriptorSetLayoutBinding::default()
                .binding(0)
                .descriptor_type(vk::DescriptorType::STORAGE_IMAGE)
                .descriptor_count(image_count)
                .stage_flags(vk::ShaderStageFlags::COMPUTE | vk::ShaderStageFlags::FRAGMENT)
        ];
        let compute_descriptor_set_layout = DescriptorSetLayout::new_push_descriptor(
            &renderer.device,
            layout_bindings
        );

        // Images
        let images = (0..image_count).map(|_| {
            Image::new(
                &renderer.device,
                &mut renderer.allocator,
                resolution.x,
                resolution.y,
                vk::ImageUsageFlags::STORAGE | vk::ImageUsageFlags::TRANSFER_SRC | vk::ImageUsageFlags::TRANSFER_DST
            )
        }).collect::<Vec<Image>>();

        // Transition images
        let image_command_buffer = Arc::new(CommandBuffer::new(&renderer.device, &renderer.command_pool));
        image_command_buffer.begin();
        {
            for image in &images {
                renderer.transition_image(&image_command_buffer, &image.handle(), vk::ImageLayout::UNDEFINED, vk::ImageLayout::GENERAL, vk::PipelineStageFlags::TOP_OF_PIPE, vk::PipelineStageFlags::BOTTOM_OF_PIPE, vk::AccessFlags::empty(), vk::AccessFlags::empty());
            }
        }
        image_command_buffer.end();
        renderer.device.submit_single_time_command(renderer.queue, image_command_buffer);

        let push_constant_ranges = &[
            vk::PushConstantRange::default()
                .stage_flags(vk::ShaderStageFlags::COMPUTE)
                .offset(0)
                .size(size_of::<PushConstants>() as u32),
        ];

        let workgroup_size = 32;
        let full_screen_dispatches = UVec3::new(
            (resolution.x as f32 / workgroup_size as f32).ceil() as u32,
            (resolution.y as f32 / workgroup_size as f32).ceil() as u32,
            1
        );

        let mut macros: HashMap<&str, &dyn ToString> = HashMap::new();
        macros.insert("NUM_IMAGES", &image_count);
        macros.insert("WORKGROUP_SIZE", &workgroup_size);

        // Load pipelines in map
        let mut pipelines = SlotMap::new();

        // Passes
        let passes = draw_config.passes
            .iter()
            .map(|c| {
                let compute_pipeline = ComputePipeline::new(
                    &renderer.device,
                    c.shader.to_string(),
                    &[&compute_descriptor_set_layout],
                    push_constant_ranges,
                    &macros
                )?;

                let dispatches = match c.dispatches {
                    DispatchConfig::Count(x, y, z) => {
                        UVec3::new(x, y, z)
                    }
                    DispatchConfig::FullScreen => {
                        full_screen_dispatches
                    }
                };

                let compute_handle = pipelines.insert(compute_pipeline);

                Ok(ShaderPass {
                    compute_handle,
                    dispatches: dispatches,
                    in_images: c.input_resources.clone(),
                    out_images: c.output_resources.clone(),
                })
            })
            .collect::<Result<Vec<ShaderPass>, PipelineErr>>()?;

        Ok(DrawOrchestrator {
            pipelines,
            compute_descriptor_set_layout,
            images,
            passes
        })
    }
}