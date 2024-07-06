use akai::app::app;
use akai::app::DrawOrchestrator;

struct OrchTest {

}

impl DrawOrchestrator for OrchTest {

}

fn main() {

    let mut orch = OrchTest{};

    let app = app::App::new();
    app.run(&mut orch);
}
