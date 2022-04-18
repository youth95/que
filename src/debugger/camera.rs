use bevy::prelude::*;
use bevy_console::ConsoleCommand;

#[derive(ConsoleCommand)]
#[console_command(name = "c")]
pub struct CameraCommand {
    /// Some message
    d_type: String,
    value: i32,
}

pub fn camera_command(
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
