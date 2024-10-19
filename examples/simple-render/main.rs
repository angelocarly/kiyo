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
                shader: "examples/simple-render/shaders/colors.comp".to_string(),
                dispatches: DispatchConfig::FullScreen,
                input_resources: Vec::from([]),
                output_resources: Vec::from([ 0 ]),
            },
        ])
    };

    app.run(config, None);
}
