use std::collections::HashMap;
use std::mem::size_of;
use std::process::Output;
use ash::vk;
use ash::vk::{ImageAspectFlags, ImageSubresourceLayers, Offset3D};
use bytemuck::{Pod, Zeroable};
use cen::graphics::pipeline_store::{PipelineConfig, PipelineKey};
use cen::graphics::Renderer;
use cen::graphics::renderer::RenderComponent;
use cen::vulkan::{CommandBuffer, DescriptorSetLayout, Image, PipelineErr};
use glam::{UVec2, UVec3};
use log::error;
use slotmap::DefaultKey;
use crate::app::audio_orch::{AudioConfig};
use crate::app::audio_orch::AudioConfig::AudioFile;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};
use core::time::{Duration};
use cen::app::gui::GuiComponent;
use egui::{menu, Context, TopBottomPanel, Vec2, Window};
use egui::WidgetType::TextEdit;

pub enum DispatchConfig
{
    Count( u32, u32, u32 ),
    FullScreen,
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct PushConstants {
    pub time: f32,
    pub in_image: i32,
    pub out_image: i32,
}

pub struct Pass {
    pub shader: String,
    pub dispatches: DispatchConfig,
    pub input_resources: Vec<u32>,
    pub output_resources: Vec<u32>,
}

#[derive(Clone)]
pub enum ClearConfig {
    None,
    Color(f32,f32,f32),
}

#[derive(Clone)]
pub struct ImageConfig {
    pub clear: ClearConfig,
}

pub struct DrawConfig {
    pub passes: Vec<Pass>,
    pub images: Vec<ImageConfig>,
}

pub struct ShaderPass {
    pub dispatches: glam::UVec3,
    pub in_images: Vec<u32>,
    pub out_images: Vec<u32>,
    pub pipeline_handle: PipelineKey,
}

pub struct ImageResource {
    pub image: Image,
    pub clear: ClearConfig,
}

struct ImgExport {
    width_text: String,
    height_text: String,
    filename: String,
    width: u32,
    height: u32,
}

/**
 *  Contains all render related structures relating to a config.
 */
pub struct DrawOrchestrator {
    draw_config: DrawConfig,
    audio_config: AudioConfig,
    audio_stream: Option<OutputStream>,
    sink: Option<Sink>,
    pub compute_descriptor_set_layout: Option<DescriptorSetLayout>,
    pub image_resources: Option<Vec<ImageResource>>,
    pub passes: Option<Vec<ShaderPass>>,
    image_export: ImgExport,
}

impl DrawOrchestrator {
    pub fn new(draw_config: DrawConfig, audio_config: AudioConfig) -> DrawOrchestrator {
        Self {
            draw_config,
            audio_config,
            audio_stream: None,
            sink: None,
            compute_descriptor_set_layout: None,
            image_resources: None,
            passes: None,
            image_export: ImgExport { filename: "".to_string(), width_text: "".to_string(), height_text: "".to_string(), width: 1024, height: 1024 },
        }
    }
}

impl GuiComponent for DrawOrchestrator {
    fn gui(&mut self, context: &Context) {
        TopBottomPanel::top("top").show(context, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("Export..", |ui| {
                    // ui.label("Width");
                    // ui.add(egui::TextEdit::singleline(&mut self.image_export.width_text));
                    // ui.label("Height");
                    // ui.add(egui::TextEdit::singleline(&mut self.image_export.height_text));
                    // ui.label("Filename");
                    // ui.add(egui::TextEdit::singleline(&mut self.image_export.filename));
                });
            });
        });
    }
}

