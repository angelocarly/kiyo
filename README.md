# Kiyo
![build](https://github.com/angelocarly/akai/actions/workflows/rust.yml/badge.svg)
[![crate](https://img.shields.io/crates/v/kiyo)](https://crates.io/crates/kiyo/)  

## What is Kiyo?
A lightweight compute shader engine using [ash](https://github.com/ash-rs/ash).

Kiyo provides a simple configuration interface to run one or more shaders, and synchronize their input/output images.

You can find examples in [./examples](./examples) and in my [toy project repository](https://github.com/angelocarly/kiyo-projects).

## Building & running

Make sure you have the [Vulkan SDK](https://vulkan.lunarg.com) installed.  
Then build and run `kiyo`:
```
git clone https://github.com/angelocarly/kiyo.git
cd kiyo
cargo run --example simple-render
```

### Mac
In order to debug Kiyo on Mac, you need to provide the following environment variables:
```bash
VULKAN_SDK=$HOME/VulkanSDK/<version>/macOS
DYLD_FALLBACK_LIBRARY_PATH=$VULKAN_SDK/lib
VK_ICD_FILENAMES=$VULKAN_SDK/share/vulkan/icd.d/MoltenVK_icd.json
VK_LAYER_PATH=$VULKAN_SDK/share/vulkan/explicit_layer.d
```

## Libraries
- [ash](https://github.com/ash-rs/ash) - Vulkan bindings
- [winit](https://github.com/rust-windowing/winit) - Window creation and handling
- [shaderc](https://github.com/google/shaderc-rs) - Shader compilation
- [gpu-allocator](https://github.com/Traverse-Research/gpu-allocator?tab=readme-ov-file) - Memory management
