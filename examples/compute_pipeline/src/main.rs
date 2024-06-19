use akai::application::{Application, GameHandler, RenderContext};
use akai::vulkan::{ComputePipeline, DescriptorSetLayout, Image};
use winit::event_loop::EventLoop;
use akai::renderer::Renderer;
use akai::window::Window;

struct Game {
    image: Image,
    compute_pipeline: ComputePipeline,
    descriptor_set_layout: DescriptorSetLayout,
}

impl Game {
    pub fn new(renderer: &mut Renderer) -> Game{

        let descriptor_set_layout = DescriptorSetLayout::new(
            &renderer.device
        );

        let image = Image::new(
            &renderer.device,
            &mut renderer.allocator,
            800,
            600
        );

       let compute_pipeline = ComputePipeline::new(
            &renderer.device,
            "examples/compute_pipeline/shaders/test_shader.comp".to_string(),
            &[&descriptor_set_layout],
        );

        Game {
            image,
            descriptor_set_layout,
            compute_pipeline,
        }
    }
}

impl GameHandler for Game {

    fn render(&mut self, render_context: &RenderContext) {

        render_context.command_buffer.bind_pipeline(&self.compute_pipeline);
        render_context.command_buffer.bind_push_descriptor_image(&self.compute_pipeline, &self.descriptor_set_layout, &self.image);
        render_context.command_buffer.dispatch(1, 1, 1);

        render_context.begin_root_render_pass();
        {
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
