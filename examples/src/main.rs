use std::sync::Arc;
use akai::application::{Application, GameHandler, GraphicsContext, RenderContext};
use akai::vulkan::GraphicsPipeline;
use winit::event_loop::EventLoop;

struct Game {
    // TODO: Strict ordering is required for the pipeline to be destroyed before the instance
    //       So maybe context should be a weak reference?
    graphics_pipeline: Arc<GraphicsPipeline>,
    graphics_context: Arc<GraphicsContext>,
}

impl Game {
    pub fn new(graphics_context: Arc<GraphicsContext>) -> Game {

        let vertex_shader_code = include_bytes!("../shaders/test_shader.vert.spv");
        let fragment_shader_code = include_bytes!("../shaders/test_shader.frag.spv");

        Game {
            graphics_context: graphics_context.clone(),
            graphics_pipeline: Arc::new(GraphicsPipeline::new(
                graphics_context.clone().device.clone(),
                graphics_context.clone().render_pass.clone(),
                vertex_shader_code.to_vec(),
                fragment_shader_code.to_vec(),
            ))
        }
    }
}

impl GameHandler for Game {

    fn render(&mut self, render_context: &RenderContext) {

        render_context.begin_root_render_pass();
        {
            render_context.command_buffer.bind_pipeline(self.graphics_pipeline.clone());
            unsafe {
                self.graphics_context.device.get_vk_device()
                    .cmd_draw(
                        render_context.command_buffer.get_vk_command_buffer(),
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
    let mut app = Application::new(&event_loop, "Akai engine", 800, 600);
    let mut game = Game::new(app.get_graphics_context());
    app.run(event_loop, &mut game);
}
