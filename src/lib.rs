use std::time::Duration;

use bevy::{prelude::*, winit::{UpdateMode, WinitSettings}};
use bevy_framepace::{FramepacePlugin, FramepaceSettings, Limiter};

pub struct QuickResponsePlugin {
    pub mode: QuickResponseMode,
    /// if true, do not add the bevy_framepace::FramepacePlugin
    _no_framepace_for_test: bool
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
    /// Power saving mode: choose FastVsync for presentation, and use desktop app settings for winit
    /// NOT recommended for games, but recommended for desktop apps.
    PowerSaving (QuickResponseParametersWithNoBaseFps),
    /// do nothing: use the app default behavior (VSync).
    /// if bool is true, add the default plugins (DefaultPlugins, and WindowPlugin in it).
    /// if bool is false, do nothing.
    None(bool)
}

impl Default for QuickResponseMode {
    fn default() -> Self {
        QuickResponseMode::FastVsync(QuickResponseParameters::default())
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct QuickResponseParameters {
    /// base fps, for example: when window is not focused.
    /// default: 60.0
    pub base_fps: f64,
    /// max fps, for example: when mouse moves over window.
    /// default: 120.0
    pub max_fps: f64,
    /// auto initialize default plugins (DefaultPlugins, and WindowPlugin in it).
    /// default: true
    pub auto_init_default_plugins: bool
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct QuickResponseParametersWithNoBaseFps {
    /// max fps, for example: when mouse moves over window.
    /// default: 120.0
    pub max_fps: f64,
    /// auto initialize default plugins (DefaultPlugins, and WindowPlugin in it).
    /// default: true
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
            _no_framepace_for_test: false
        }
    }

    pub fn power_saving(max_fps: f64) -> Self {
        QuickResponsePlugin::new(QuickResponseMode::PowerSaving(QuickResponseParametersWithNoBaseFps {
            max_fps,
            auto_init_default_plugins: true
        }))
    }

    pub fn fast_vsync(base_fps: f64, max_fps: f64) -> Self {
        QuickResponsePlugin::new(QuickResponseMode::FastVsync(QuickResponseParameters {
            base_fps,
            max_fps,
            auto_init_default_plugins: true
        }))
    }

    pub fn immediate(base_fps: f64, max_fps: f64) -> Self {
        QuickResponsePlugin::new(QuickResponseMode::Immediate(QuickResponseParameters {
            base_fps,
            max_fps,
            auto_init_default_plugins: true
        }))
    }

    pub fn auto_no_vsync(base_fps: f64, max_fps: f64) -> Self {
        QuickResponsePlugin::new(QuickResponseMode::AutoNoVsync(QuickResponseParameters {
            base_fps,
            max_fps,
            auto_init_default_plugins: true
        }))
    }

    pub fn none(should_default_plugins_enabled: bool) -> Self {
        QuickResponsePlugin::new(QuickResponseMode::None(should_default_plugins_enabled))
    }

    pub(crate) fn with_no_framepace_for_test(&self) -> Self {
        QuickResponsePlugin {
            mode: self.mode,
            _no_framepace_for_test: true,
        }
    }

    pub fn with_no_default_plugins(&self) -> Self {
        match self.mode {
            QuickResponseMode::None(_) => {
                QuickResponsePlugin::none(false)
            }
            QuickResponseMode::FastVsync(params) => {
                QuickResponsePlugin::new(
                    QuickResponseMode::FastVsync(QuickResponseParameters {
                        auto_init_default_plugins: false,
                        ..params
                    })
                )
            }
            QuickResponseMode::Immediate(params) => {
                QuickResponsePlugin::new(
                    QuickResponseMode::Immediate(QuickResponseParameters {
                        auto_init_default_plugins: false,
                        ..params
                    })
                )
            }
            QuickResponseMode::AutoNoVsync(params) => {
                QuickResponsePlugin::new(
                    QuickResponseMode::AutoNoVsync(QuickResponseParameters {
                        auto_init_default_plugins: false,
                        ..params
                    })
                )
            }
            QuickResponseMode::PowerSaving(params) => {
                QuickResponsePlugin::new(
                    QuickResponseMode::PowerSaving(QuickResponseParametersWithNoBaseFps {
                        auto_init_default_plugins: false,
                        ..params
                    })
                )
            }
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
            QuickResponseMode::PowerSaving(_) => {
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
            QuickResponseMode::None(_) => {
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

fn is_base_fps_enabled(mode: QuickResponseMode) -> bool {
    match mode {
        QuickResponseMode::FastVsync(_) => true,
        QuickResponseMode::Immediate(_) => true,
        QuickResponseMode::AutoNoVsync(_) => true,
        QuickResponseMode::PowerSaving(_) => false,
        QuickResponseMode::None(_) => false,
    }
}

fn is_power_saving_enabled(mode: QuickResponseMode) -> bool {
    match mode {
        QuickResponseMode::FastVsync(_) => false,
        QuickResponseMode::Immediate(_) => false,
        QuickResponseMode::AutoNoVsync(_) => false,
        QuickResponseMode::PowerSaving(_) => true,
        QuickResponseMode::None(_) => false,
    }
}

impl Plugin for QuickResponsePlugin {
    fn build(&self, app: &mut App) {
        if self.mode == QuickResponseMode::None(false) {
            // do nothing
            return;
        } else if self.mode == QuickResponseMode::None(true) {
            // just add the default plugins
            app.add_plugins(DefaultPlugins);
            return;
        }

        if is_base_fps_enabled(self.mode) {
            let base_fps = match self.mode {
                QuickResponseMode::FastVsync(params) => params.base_fps,
                QuickResponseMode::AutoNoVsync(params) => params.base_fps,
                QuickResponseMode::Immediate(params) => params.base_fps,
                QuickResponseMode::PowerSaving(_) => unreachable!(),
                QuickResponseMode::None(_) => unreachable!(),
            };

            app
                .insert_resource(WinitSettings {
                    focused_mode: UpdateMode::ReactiveLowPower { wait: Duration::from_secs_f64(1.0 / base_fps) },
                    unfocused_mode: UpdateMode::ReactiveLowPower { wait: Duration::from_secs_f64(1.0 / base_fps) },
                    ..default()
                })
                ;
        } else if is_power_saving_enabled(self.mode) {
            app
                .insert_resource(WinitSettings::desktop_app())
                ;
        }

        let max_fps = match self.mode {
            QuickResponseMode::FastVsync(params) => params.max_fps,
            QuickResponseMode::AutoNoVsync(params) => params.max_fps,
            QuickResponseMode::Immediate(params) => params.max_fps,
            QuickResponseMode::PowerSaving(params) => params.max_fps,
            QuickResponseMode::None(_) => unreachable!(),
        };

        let auto_init_default_plugins = match self.mode {
            QuickResponseMode::FastVsync(params) => params.auto_init_default_plugins,
            QuickResponseMode::AutoNoVsync(params) => params.auto_init_default_plugins,
            QuickResponseMode::Immediate(params) => params.auto_init_default_plugins,
            QuickResponseMode::PowerSaving(params) => params.auto_init_default_plugins,
            QuickResponseMode::None(_) => unreachable!(),
        };

        app
            .add_plugins(())
            ;

        if auto_init_default_plugins {
            app.add_plugins(DefaultPlugins.set(
                self.window_plugin()
            ));
        }

        if !self._no_framepace_for_test {
            if !app.is_plugin_added::<FramepacePlugin>() {
                app.add_plugins(FramepacePlugin);
            }
            app.add_systems(Startup, setup_fps(max_fps));
        }
    }
}

#[cfg(test)] #[macro_use]
extern crate assert_matches;

#[cfg(test)]
mod tests {
    use super::*;

    const CHECK_PRECISION : f64 = 0.0001;

    fn float_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < CHECK_PRECISION
    }

    #[test]
    fn test_plugin_none() {
        App::new()
            .add_plugins(MinimalPlugins)
            .add_plugins(QuickResponsePlugin::none(false))
            .update()
    }

    #[test]
    #[should_panic]
    fn test_plugin_none_default_plugins() {
        App::new()
            .add_plugins(MinimalPlugins)
            .add_plugins(QuickResponsePlugin::none(true))
            .update()
    }

    #[test]
    fn test_plugin_power_saving() {
        let pl = QuickResponsePlugin::power_saving(60.0);

        assert_matches!(pl.mode, QuickResponseMode::PowerSaving(
            QuickResponseParametersWithNoBaseFps { max_fps: x, auto_init_default_plugins: true })
            if float_eq(x, 60.0)
        );

        let pl = pl
            .with_no_default_plugins()
            .with_no_framepace_for_test();

        let window_pl = pl.window_plugin();

        #[cfg(any(target_os = "windows", target_os = "linux"))]
        assert_matches!(window_pl.primary_window, Some(Window {
            present_mode: bevy::window::PresentMode::Mailbox, .. })
        );

        #[cfg(target_os = "macos")]
        assert_matches!(window_pl.primary_window, Some(Window {
            present_mode: bevy::window::PresentMode::AutoNoVsync, .. })
        );

        App::new()
            .add_plugins(MinimalPlugins)
            .add_plugins(window_pl)
            .add_plugins(pl)
            .update()
    }

    #[test]
    fn test_plugin_default() {
        let pl = QuickResponsePlugin::default();

        assert_matches!(pl.mode, QuickResponseMode::FastVsync(
            QuickResponseParameters { base_fps: x, max_fps: y, auto_init_default_plugins: true })
            if float_eq(x, 60.0) && float_eq(y, 120.0)
        );

        let pl = pl
            .with_no_default_plugins()
            .with_no_framepace_for_test();

        let window_pl = pl.window_plugin();

        #[cfg(any(target_os = "windows", target_os = "linux"))]
        assert_matches!(window_pl.primary_window, Some(Window {
            present_mode: bevy::window::PresentMode::Mailbox, .. })
        );

        #[cfg(target_os = "macos")]
        assert_matches!(window_pl.primary_window, Some(Window {
            present_mode: bevy::window::PresentMode::AutoNoVsync, .. })
        );

        App::new()
            .add_plugins(MinimalPlugins)
            .add_plugins(window_pl)
            .add_plugins(pl)
            .update()
    }

    #[test]
    fn test_plugin_fast_vsync() {
        let pl = QuickResponsePlugin::new(QuickResponseMode::FastVsync(QuickResponseParameters::default()));

        assert_matches!(pl.mode, QuickResponseMode::FastVsync(
            QuickResponseParameters { base_fps: x, max_fps: y, auto_init_default_plugins: true })
            if float_eq(x, 60.0) && float_eq(y, 120.0)
        );

        let pl = pl
            .with_no_default_plugins()
            .with_no_framepace_for_test();

        let window_pl = pl.window_plugin();
        
        #[cfg(any(target_os = "windows", target_os = "linux"))]
        assert_matches!(window_pl.primary_window, Some(Window {
            present_mode: bevy::window::PresentMode::Mailbox, .. })
        );

        #[cfg(target_os = "macos")]
        assert_matches!(window_pl.primary_window, Some(Window {
            present_mode: bevy::window::PresentMode::AutoNoVsync, .. })
        );

        App::new()
            .add_plugins(MinimalPlugins)
            .add_plugins(window_pl)
            .add_plugins(pl)
            .update()
    }

    #[test]
    fn test_plugin_immediate() {
        let pl = QuickResponsePlugin::new(QuickResponseMode::Immediate(QuickResponseParameters::default()));

        assert_matches!(pl.mode, QuickResponseMode::Immediate(
            QuickResponseParameters { base_fps: x, max_fps: y, auto_init_default_plugins: true })
            if float_eq(x, 60.0) && float_eq(y, 120.0)
        );

        let pl = pl
            .with_no_default_plugins()
            .with_no_framepace_for_test();

        let window_pl = pl.window_plugin();
        
        assert_matches!(window_pl.primary_window, Some(Window {
            present_mode: bevy::window::PresentMode::Immediate, .. })
        );

        App::new()
            .add_plugins(MinimalPlugins)
            .add_plugins(window_pl)
            .add_plugins(pl)
            .update()
    }

    #[test]
    fn test_plugin_auto_no_vsync() {
        let pl = QuickResponsePlugin::new(QuickResponseMode::AutoNoVsync(QuickResponseParameters::default()));

        assert_matches!(pl.mode, QuickResponseMode::AutoNoVsync(
            QuickResponseParameters { base_fps: x, max_fps: y, auto_init_default_plugins: true })
            if float_eq(x, 60.0) && float_eq(y, 120.0)
        );

        let pl = pl
            .with_no_default_plugins()
            .with_no_framepace_for_test();

        let window_pl = pl.window_plugin();
        
        assert_matches!(window_pl.primary_window, Some(Window {
            present_mode: bevy::window::PresentMode::AutoNoVsync, .. })
        );

        App::new()
            .add_plugins(MinimalPlugins)
            .add_plugins(window_pl)
            .add_plugins(pl)
            .update()
    }
}