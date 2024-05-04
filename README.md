# Akai
![build](https://github.com/angelocarly/lov/actions/workflows/rust.yml/badge.svg)  
Generative art rendering library using [ash](https://github.com/ash-rs/ash).

## Philosophy

Akai is meant to be an easily extendable graphics library. With a complexity somewhere in between the hand-holding of OpenGL and the pitfall of Vulkan.
The goal is to have an enjoyable home for graphic programming algorithms and techniques to grow.

In order to reach this goal, I try to focus on the following:
- Keep the library light, don't delve into heavy abstractions early on.
- Routinely clean up the codebase. This is to keep the pace controlled and keep the codebase enjoyable to navigate.
- Build a stable basis with ash. I'll get things wrong and inefficient ofc, but let's make those mistakes and improve on them.
- Keep it fun and focus on art. Engine dev is cool af. But discipline and *relaxation* help eachother.

I'm using the following examples to kickstart this project:
- [vulkan-tutorial-rust](https://github.com/unknownue/vulkan-tutorial-rust) - A Rust/Ash port of [Vulkan Tutorial](https://vulkan-tutorial.com/)
- [ash-example](https://github.com/ash-rs/ash/blob/master/ash-examples/src/lib.rs) - Ash examples

## Roadmap

The repository is still in a setup state. The following steps are necessary to get the project going:

1. [ ] Get basic Vulkan wrappers working.
2. [ ] Get the bare minimum of a Vulkan renderer going. These are:
   - [ ] Resizable windows
   - [ ] In-flight command buffers
3. Specific engine dev can start
 
Once the basic engine skeleton is set up, then child projects can be started to experiment with adding new functionality to Akai.

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
