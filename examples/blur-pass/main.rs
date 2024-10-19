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
            ImageConfig {
                clear: ClearConfig::Color(0.0, 0.0, 0.0)
            },
        ]),
        passes: Vec::from([
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
        ])
    };

    app.run(config, None);
}
