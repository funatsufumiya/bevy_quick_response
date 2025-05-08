use bevy::{diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, prelude::*, window::PrimaryWindow};
use bevy_quick_response::QuickResponsePlugin;

pub fn close_on_esc(
    mut commands: Commands,
    focused_windows: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (window, focus) in focused_windows.iter() {
        if !focus.focused {
            continue;
        }

        if input.just_pressed(KeyCode::Escape) {
            commands.entity(window).despawn();
        }
    }
}

fn main() {
    App::new()
        // NOTE: DefaultPlugin added automatically by default in QuickResponsePlugin
        .add_plugins(QuickResponsePlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_systems(Update, close_on_esc)
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
    commands.spawn((Camera2d::default(), MainCamera));
}

fn draw_gizmos(
    mut gizmos: Gizmos,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let window = windows.single().unwrap();
    let (camera, camera_transform) = cameras.single().unwrap();

    if let Some(position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())
    {
        // gizmos.circle_2d(position, 10.0, Color::RED);
        gizmos.circle_2d(position, 10.0, Color::linear_rgb(1.0, 0.0, 0.0));
    }
}

pub fn setup_fps_text(
    mut commands: Commands,
) {
    commands.spawn((
        Text::new("FPS: "),
        TextFont { font_size: 30.0, ..default() },
    )).with_children(|parent| {
        parent.spawn(
        (
            TextSpan::new("0"),
            TextFont { font_size: 30.0, ..default() },
            FpsText,
        ));
    });
}

fn show_fps(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut TextSpan, With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.0 = format!("{value:.2}");
            }
        }
    }
}