use bevy::{diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, prelude::*, window::PrimaryWindow};
use bevy_quick_response::{QuickResponseMode, QuickResponseParameters, QuickResponsePlugin};

fn main() {
    let quick_response_plugin = QuickResponsePlugin::new(
            QuickResponseMode::FastVsync(QuickResponseParameters {
            base_fps: 60.0, // Base FPS, for example: when window is not focused
            max_fps: 60.0, // Max FPS, for example: when mouse moves over window
            auto_init_default_plugins: false, // Disable DefaultPlugin initialization
        })
    );

    let mut window_plugin = quick_response_plugin.window_plugin();
    window_plugin.primary_window.as_mut().unwrap().title = "Advanced Example".to_string();

    App::new()
        .add_plugins(DefaultPlugins.set(
            window_plugin
        ))
        .add_plugins(quick_response_plugin)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, setup_fps_text)
        .add_systems(Update, draw_gizmos)
        .add_systems(Update, show_fps)
        .run();
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct FpsText;

fn setup_camera(
    mut commands: Commands,
) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

fn draw_gizmos(
    mut gizmos: Gizmos,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let window = windows.single();
    let (camera, camera_transform) = cameras.single();

    if let Some(position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        gizmos.circle_2d(position, 10.0, Color::RED);
    }
}

pub fn setup_fps_text(
    mut commands: Commands,
) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font_size: 30.0,
                    ..default()
                },
            ),
            TextSection::new(
                "0",
                TextStyle {
                    font_size: 30.0,
                    ..default()
                },
            ),
        ]),
        FpsText,
    ));
}

fn show_fps(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}