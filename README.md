# bevy_quick_response

[![Crates.io](https://img.shields.io/crates/v/bevy_quick_response)](https://crates.io/crates/bevy_quick_response)
[![Docs.rs](https://docs.rs/bevy_quick_response/badge.svg)](https://docs.rs/bevy_quick_response)
[![License](https://img.shields.io/crates/l/bevy_quick_response)](LICENSE)

(README japanese: [README_ja.md](README_ja.md))

A Bevy plugin, which changes the initial settings to respond immediately to user input.

The normal behavior of Bevy is turning on VSync, which causes a delay of 3 frames. On the other hand, turning off VSync removes the FPS limit and increases the load on the CPU/GPU.

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
| 0.16          | 0.4                       |
| 0.15          | 0.3                       |
| 0.14          | 0.2                       |
| 0.13          | 0.1                       |

## What this plugin does

- By default, [`Mailbox`](https://docs.rs/bevy_window/latest/bevy_window/enum.PresentMode.html#variant.Mailbox) (Fast VSync) is selected for Windows/Linux (DX11/DX12, Vulkan), and [`AutoNoVsync`](https://docs.rs/bevy_window/latest/bevy_window/enum.PresentMode.html#variant.AutoNoVsync) is selected for macOS (Metal) and others.
- The base FPS is set to 60, and the maximum FPS is set to 120.
    - The base FPS setting uses the `wait` of [`UpdateMode::reactive_low_power()`](https://docs.rs/bevy/latest/bevy/winit/enum.UpdateMode.html#method.reactive_low_power).
    - The maximum FPS limit uses [bevy_framepace](https://github.com/aevyrie/bevy_framepace).

## Notes

- Currently, there is no mode provided from Bevy Engine that corresponds to Fast VSync on Mac, so there may be flickering, but the tuning is fine in the author's verification environment (M1/M2). (The reason why the maximum FPS is set to 120 is for that adjustment.)

## TODOs

- Update window settings after added `DefaultPlugin` by other settings except this plugin.

## License

Dual licensed under WTFPL and/or 0BSD.