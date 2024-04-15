pub mod application;
mod vulkan;
mod window;

#[cfg(test)]
mod tests {
    use crate::application::Application;

    #[test]
    fn basic_run() {
        let mut app = Application::new();
        app.run();
    }
}
