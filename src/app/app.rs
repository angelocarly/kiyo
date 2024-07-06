use std::time::SystemTime;
use winit::event::{Event, StartCause, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::run_on_demand::EventLoopExtRunOnDemand;
use crate::app::DrawOrchestrator;
use crate::renderer::Renderer;
use crate::window::Window;

// Maybe delete all the following blocks
use crate::vulkan::{Device, RenderPass, Framebuffer, CommandBuffer};

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
// Stop delete

pub struct App {
    _start_time: SystemTime,
    renderer: Renderer,
    window: Window,
    event_loop: EventLoop<()>,
}

impl App {
    pub fn new() -> App{

        let start_time = std::time::SystemTime::now();

        let event_loop = EventLoop::new().expect("Failed to create event loop.");
        let window = Window::create(&event_loop, "Akai engine", 1000, 1000);
        let renderer = Renderer::new(&window);

        App {
            event_loop,
            window,
            renderer,
            _start_time: start_time,
        }
    }

    pub fn run(mut self, draw_orchestrator: &mut dyn DrawOrchestrator) {
        self.event_loop
            .run_on_demand( |event, elwt| {
                elwt.set_control_flow(ControlFlow::Poll);

                match event {
                    | Event::NewEvents(StartCause::Poll) => {
                        self.renderer.draw_frame(draw_orchestrator);
                    }
                    | Event::WindowEvent { event, .. } => {
                        self.window.window_event( event.clone(), elwt );

                        match event {
                            WindowEvent::RedrawRequested => {
                                self.renderer.draw_frame(draw_orchestrator);
                            },
                            _ => (),
                        }
                    }
                    _ => (),
                }

            })
            .unwrap();

        // Wait for all render operations to finish before exiting
        // This ensures we can safely start dropping gpu resources
        self.renderer.device.wait_idle();
    }
}