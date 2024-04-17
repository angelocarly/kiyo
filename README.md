# Akai
![build](https://github.com/angelocarly/lov/actions/workflows/rust.yml/badge.svg)  
Generative art rendering library using [ash](https://github.com/ash-rs/ash).

## Philosophy
As this is my first delve into combining Vulkan with Rust I'd like to keep things simple. Some points I want to consider:
- Keep the library light, don't delve into heavy abstractions early.
- Build a stable basis with ash. I'll get things wrong and inefficient ofc, but let's make those mistakes and improve on them.
- Keep it fun and focus on art. Engine dev is cool af. But discipline and relaxation help eachother.

## Building & running

Install the [Vulkan SDK](https://vulkan.lunarg.com) and set the path environment variables:
```
export VULKAN_SDK=/path/to/vulkan/sdk
# Required for MoltenVK/Mac
export VK_ICD_FILENAMES=/path/to/vulkan/sdk/etc/vulkan/icd.d
```

Then build `akai`:
```
git clone https://github.com/angelocarly/akai.git
cd akai
cargo run
```
