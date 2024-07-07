use glam::{UVec3};
use akai::app::app::App;
use akai::app::draw_orch::{DrawConfig, Pass};

fn main() {
    let mut config = DrawConfig::new();
    config.passes.push( Pass {
        shader: "examples/fractals/shaders/fractal_shader.comp".to_string(),
        dispatches: UVec3::new( 100, 100, 1 ),
    });

    let app = App::new();
    app.run(config);
}
