use kiyo::app::app::App;
use kiyo::app::draw_orch::{DispatchConfig, DrawConfig, Pass};

fn main() {
    let mut config = DrawConfig::new();

    config.passes = Vec::from([
        Pass {
            shader: "examples/simple-render/shaders/screen_shader.comp".to_string(),
            dispatches: DispatchConfig::FullScreen,
            input_resources: Vec::from([]),
            output_resources: Vec::from([ 0 ]),
        },
        Pass {
            shader: "examples/simple-render/shaders/blur.comp".to_string(),
            dispatches: DispatchConfig::FullScreen,
            input_resources: Vec::from([ 0 ]),
            output_resources: Vec::from([ 1 ]),
        }
    ]);

    let app = App::new(1000, 1000);
    app.run(config);
}
