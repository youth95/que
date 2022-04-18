mod camera;
mod switch;
mod trigger;

use bevy::prelude::*;
use bevy_console::{AddConsoleCommand, ConsoleConfiguration, ConsolePlugin};

use self::{
    camera::{camera_command, CameraCommand},
    switch::{switch_command, SwitchCommand},
    trigger::{trigger_command, TriggerCommand},
};
pub struct DebuggerPlugin;

impl Plugin for DebuggerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ConsolePlugin)
            .insert_resource(ConsoleConfiguration {
                top_pos: 0.,
                left_pos: 1024.,
                width: 300.,
                height: 660.,
                ..Default::default()
            })
            .add_console_command::<CameraCommand, _, _>(camera_command)
            .add_console_command::<TriggerCommand, _, _>(trigger_command)
            .add_console_command::<SwitchCommand, _, _>(switch_command);
    }
}
