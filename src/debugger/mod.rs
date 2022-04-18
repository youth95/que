mod camera;
mod switch;

use bevy::prelude::*;
use bevy_console::{AddConsoleCommand, ConsoleConfiguration, ConsolePlugin};

use self::{
    camera::{camera_command, CameraCommand},
    switch::{switch_command, SwitchCommand},
};
pub struct DebuggerPlugin;

impl Plugin for DebuggerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ConsolePlugin)
            .insert_resource(ConsoleConfiguration {
                ..Default::default()
            })
            .add_console_command::<CameraCommand, _, _>(camera_command)
            .add_console_command::<SwitchCommand, _, _>(switch_command);
    }
}
