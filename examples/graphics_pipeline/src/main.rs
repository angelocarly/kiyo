use akai::application::{Application, GameHandler, RenderContext};
use akai::vulkan::GraphicsPipeline;
use winit::event_loop::EventLoop;
use akai::renderer::Renderer;
use akai::window::Window;

struct Game {
    graphics_pipeline: GraphicsPipeline,
}

impl Game {
    pub fn new(renderer: &Renderer) -> Game{

        Game {
            graphics_pipeline: GraphicsPipeline::new(
                &renderer.device,
                &renderer.render_pass,
                "examples/graphics_pipeline/shaders/test_shader.vert".to_string(),
                "examples/graphics_pipeline/shaders/test_shader.frag".to_string(),
                &[]
            )
        }
    }
}

impl GameHandler for Game {

    fn render(&mut self, render_context: &RenderContext) {

        render_context.begin_root_render_pass();
        {
            render_context.command_buffer.bind_pipeline(&self.graphics_pipeline);
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
    let mut game = Game::new(&renderer);

    let app = Application::new();
    app.run(event_loop, &mut renderer, &mut window, &mut game);
}
