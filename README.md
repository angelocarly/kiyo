# Kiyo
![build](https://github.com/angelocarly/akai/actions/workflows/rust.yml/badge.svg)  
Generative art graphics engine using [ash](https://github.com/ash-rs/ash).

## Architecture

Kiyo currently contains two nondistinct layers.
- Vulkan wrapper code
- Engine management code

The engine management code (searching for a name) is the interface for the end user. This contains the high level
rendering bindings. The management layer uses the Vulkan layer for convenience. It is *not* the intention to allow the
end user to access ash/vulkan objects immediately.

There now is enough Vulkan logic to run some simple rendering applications, see examples. But for further development
it will be useful to have a simple high level abstraction the end user can utilize. Current development will focus on
that.

## Planned features

- [ ] CI tests using swiftshader. Possibly using [vulkanci](https://github.com/marketplace/actions/vulkanci).
- [ ] A simple GUI library. For example with [imgui-rs-vulkan-renderer](https://github.com/adrien-ben/imgui-rs-vulkan-renderer).
- [ ] An integrated deferred rendering system.
  - Could be an interesting way to easily add lighting to generative art, as lighting computations are done in a separate pass.

## Building & running

Make sure you have the [Vulkan SDK](https://vulkan.lunarg.com) installed.  
Then build `akai`:
```
git clone https://github.com/angelocarly/akai.git
cd akai
cargo run --example compute-pipeline
```

## Libraries
- [ash](https://github.com/ash-rs/ash) - Vulkan bindings
- [winit](https://github.com/rust-windowing/winit) - Window creation and handling
- [shaderc](https://github.com/google/shaderc-rs) - Shader compilation
- [gpu-allocator](https://github.com/Traverse-Research/gpu-allocator?tab=readme-ov-file) - Memory management
