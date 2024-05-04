# Akai
![build](https://github.com/angelocarly/lov/actions/workflows/rust.yml/badge.svg)  
Generative art rendering library using [ash](https://github.com/ash-rs/ash).

## Philosophy
This is my first serious try into combining Vulkan and Rust, for that reason I'll try to keep this project simple. Some points I consider:
- Keep the library light, don't delve into heavy abstractions early on.
- Build a stable basis with ash. I'll get things wrong and inefficient ofc, but let's make those mistakes and improve on them.
- Keep it fun and focus on art. Engine dev is cool af. But discipline and *relaxation* help eachother.

I'm using the following repositories to kickstart this project. They might be helpful for you as well.
- [vulkan-tutorial-rust](https://github.com/unknownue/vulkan-tutorial-rust)
- [ash-example](https://github.com/ash-rs/ash/blob/master/ash-examples/src/lib.rs)

## Building & running

Make sure you have the [Vulkan SDK](https://vulkan.lunarg.com) installed.  
Then build `akai`:
```
git clone https://github.com/angelocarly/akai.git
cd akai
cargo run
```

## Libraries
- [ash](https://github.com/ash-rs/ash) - Vulkan bindings
- [winit](https://github.com/rust-windowing/winit) - Window creation and handling

## Possible extensions
- [imgui-rs-vulkan-renderer](https://github.com/adrien-ben/imgui-rs-vulkan-renderer) - Add imgui support.
