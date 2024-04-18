# bevy_quick_response

[![Crates.io](https://img.shields.io/crates/v/bevy_quick_response)](https://crates.io/crates/bevy_quick_response)
[![Docs.rs](https://docs.rs/bevy_quick_response/badge.svg)](https://docs.rs/bevy_quick_response)
[![License](https://img.shields.io/crates/l/bevy_quick_response)](LICENSE)

Bevyでユーザー入力に対して即座に反応するように初期設定を変更するプラグインです。

Bevyの通常の挙動は、VSyncがオンになり、3フレームの遅延が発生します。一方でVSyncをオフにすると、FPSに制限がなくなり、CPU/GPUの負荷が高まります。

このプラグインでは、VSyncをオフにして応答性を改善しつつも、できる限りVSyncをオンにした場合と同じように動作するように設定を変更します。(デフォルトでは、ベースFPSは60、最大FPSは120に設定されます。)

## Usage

```rust
app.add_plugins(QuickResponsePlugin::default())
```

(`DefaultPlugin`も自動で有効化されるため、追加の必要はありません。)

挙動をカスタマイズしたい場合は、[examples/advanced.rs](examples/advanced.rs)を参照してください。

## Version table

| Bevy | bevy_quick_response |
|---------|-----------------------------|
| 0.13          | 0.1                       |

## このプラグインがやっていること

- デフォルトでは、Windows/Linux (DX11/DX12, Vulkan) では[`Mailbox`](https://docs.rs/bevy_window/latest/bevy_window/enum.PresentMode.html#variant.Mailbox) (Fast VSync)、macOS (Metal) やそれ以外では [`AutoNoVsync`](https://docs.rs/bevy_window/latest/bevy_window/enum.PresentMode.html#variant.AutoNoVsync) が選択されます。
- ベースFPSは60、最大FPSは120に設定されます。
    - ベースFPSの設定は [`UpdateMode::ReactiveLowPower`](https://docs.rs/bevy/latest/bevy/winit/enum.UpdateMode.html#variant.ReactiveLowPower) の`wait`を利用しています。
    - 最大FPSの制限は、[bevy_framepace](https://github.com/aevyrie/bevy_framepace)を利用しています。

## 注意点

- Macでは現状Fast VSyncにあたるモードがBevyで提供されていないため、チラツキが発生する可能性がありますが、作者の検証環境(M1/M2)では問題ないチューニングになっています。(最大FPSを120にしているのはその調整のためです。)