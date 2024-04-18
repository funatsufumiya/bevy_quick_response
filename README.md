# bevy_quick_response

[![Crates.io](https://img.shields.io/crates/v/bevy_quick_response)](https://crates.io/crates/bevy_quick_response)
[![Docs.rs](https://docs.rs/bevy_quick_response/badge.svg)](https://docs.rs/bevy_quick_response)
[![License](https://img.shields.io/crates/l/bevy_quick_response)](LICENSE)

(README japanese: [README_ja.md](README_ja.md))

A Bevy plugin, which changes the initial settings to respond immediately to user input.

The normal behavior of Bevy is to turn on VSync, which causes a delay of 3 frames. On the other hand, turning off VSync removes the FPS limit and increases the load on the CPU/GPU.

This plugin changes the settings to turn off VSync to improve responsiveness while behaving as close as possible to when VSync is on. (By default, the base FPS is set to 60 and the maximum FPS is set to 120.)

## Usage

```rust
app.add_plugins(QuickResponsePlugin::default())
```

(`DefaultPlugin` is automatically enabled, so no need to add it.)

Customizing the behavior, see [examples/advanced.rs](examples/advanced.rs).

## Version table

| Bevy | bevy_quick_response |
|---------|-----------------------------|
| 0.13          | 0.1                       |