impl RenderComponent for DrawOrchestrator {
    fn initialize(&mut self, renderer: &mut Renderer)
    {
        let image_count = self.draw_config.images.len() as u32;

        // Verify max referred index
        let max_reffered_image = self.draw_config.passes.iter()
            .map(|p| p.output_resources.iter())
            .flatten().max().unwrap_or(&0);
        if *max_reffered_image as i32 > image_count as i32 - 1 {
            error!("Image index out of bounds, provide enough image resources");
            panic!("Image index out of bounds, provide enough image resources");
        }

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
        let image_resources = self.draw_config.images.iter().map(|c| {
            let image = Image::new(
                &renderer.device,
                &mut renderer.allocator,
                renderer.swapchain.get_extent().width,
                renderer.swapchain.get_extent().height,
                vk::ImageUsageFlags::STORAGE | vk::ImageUsageFlags::TRANSFER_SRC | vk::ImageUsageFlags::TRANSFER_DST
            );

            ImageResource {
                image,
                clear: c.clear.clone(),
            }
        }).collect::<Vec<ImageResource>>();

        // Transition images
        let mut image_command_buffer = CommandBuffer::new(&renderer.device, &renderer.command_pool);
        image_command_buffer.begin();
        {
            for image_resource in &image_resources {
                renderer.transition_image(&image_command_buffer, &image_resource.image.handle(), vk::ImageLayout::UNDEFINED, vk::ImageLayout::GENERAL, vk::PipelineStageFlags::TOP_OF_PIPE, vk::PipelineStageFlags::BOTTOM_OF_PIPE, vk::AccessFlags::empty(), vk::AccessFlags::empty());
            }
        }
        image_command_buffer.end();
        renderer.device.submit_single_time_command(renderer.queue, &image_command_buffer);

        let push_constant_ranges = Vec::from([
            vk::PushConstantRange::default()
                .stage_flags(vk::ShaderStageFlags::COMPUTE)
                .offset(0)
                .size(size_of::<PushConstants>() as u32),
        ]);

        let workgroup_size = 32;
        let full_screen_dispatches = UVec3::new(
            (renderer.swapchain.get_extent().width as f32 / workgroup_size as f32).ceil() as u32,
            (renderer.swapchain.get_extent().height as f32 / workgroup_size as f32).ceil() as u32,
            1
        );

        let mut macros: HashMap<String, String> = HashMap::new();
        macros.insert("NUM_IMAGES".to_string(), image_count.to_string());
        macros.insert("WORKGROUP_SIZE".to_string(), workgroup_size.to_string());

        // Passes
        let passes = self.draw_config.passes
            .iter()
            .map(|c| {
                let pipeline_handle = renderer.pipeline_store().insert(
                    PipelineConfig {
                        shader_path: c.shader.clone().into(),
                        descriptor_set_layouts: vec![compute_descriptor_set_layout.clone()],
                        push_constant_ranges: push_constant_ranges.clone(),
                        macros: macros.clone()
                    }
                )?;

                let dispatches = match c.dispatches {
                    DispatchConfig::Count(x, y, z) => {
                        UVec3::new(x, y, z)
                    }
                    DispatchConfig::FullScreen => {
                        full_screen_dispatches
                    }
                };

                Ok(ShaderPass {
                    pipeline_handle,
                    dispatches: dispatches,
                    in_images: c.input_resources.clone(),
                    out_images: c.output_resources.clone(),
                })
            })
            .collect::<Result<Vec<ShaderPass>, PipelineErr>>()
            .expect("Failed to create shader pass");

        self.compute_descriptor_set_layout = Some(compute_descriptor_set_layout);
        self.image_resources = Some(image_resources);
        self.passes = Some(passes);

        // Audio things
        if let AudioFile(file) = self.audio_config.clone() {
            let (stream, stream_handle) = OutputStream::try_default().unwrap();
            self.audio_stream = Some(stream);
            self.sink = Some(Sink::try_new(&stream_handle).unwrap());
            // Load a sound from a file, using a path relative to Cargo.toml
            let file = BufReader::new(File::open(file).unwrap());
            // Decode that sound file into a source
            let source = Decoder::new(file).unwrap();

            self.sink.as_ref().map(|sink| Sink::append(sink, source));
            self.sink.as_ref().map(|sink| Sink::play(sink));
        };
    }

