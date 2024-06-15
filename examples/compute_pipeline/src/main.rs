use akai::application::{Application, GameHandler, RenderContext};
use akai::vulkan::{ComputePipeline};
use winit::event_loop::EventLoop;
use akai::renderer::Renderer;
use akai::window::Window;

struct Game {
    compute_pipeline: ComputePipeline,
}

impl Game {
    pub fn new(renderer: &Renderer) -> Game{

        Game {
            compute_pipeline: ComputePipeline::new(
                &renderer.device,
                "examples/compute_pipeline/shaders/test_shader.comp".to_string(),
            )
        }
    }
}

impl GameHandler for Game {

    fn render(&mut self, render_context: &RenderContext) {

        render_context.command_buffer.bind_pipeline(&self.compute_pipeline);
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
    let mut game = Game::new(&renderer);

    let app = Application::new();
    app.run(event_loop, &mut renderer, &mut window, &mut game);
}
