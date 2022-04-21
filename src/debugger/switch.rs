use bevy::prelude::{Query, Visibility, With};
use bevy_console::ConsoleCommand;

use crate::marks::IDText;

#[derive(ConsoleCommand)]
#[console_command(name = "set")]
pub struct SwitchCommand {
    flag: String,
    value: i32,
}

pub fn switch_command(
    mut log: ConsoleCommand<SwitchCommand>,
    mut id_text_visibility_query: Query<&mut Visibility, With<IDText>>,
) {
    if let Some(SwitchCommand { value, flag }) = log.take() {
        let iter = match flag.as_str() {
            "id" => id_text_visibility_query.iter_mut(),
            _ => todo!(),
        };

        for mut visible in iter {
            *visible = Visibility {
                is_visible: value != 0,
            };
        }
    }
}
