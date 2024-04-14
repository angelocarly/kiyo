mod window;
mod application;
mod vulkan;

fn main() {
    let mut app = application::Application::new();
    app.run();
}
