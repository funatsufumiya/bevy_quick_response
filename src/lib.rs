use std::time::Duration;

use bevy::{prelude::*, winit::{UpdateMode, WinitSettings}};
use bevy_framepace::{FramepacePlugin, FramepaceSettings, Limiter};

pub struct QuickResponsePlugin {
    mode: QuickResponseMode,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum QuickResponseMode {
    /// use Mailbox (FastVsync) for DX11/DX12, Vulkan, and use AutoNoVsync mode for Metal (flickering may occur)
    FastVsync (QuickResponseParameters),
    /// use immediate mode as much as possible (flickering may occur)
    /// NOTE: older DX12 and Wayland may not support this mode (may cause panic)
    Immediate (QuickResponseParameters),
    /// use auto no vsync for all platforms
    /// recommended if you want to work with multiple platforms, but may cause flickering
    AutoNoVsync (QuickResponseParameters),
    /// do nothing: use the app default behavior (VSync)
    None
}

impl Default for QuickResponseMode {
    fn default() -> Self {
        QuickResponseMode::FastVsync(QuickResponseParameters::default())
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct QuickResponseParameters {
    /// base fps, for example: when window is not focused
    /// default: 60.0
    pub base_fps: f64,
    /// max fps, for example: when mouse moves over window
    /// default: 120.0
    pub max_fps: f64,
    /// auto initialize default plugins (DefaultPlugins, and WindowPlugin in it)
    pub auto_init_default_plugins: bool
}

impl Default for QuickResponseParameters {
    fn default() -> Self {
        QuickResponseParameters {
            base_fps: 60.0,
            max_fps: 120.0,
            auto_init_default_plugins: true
        }
    }
}

impl QuickResponsePlugin {
    pub fn new(mode: QuickResponseMode) -> Self {
        QuickResponsePlugin {
            mode,
        }
    }

    pub fn window_plugin(&self) -> WindowPlugin {
        match self.mode {
            QuickResponseMode::FastVsync(_) => {
                WindowPlugin {
                    primary_window: Some(Window {
                        #[cfg(target_os = "windows")]
                        present_mode: bevy::window::PresentMode::Mailbox,
                        #[cfg(target_os = "macos")]
                        present_mode: bevy::window::PresentMode::AutoNoVsync,
                        #[cfg(target_os = "linux")]
                        present_mode: bevy::window::PresentMode::Mailbox,
                        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
                        present_mode: bevy::window::PresentMode::AutoNoVsync,
                        ..default()
                    }),
                    ..default()
                }
            },
            QuickResponseMode::Immediate(_) => {
                WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: bevy::window::PresentMode::Immediate,
                        ..default()
                    }),
                    ..default()
                }
            },
            QuickResponseMode::AutoNoVsync(_) => {
                WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: bevy::window::PresentMode::AutoNoVsync,
                        ..default()
                    }),
                    ..default()
                }
            },
            QuickResponseMode::None => {
                WindowPlugin::default()
            }
        }
    }
}

impl Default for QuickResponsePlugin {
    fn default() -> Self {
        QuickResponsePlugin::new(QuickResponseMode::default())
    }
}

fn setup_fps(max_fps: f64) -> impl Fn(ResMut<FramepaceSettings>) {
    move |mut framepace_settings: ResMut<FramepaceSettings>| {
        framepace_settings.limiter = Limiter::from_framerate(max_fps);
    }
}

impl Plugin for QuickResponsePlugin {
    fn build(&self, app: &mut App) {
        if self.mode == QuickResponseMode::None {
            // do nothing
            return;
        }

        let base_fps = match self.mode {
            QuickResponseMode::FastVsync(params) => params.base_fps,
            QuickResponseMode::AutoNoVsync(params) => params.base_fps,
            QuickResponseMode::Immediate(params) => params.base_fps,
            _ => unreachable!()
        };

        let max_fps = match self.mode {
            QuickResponseMode::FastVsync(params) => params.max_fps,
            QuickResponseMode::AutoNoVsync(params) => params.max_fps,
            QuickResponseMode::Immediate(params) => params.max_fps,
            _ => unreachable!()
        };

        let auto_init_default_plugins = match self.mode {
            QuickResponseMode::FastVsync(params) => params.auto_init_default_plugins,
            QuickResponseMode::AutoNoVsync(params) => params.auto_init_default_plugins,
            QuickResponseMode::Immediate(params) => params.auto_init_default_plugins,
            _ => unreachable!()
        };

        app
            .insert_resource(WinitSettings {
                focused_mode: UpdateMode::ReactiveLowPower { wait: Duration::from_secs_f64(1.0 / base_fps) },
                unfocused_mode: UpdateMode::ReactiveLowPower { wait: Duration::from_secs_f64(1.0 / base_fps) },
                ..default()
            })
            ;

        app
            .add_plugins(())
            ;

        if auto_init_default_plugins {
            app.add_plugins(DefaultPlugins.set(
                self.window_plugin()
            ));
        }

        app.add_plugins(FramepacePlugin);
        app.add_systems(Startup, setup_fps(max_fps));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin() {
        App::new()
            // .add_plugins(MinimalPlugins)
            .add_plugins(QuickResponsePlugin::default())
            .update();
    }
}