# Akai
![build](https://github.com/angelocarly/akai/actions/workflows/rust.yml/badge.svg)  
Generative art rendering library using [ash](https://github.com/ash-rs/ash).

## Roadmap

- [ ] A simple GUI library. For example with [imgui-rs-vulkan-renderer](https://github.com/adrien-ben/imgui-rs-vulkan-renderer).
- [ ] An integrated deferred rendering system.

## Building & running

Make sure you have the [Vulkan SDK](https://vulkan.lunarg.com) installed.  
Then build `akai`:
```
git clone https://github.com/angelocarly/akai.git
cd akai
cargo run --bin compute_test
```

## Libraries
- [ash](https://github.com/ash-rs/ash) - Vulkan bindings
- [winit](https://github.com/rust-windowing/winit) - Window creation and handling
- [shaderc](https://github.com/google/shaderc-rs) - Shader compilation
- [gpu-allocator](https://github.com/Traverse-Research/gpu-allocator?tab=readme-ov-file) - Memory management
