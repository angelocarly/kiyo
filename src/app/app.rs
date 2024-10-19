use crate::app::StreamFactory;
use crate::app::draw_orch::{DrawConfig};
use cen::app::app::AppConfig;
use glam::UVec2;
use log::{error, info};
use cpal::traits::StreamTrait;
use crate::app::{DrawOrchestrator};

pub struct App {
    pub cen: cen::app::App,
}

impl App {

    pub fn new(app_config: AppConfig) -> App{
        App {
            cen: cen::app::App::new(app_config),
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

        self.cen.run(Box::new(orchestrator));
    }
}