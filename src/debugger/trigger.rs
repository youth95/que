use bevy::prelude::*;
use bevy_console::ConsoleCommand;

use crate::regions::TriggerRegionEvent;

#[derive(ConsoleCommand)]
#[console_command(name = "tr")]
pub struct TriggerCommand {
    value: i32,
}

pub fn trigger_command(
    mut log: ConsoleCommand<TriggerCommand>,
    mut trigger_region_event: EventWriter<TriggerRegionEvent>,
) {
    if let Some(TriggerCommand { value }) = log.take() {
        trigger_region_event.send(TriggerRegionEvent(value as u64));
    }
}
