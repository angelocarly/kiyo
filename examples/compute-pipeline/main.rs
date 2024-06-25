use std::time::SystemTime;
use ash::vk;
use akai::application::{Application, GameHandler, RenderContext};
use akai::vulkan::{ComputePipeline, DescriptorSetLayout, GraphicsPipeline, Image};
use winit::event_loop::EventLoop;
use akai::renderer::Renderer;
use akai::window::Window;

struct Game {
    image: Image,
    compute_pipeline: ComputePipeline,
    graphics_pipeline: GraphicsPipeline,
    _descriptor_set_layout: DescriptorSetLayout,
    start_time: SystemTime,
}

impl Game {
    pub fn new(renderer: &mut Renderer) -> Game{

        let layout_bindings = &[
            vk::DescriptorSetLayoutBinding::default()
                .binding(0)
                .descriptor_type(vk::DescriptorType::STORAGE_IMAGE)
                .descriptor_count(1)
                .stage_flags(vk::ShaderStageFlags::COMPUTE | vk::ShaderStageFlags::FRAGMENT)
        ];
        let descriptor_set_layout = DescriptorSetLayout::new_push_descriptor(
            &renderer.device,
            layout_bindings
        );

        let image = Image::new(
            &renderer.device,
            &mut renderer.allocator,
            1400,
            1400,
            vk::ImageUsageFlags::STORAGE | vk::ImageUsageFlags::TRANSFER_SRC | vk::ImageUsageFlags::TRANSFER_DST
        );

        renderer.transition_image(&image, vk::ImageLayout::UNDEFINED, vk::ImageLayout::GENERAL);

        let push_constant_ranges = [
            vk::PushConstantRange::default()
                .offset(0)
                .stage_flags(vk::ShaderStageFlags::COMPUTE)
                .size(4)
        ];

        let compute_pipeline = ComputePipeline::new(
             &renderer.device,
             "examples/compute-pipeline/shaders/test_shader.comp".to_string(),
             &[&descriptor_set_layout],
             &push_constant_ranges
        );

        let graphics_pipeline = GraphicsPipeline::new(
            &renderer.device,
            &renderer.render_pass,
            "examples/compute-pipeline/shaders/test_shader.vert".to_string(),
            "examples/compute-pipeline/shaders/test_shader.frag".to_string(),
            &[&descriptor_set_layout]
        );

        let start_time = std::time::SystemTime::now();

        Game {
            image,
            compute_pipeline,
            graphics_pipeline,
            _descriptor_set_layout: descriptor_set_layout,
            start_time
        }
    }

}

impl GameHandler for Game {

    fn render(&mut self, render_context: &RenderContext) {

        render_context.command_buffer.clear_color_image(&self.image);

        render_context.command_buffer.bind_pipeline(&self.compute_pipeline);
        render_context.command_buffer.bind_push_descriptor_image(&self.compute_pipeline, &self.image);
        let time = self.start_time.elapsed().unwrap().as_secs_f32();
        render_context.command_buffer.push_constants(&self.compute_pipeline, vk::ShaderStageFlags::COMPUTE, 0, &time.to_ne_bytes());
        render_context.command_buffer.dispatch(2, 2, 1);

        render_context.begin_root_render_pass();
        {
            render_context.command_buffer.bind_pipeline(&self.graphics_pipeline);
            render_context.command_buffer.bind_push_descriptor_image(&self.graphics_pipeline, &self.image);
            unsafe {
                render_context.device.handle()
                    .cmd_draw(
                        render_context.command_buffer.handle(),
                        3,
                        1,
                        0,
                        0
                    )
            };
        }
        render_context.command_buffer.end_render_pass();
    }
}

fn main() {
    let event_loop = EventLoop::new().expect("Failed to create event loop.");
    let mut window = Window::create(&event_loop, "Akai engine", 1400, 1400);
    let mut renderer = Renderer::new(&window);
    let mut game = Game::new(&mut renderer);

    let app = Application::new();
    app.run(event_loop, &mut renderer, &mut window, &mut game);
}
