use akai::application::Application;
use winit::event_loop::EventLoop;

fn main() {
    let event_loop = EventLoop::new().expect("Failed to create event loop.");
    let app = Application::new(&event_loop, "Akai engine", 800, 600);
    app.run(event_loop);
}
