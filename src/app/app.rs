use notify::event::AccessKind::Close;
use notify::EventKind::{Access, Modify};
use std::path::Path;
use std::time::SystemTime;
use env_logger::{Builder, Env};
use glam::UVec2;
use log::{error, info, LevelFilter};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use notify::event::AccessMode::Write;
use winit::event::{Event, StartCause, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop, EventLoopBuilder};
use winit::platform::run_on_demand::EventLoopExtRunOnDemand;
use cpal::traits::StreamTrait;
use crate::app::{DrawOrchestrator, Window};
use crate::graphics::Renderer;

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
    event_loop: EventLoop<UserEvent>,
    pub app_config: AppConfig,
}

pub struct AppConfig {
    pub width: u32,
    pub height: u32,
    pub vsync: bool,
    pub log_fps: bool,
}

pub enum UserEvent {
    Test
}

impl App {

    fn init_logger() {
        let env = Env::default()
            .filter_or("LOG_LEVEL", "trace")
            .write_style_or("LOG_STYLE", "always");

        Builder::from_env(env)
            .format_level(true)
            .format_timestamp_millis()
            .filter(Some("winit"), LevelFilter::Error)
            .filter(Some("calloop"), LevelFilter::Error)
            .filter(Some("notify::inotify"), LevelFilter::Error)
            .filter(Some("mio::poll"), LevelFilter::Error)
            .filter(Some("sctk"), LevelFilter::Error)
            .init();
    }

    pub fn new(app_config: AppConfig) -> App{

        Self::init_logger();

        // App setup
        let start_time = SystemTime::now();

        let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build().expect("Failed to create event loop.");
        let window = Window::create(&event_loop, "kiyo engine", app_config.width, app_config.height);
        let renderer = Renderer::new(&window, event_loop.create_proxy(), app_config.vsync);

        App {
            event_loop,
            window,
            renderer,
            _start_time: start_time,
            app_config,
        }
    }

    pub fn run(mut self, draw_config: DrawConfig, audio_func: Option<fn(f32)->(f32, f32)>) {

        let resolution = UVec2::new( self.window.get_extent().width, self.window.get_extent().height );

        let mut orchestrator = match DrawOrchestrator::new(&mut self.renderer, resolution, &draw_config) {
            Ok(d) => {
                d
            },
            Err(e) => {
                error!("{}", e);
                info!("A shader contains an error, quitting");
                std::process::abort();
            }
        };

        let paths = &draw_config.passes.iter().map(|p| { p.shader.clone() }).collect::<Vec<String>>();

        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher = RecommendedWatcher::new(tx, Config::default()).unwrap();
        for path in paths {
            watcher.watch(Path::new(path), RecursiveMode::Recursive).unwrap();
        };

        // audio

        if let Some(audio_func) = audio_func {

            let sf = StreamFactory::default_factory().unwrap();

            let sample_rate = sf.config().sample_rate.0;
            let mut sample_clock = 0;
            let routin = move |len: usize| -> Vec<f32> {
                (0..len / 2) // len is apparently left *and* right
                    .flat_map(|_| {
                        sample_clock = (sample_clock + 1) % sample_rate;

                        let (l, r) = audio_func(sample_clock as f32 / sample_rate as f32);
                        vec![l, r]
                    })
                    .collect()
            };

            let stream = sf.create_stream(routin).unwrap();
            StreamTrait::play(&stream).unwrap();
        }

        // Event loop

        let mut last_print_time = SystemTime::now();
        let mut frame_count = 0;

        self.event_loop
            .run_on_demand( |event, elwt| {
                elwt.set_control_flow(ControlFlow::Poll);

                // File watching and reloading application
                if let Ok(event) = &rx.try_recv() {
                    if let Ok(e) = event {
                        match e.kind {
                            Access(Close(Write)) | Modify(_) => {
                                log::info!("File write event: {:?}", e.paths);

                                // Currently just reloads all shaders, it might be better to only compile the changed shader
                                let new_orch = DrawOrchestrator::new(&mut self.renderer, resolution, &draw_config);
                                match new_orch {
                                    Ok(o) => {
                                        orchestrator = o;
                                    }
                                    Err(e) => {
                                        error!("{}", e);
                                        log::info!("Shader contains error, not updating");
                                    }
                                }
                            },
                            _ => {}
                        }
                    }
                }

                // Window event
                match event {
                    | Event::NewEvents(StartCause::Poll) => {
                        self.renderer.draw_frame(&mut orchestrator);

                        if self.app_config.log_fps {
                            let current_frame_time = SystemTime::now();
                            let elapsed = current_frame_time.duration_since(last_print_time).unwrap();
                            frame_count += 1;

                            if elapsed.as_secs() >= 1 {
                                info!("fps: {}, frametime: {:.3}ms", frame_count, elapsed.as_millis() as f32 / frame_count as f32);
                                frame_count = 0;
                                last_print_time = current_frame_time;
                            }
                        }
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