use glam::{UVec2, UVec3};
use akai::app::app::App;
use akai::app::draw_orch::{DrawConfig, ImageResource, Pass};

fn main() {
    let mut config = DrawConfig::new();

    config.resolution = UVec2::new( 1000, 1000 );
    let workgroup_size = UVec3::new( 32, 32, 1 );

    let full_screen_dispatches = UVec3::new(
        (config.resolution.x as f32 / workgroup_size.x as f32).ceil() as u32,
        (config.resolution.y as f32 / workgroup_size.y as f32).ceil() as u32,
        1
    );

    config.image_resources = Vec::from([
        ImageResource {
            name: "draw".to_string(),
        }
    ]);

    config.passes = Vec::from([
        Pass {
            shader: "examples/draworchestra/shaders/screen_shader.comp".to_string(),
            dispatches: full_screen_dispatches,
            input_resources: Vec::new(),
            output_resources: Vec::from([ "draw".into() ]),
        },
        Pass {
            shader: "examples/draworchestra/shaders/blur.comp".to_string(),
            dispatches: full_screen_dispatches,
            input_resources: Vec::from([ "draw".into() ]),
            output_resources: Vec::from([ "draw".into() ]),
        }
    ]);

    let app = App::new();
    app.run(config);
}
