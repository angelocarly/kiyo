use akai::app::app::App;
use akai::app::DrawOrchestrator;

fn main() {
    let mut orch = DrawOrchestrator::new();

    let app = App::new();
    app.run(&mut orch);
}
