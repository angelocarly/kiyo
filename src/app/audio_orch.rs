#[derive(Clone)]
pub enum AudioConfig {
    None,
    Program(fn(f32)->(f32, f32)),
    AudioFile(String)
}