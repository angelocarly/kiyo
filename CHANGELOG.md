# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.5](https://github.com/angelocarly/kiyo/compare/v0.0.4...v0.0.5) - 2024-10-19

### Other

- Move vulkan and window code to "cen" library
- Clean up sharpen pass
- Remove extra space
- Fix sharpen shader
- Extract render code to orchestrator
- Feedback loop example update
- Also log hot-reload errors
- Output compilation errors
- Add clearing example
- Add image clear options
- Fix imports
- Clean up imports
- Command buffer reference counting
- Add initial pipeline reconstruction
- Fix reference name
- Pass path through UserEvent
- Refactoring
- Use watch_callback and UserEvents
- Remove unneeded RenderContext
- Slotmap setup for pipelines
- Use SlotMap to store pipelines
- Add references
- Update dependencies
- Add vulkan trace logging
- Add reference to Readme
- Only playback audio when a func is provided
- Remove test to verify if this break build
- Reorder dependencies and fix missing Display trait
- Reorder dependencies
- Add alsa system dependency
- Add alsa system dependency
- Add dummy alsa audio driver
- integrated cpal audio output
- Check for hot-reload events on Mac

## [0.0.4](https://github.com/angelocarly/kiyo/compare/v0.0.3...v0.0.4) - 2024-08-17

### Other
- Add a little bit of documentation
- Fix compile errors
- Update README.md
- Merge branch 'refs/heads/main' into feature/hot_reload
- Add shader hot-reload
- Fix and improve blur shader
- Improve shader compilation logging
- Clean up examples
- Calculate and pass the macros into the shader compilation
- Pass compute image count through code

## [0.0.3](https://github.com/angelocarly/kiyo/compare/v0.0.2...v0.0.3) - 2024-08-14

### Other
- Create release.yml
- Switch swapchain image copy to a blit
- Lighten the blur pass
- Merge pull request [#19](https://github.com/angelocarly/kiyo/pull/19) from angelocarly/feature/fps_counter
- Add fps logging and vsync option
- Remove calloop log spam
- Automatically deduce the image count
- Add logging
