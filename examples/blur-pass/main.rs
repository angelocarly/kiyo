use kiyo::app::app::{App, AppConfig};
use kiyo::app::draw_orch::{DispatchConfig, DrawConfig, Pass};

fn main() {

    let app = App::new(AppConfig {
        width: 1000,
        height: 1000,
        vsync: true,
        log_fps: true,
    });

    let mut config = DrawConfig::new();
    config.passes = Vec::from([
        Pass {
            shader: "examples/blur-pass/shaders/screen_shader.comp".to_string(),
            dispatches: DispatchConfig::FullScreen,
            input_resources: Vec::from([]),
            output_resources: Vec::from([ 0 ]),
        },
        Pass {
            shader: "examples/blur-pass/shaders/blur.comp".to_string(),
            dispatches: DispatchConfig::FullScreen,
            input_resources: Vec::from([ 0 ]),
            output_resources: Vec::from([ 1 ]),
        }
    ]);

    app.run(config);
}
