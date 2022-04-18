use bevy::prelude::*;
use bevy_console::{AddConsoleCommand, ConsoleCommand, ConsoleConfiguration, ConsolePlugin};
use que::regions::{spawn_tiles_sprite_system, Regions};
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ConsolePlugin)
        .insert_resource(ConsoleConfiguration {
            ..Default::default()
        })
        .insert_resource(WindowDescriptor {
            title: "䧿".to_string(),
            ..Default::default()
        })
        .init_resource::<Regions>()
        .add_startup_system(spawn_tiles_sprite_system)
        .add_console_command::<CameraCommand, _, _>(example_command)
        .run();
}

#[derive(ConsoleCommand)]
#[console_command(name = "c",)]
struct CameraCommand {
    /// Some message
    d_type: String,
    value: i32,
}

fn example_command(
    mut log: ConsoleCommand<CameraCommand>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    if let Some(CameraCommand { d_type, value }) = log.take() {
        for mut camera in camera_query.iter_mut() {
            match d_type.as_str() {
                "x" => camera.translation.x += value as f32,
                "y" => camera.translation.y += value as f32,
                "z" => camera.translation.z += value as f32,
                _ => {}
            }
        }
    }
}
