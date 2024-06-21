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
}

impl Game {
    pub fn new(renderer: &mut Renderer) -> Game{

        let descriptor_set_layout = DescriptorSetLayout::new_push_descriptor(
            &renderer.device
        );

        let image = Image::new(
            &renderer.device,
            &mut renderer.allocator,
            800,
            600
        );

        renderer.transition_image(&image);

        let compute_pipeline = ComputePipeline::new(
             &renderer.device,
             "examples/compute-pipeline/shaders/test_shader.comp".to_string(),
             &[&descriptor_set_layout],
        );

        let graphics_pipeline = GraphicsPipeline::new(
            &renderer.device,
            &renderer.render_pass,
            "examples/compute-pipeline/shaders/test_shader.vert".to_string(),
            "examples/compute-pipeline/shaders/test_shader.frag".to_string(),
            &[&descriptor_set_layout]
        );

        Game {
            image,
            compute_pipeline,
            graphics_pipeline,
            _descriptor_set_layout: descriptor_set_layout
        }
    }

}

impl GameHandler for Game {

    fn render(&mut self, render_context: &RenderContext) {

        render_context.command_buffer.clear_color_image(&self.image);

        render_context.command_buffer.bind_pipeline(&self.compute_pipeline);
        render_context.command_buffer.bind_push_descriptor_image(&self.compute_pipeline, &self.image);
        render_context.command_buffer.dispatch(100, 100, 1);

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
    let mut window = Window::create(&event_loop, "Akai engine", 800, 600);
    let mut renderer = Renderer::new(&window);
    let mut game = Game::new(&mut renderer);

    let app = Application::new();
    app.run(event_loop, &mut renderer, &mut window, &mut game);
}
