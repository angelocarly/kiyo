use cen::app::app::AppConfig;
use kiyo::app::app::{App};
use kiyo::app::draw_orch::{ClearConfig, DispatchConfig, DrawConfig, ImageConfig, Pass};

fn main() {

    let app = App::new(AppConfig {
        width: 1000,
        height: 1000,
        vsync: true,
        log_fps: false,
    });

    let config = DrawConfig {
        images: Vec::from([
            ImageConfig {
                clear: ClearConfig::Color(0.0, 0.0, 0.0)
            },
        ]),
        passes: Vec::from([
            Pass {
                shader: "examples/with-audio/shaders/colors.comp".to_string(),
                dispatches: DispatchConfig::FullScreen,
                input_resources: Vec::from([]),
                output_resources: Vec::from([ 0 ]),
            },
        ])
    };

    // t: time in seconds
    fn audio_shader(t:f32) -> (f32, f32) {
        let tau = 2.0 * std::f32::consts::PI;
        let n = f32::sin(tau * 440.0 * t);
        let m = n*f32::powf(1.0-t,3.0);
        let a = (f32::sin(t*tau)/2.0-0.5)*m;
        let b = (f32::sin(t*tau + tau*0.5)/2.0-0.5)*m;

        (a, b)
    }
    app.run(config, Option::Some(audio_shader));
}
