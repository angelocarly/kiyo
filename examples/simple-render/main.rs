use kiyo::app::app::{App, AppConfig};
use kiyo::app::draw_orch::{DispatchConfig, DrawConfig, Pass};

fn main() {

    let app = App::new(AppConfig {
        width: 1000,
        height: 1000,
        vsync: true,
        log_fps: false,
    });

    let mut config = DrawConfig::new();
    config.passes = Vec::from([
        Pass {
            shader: "examples/simple-render/shaders/colors.comp".to_string(),
            dispatches: DispatchConfig::FullScreen,
            input_resources: Vec::from([]),
            output_resources: Vec::from([ 0 ]),
        },
    ]);

    app.run(config);
}
