use crate::app::StreamFactory;
use crate::app::draw_orch::{DrawConfig};
use cpal::Stream;
use glam::UVec2;
use log::{error, info};
use cpal::traits::StreamTrait;
use crate::app::{DrawOrchestrator};

pub struct App {
    pub cen: cen::app::App,
}

struct AudioPlayer {
    stream: Stream,
}

impl AudioPlayer {
    fn new(func: fn(f32)->(f32, f32)) -> Self {
        let sf = StreamFactory::default_factory().unwrap();

        let sample_rate = sf.config().sample_rate.0;
        let mut sample_clock = 0;
        let routin = Box::new(move |len: usize| -> Vec<f32> {
            (0..len / 2) // len is apparently left *and* right
                .flat_map(|_| {
                    sample_clock = (sample_clock + 1) % sample_rate;

                    let (l, r) = func(sample_clock as f32 / sample_rate as f32);
                    vec![l, r]
                })
                .collect()
        });

        Self {
            stream: sf.create_stream(routin).unwrap() // creates stream from function "routin"
        }
    }

    fn play(&self) {
        StreamTrait::play(&self.stream).unwrap();
    }
}

pub struct AppConfig {
    pub width: u32,
    pub height: u32,
    pub vsync: bool,
    pub log_fps: bool,
}

impl App {

    pub fn new(app_config: AppConfig) -> App{
        App {
            cen: cen::app::App::new(cen::app::app::AppConfig {
                width: app_config.width,
                height: app_config.height,
                vsync: app_config.vsync,
                log_fps: app_config.log_fps,
            }),
        }
    }

    pub fn run(mut self, draw_config: DrawConfig, audio_func: Option<fn(f32)->(f32, f32)>) {

        // Parse orchestrator
        let resolution = UVec2::new( self.cen.window().get_extent().width, self.cen.window().get_extent().height );
        let orchestrator = match DrawOrchestrator::new(&mut self.cen.renderer(), resolution, &draw_config) {
            Ok(d) => {
                d
            },
            Err(e) => {
                error!("{}", e);
                info!("A shader contains an error, quitting");
                std::process::abort();
            }
        };

        // audio
        let player = audio_func.map(|f| {
            AudioPlayer::new(f)
        });
        if let Some(p) = &player {
            p.play();
        }

        // Run graphics backend
        self.cen.run(Box::new(orchestrator));
    }
}