    fn render(&mut self, renderer: &mut Renderer, command_buffer: &mut CommandBuffer, swapchain_image: &vk::Image, view: &vk::ImageView) {

        for i in self.image_resources.as_ref().unwrap() {
            renderer.transition_image(
                &command_buffer,
                &i.image.handle(),
                vk::ImageLayout::GENERAL,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                vk::PipelineStageFlags::TOP_OF_PIPE,
                vk::PipelineStageFlags::TRANSFER,
                vk::AccessFlags::NONE,
                vk::AccessFlags::TRANSFER_WRITE
            );

            match &i.clear {
                ClearConfig::None => {},
                ClearConfig::Color(r,g,b) => {
                    unsafe {
                        renderer.device.handle()
                            .cmd_clear_color_image(
                                command_buffer.handle(),
                                *i.image.handle(),
                                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                                &vk::ClearColorValue {
                                    float32: [*r, *g, *b, 1f32]
                                },
                                &[vk::ImageSubresourceRange {
                                    aspect_mask: ImageAspectFlags::COLOR,
                                    base_mip_level: 0,
                                    level_count: 1,
                                    base_array_layer: 0,
                                    layer_count: 1,
                                }]
                            );
                    }
                }
            }

            renderer.transition_image(
                &command_buffer,
                &i.image.handle(),
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                vk::ImageLayout::GENERAL,
                vk::PipelineStageFlags::TRANSFER,
                vk::PipelineStageFlags::COMPUTE_SHADER,
                vk::AccessFlags::TRANSFER_WRITE,
                vk::AccessFlags::SHADER_WRITE
            );
        }

        // Compute images
        let current_time = renderer.start_time.elapsed().as_secs_f32();
        for p in self.passes.as_ref().unwrap() {
            if let Some(pipeline) = renderer.pipeline_store().get(p.pipeline_handle) {
                command_buffer.bind_pipeline(&pipeline);
                let push_constants = PushConstants {
                    time: current_time,
                    in_image: p.in_images.first().map(|&x| x as i32).unwrap_or(-1),
                    out_image: p.out_images.first().map(|&x| x as i32).unwrap_or(-1),
                };
                command_buffer.push_constants(&pipeline, vk::ShaderStageFlags::COMPUTE, 0, &bytemuck::cast_slice(std::slice::from_ref(&push_constants)));
                command_buffer.bind_push_descriptor_images(
                    &pipeline,
                    &self.image_resources.as_ref().unwrap().iter().map(|r| {
                        &r.image
                    }).collect::<Vec<&Image>>()
                );
                command_buffer.dispatch(p.dispatches.x, p.dispatches.y, p.dispatches.z);
            }

            // TODO: Add synchronization between passes
        };

        self.sink.as_ref().map(|sink| {
            let seekhead = sink.get_pos();
            let render_time = renderer.start_time.elapsed();

            if seekhead.abs_diff(render_time) > Duration::from_secs_f32(0.05) {
                _ = Sink::try_seek(sink, render_time);
            }
        });

        // Copy to swapchain

        let output_image = &self.image_resources.as_ref().unwrap().last().expect("No images found to output").image;

        renderer.transition_image(
            &command_buffer,
            &output_image.handle(),
            vk::ImageLayout::GENERAL,
            vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
            vk::PipelineStageFlags::TRANSFER,
            vk::PipelineStageFlags::TRANSFER,
            vk::AccessFlags::TRANSFER_WRITE,
            vk::AccessFlags::TRANSFER_READ
        );


        // Transition the swapchain image
        renderer.transition_image(
            &command_buffer,
            &swapchain_image,
            vk::ImageLayout::PRESENT_SRC_KHR,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            vk::PipelineStageFlags::TOP_OF_PIPE,
            vk::PipelineStageFlags::TRANSFER,
            vk::AccessFlags::NONE,
            vk::AccessFlags::TRANSFER_WRITE
        );

        unsafe {

            renderer.device.handle().cmd_clear_color_image(
                command_buffer.handle(),
                *swapchain_image,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                &vk::ClearColorValue {
                    float32: [0.0, 0.0, 0.0, 1.0]
                },
                &[vk::ImageSubresourceRange {
                    aspect_mask: ImageAspectFlags::COLOR,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,
                }]
            );

            // Use a blit, as a copy doesn't synchronize properly to the swapchain on MoltenVK
            renderer.device.handle().cmd_blit_image(
                command_buffer.handle(),
                *output_image.handle(),
                vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                *swapchain_image,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                &[vk::ImageBlit::default()
                    .src_offsets([
                        Offset3D::default(),
                        Offset3D::default().x(output_image.width as i32).y(output_image.height as i32).z(1)
                    ])
                    .dst_offsets([
                        Offset3D::default(),
                        Offset3D::default().x(output_image.width as i32).y(output_image.height as i32).z(1)
                    ])
                    .src_subresource(
                        ImageSubresourceLayers::default()
                            .aspect_mask(ImageAspectFlags::COLOR)
                            .base_array_layer(0)
                            .layer_count(1)
                            .mip_level(0)
                    )
                    .dst_subresource(
                        ImageSubresourceLayers::default()
                            .aspect_mask(ImageAspectFlags::COLOR)
                            .base_array_layer(0)
                            .layer_count(1)
                            .mip_level(0)
                    )
                ],
                vk::Filter::NEAREST,
            );
        }

        // Transfer back to default states
        renderer.transition_image(
            &command_buffer,
            &swapchain_image,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            vk::ImageLayout::PRESENT_SRC_KHR,
            vk::PipelineStageFlags::TRANSFER,
            vk::PipelineStageFlags::BOTTOM_OF_PIPE,
            vk::AccessFlags::TRANSFER_WRITE,
            vk::AccessFlags::NONE
        );

        renderer.transition_image(
            &command_buffer,
            output_image.handle(),
            vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
            vk::ImageLayout::GENERAL,
            vk::PipelineStageFlags::TRANSFER,
            vk::PipelineStageFlags::BOTTOM_OF_PIPE,
            vk::AccessFlags::TRANSFER_READ,
            vk::AccessFlags::NONE
        );
    }
}
