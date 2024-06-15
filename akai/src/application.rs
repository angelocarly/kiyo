use winit::event::{Event, StartCause, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::run_on_demand::EventLoopExtRunOnDemand;
use crate::renderer::Renderer;
use crate::vulkan::{Device, RenderPass, Framebuffer, CommandBuffer};
use crate::window::Window;

pub trait GameHandler {
    fn render(&mut self, render_context: &RenderContext);
}

pub struct RenderContext<'a> {
    pub device: &'a Device,
    pub(crate) render_pass: &'a RenderPass,
    pub(crate) framebuffer: &'a Framebuffer,
    pub command_buffer: &'a CommandBuffer,
}

impl RenderContext<'_> {
    pub fn begin_root_render_pass(&self) {
        self.command_buffer.begin_render_pass(
            &self.render_pass,
            &self.framebuffer
        );
    }
}

/// Generative art runtime.
/// Manages the window and graphics recording.
pub struct Application {
}

impl Application {

    pub fn new() -> Application {
        Application {
        }
    }

    pub fn run(self, mut event_loop: EventLoop<()>, renderer: &mut Renderer, window: &mut Window, game_handler: &mut dyn GameHandler) {
        event_loop
            .run_on_demand( |event, elwt| {
                elwt.set_control_flow(ControlFlow::Poll);

                match event {
                    | Event::NewEvents(StartCause::Poll) => {
                        renderer.draw_frame(game_handler);
                    }
                    | Event::WindowEvent { event, .. } => {
                        window.window_event( event.clone(), elwt );

                        match event {
                            WindowEvent::RedrawRequested => {
                                renderer.draw_frame(game_handler);
                            },
                            _ => (),
                        }
                    }
                    _ => (),
                }

            })
            .unwrap();

        unsafe { renderer.device.handle().device_wait_idle().unwrap(); }
    }
}

