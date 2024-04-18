use bevy::{diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, prelude::*, window::PrimaryWindow};
use bevy_quick_response::QuickResponsePlugin;

// NOTE:
// FPS update will stop when the window is not focused
// so if showing something, please treat it just as information

fn main() {
    App::new()
        // NOTE: DefaultPlugin added automatically by default in QuickResponsePlugin
        .add_plugins(QuickResponsePlugin::power_saving(60.0))
        // .add_plugins(QuickResponsePlugin::immediate(60.0, 60.0)) // other mode
        // .add_plugins(QuickResponsePlugin::auto_no_vsync(60.0, 60.0)) // other mode
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