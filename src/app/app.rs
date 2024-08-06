use std::time::SystemTime;
use glam::UVec2;
use winit::event::{Event, StartCause, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::run_on_demand::EventLoopExtRunOnDemand;
use crate::app::draw_orch::DrawConfig;
use crate::app::{DrawOrchestrator, Renderer, Window};

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
    pub fn new(width: u32, height: u32) -> App{

        let start_time = SystemTime::now();

        let event_loop = EventLoop::new().expect("Failed to create event loop.");
        let window = Window::create(&event_loop, "kiyo engine", width, height);
        let renderer = Renderer::new(&window);

        App {
            event_loop,
            window,
            renderer,
            _start_time: start_time,
        }
    }

    pub fn run(mut self, draw_config: DrawConfig) {

        let resolution = UVec2::new( self.window.get_extent().width, self.window.get_extent().height );
        let mut orchestrator = DrawOrchestrator::new(&mut self.renderer, resolution, draw_config);

        self.event_loop
            .run_on_demand( |event, elwt| {
                elwt.set_control_flow(ControlFlow::Poll);

                match event {
                    | Event::NewEvents(StartCause::Poll) => {
                        self.renderer.draw_frame(&mut orchestrator);
                    }
                    | Event::WindowEvent { event, .. } => {
                        self.window.window_event( event.clone(), elwt );

                        match event {
                            WindowEvent::RedrawRequested => {
                                self.renderer.draw_frame(&mut orchestrator);
                            },
                            WindowEvent::Resized( _ ) => {
                            }
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