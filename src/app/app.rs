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
    pub fullscreen: bool,
}

impl App {

    pub fn run(app_config: AppConfig, draw_config: DrawConfig, audio_func: Option<fn(f32)->(f32, f32)>) {

        let cen_conf = cen::app::app::AppConfig::default()
            .width(app_config.width)
            .height(app_config.height)
            .vsync(app_config.vsync)
            .fullscreen(app_config.fullscreen)
            .log_fps(app_config.log_fps);

        // Parse orchestrator
        let orchestrator = DrawOrchestrator::new(draw_config);

        // audio
        let player = audio_func.map(|f| {
            AudioPlayer::new(f)
        });
        if let Some(p) = &player {
            p.play();
        }

        // Run graphics backend
        cen::app::App::run(cen_conf, Box::new(orchestrator));
    }
}