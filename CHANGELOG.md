# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.6](https://github.com/angelocarly/kiyo/compare/v0.0.5...v0.0.6) - 2025-03-10

### Other

- Update Cargo.toml
- Update rust.yml
- Update rust.yml
- Add custom resolution support
- Better error logging when for shader compilation
- Clean up image export using command buffer callbacks
- Add initial image export
- Connect egui
- Fix cen version
- use the most recent cen
- added support for playing audio file
- Add Fullscreen support
- Use kiyo's Appconfig instead of cen's, this way dependencies don't need to include cen
- Fix audio player getting destroyed
- Update CHANGELOG.md
- Update CHANGELOG.md

## [0.0.5](https://github.com/angelocarly/kiyo/compare/v0.0.4...v0.0.5) - 2024-10-19

### Other

- Integrated cpal audio output
- Log shader compilation errors
- Add image clearing options
- Move vulkan and window managent code to "cen" library
- Command buffer reference counting -> cen
- Add vulkan trace logging -> cen

## [0.0.4](https://github.com/angelocarly/kiyo/compare/v0.0.3...v0.0.4) - 2024-08-17

### Other
- Add shader hot-reloading
- Pass environment variables to shaders, no more hardcoded imagecounts!
- Fix and improve blur shader
- Improve shader compilation logging

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
