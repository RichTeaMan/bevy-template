use bevy::prelude::*;

use crate::{
    events::{ControlAction, ControlEvent},
    ui::DebugInfo,
};

pub fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    mut control_events: ResMut<Events<ControlEvent>>,
    mut debug_query: Query<&mut DebugInfo>,
) {
    if keys.pressed(KeyCode::W) {
        control_events.send(ControlEvent {
            action: ControlAction::Forward,
        });
    } else if keys.pressed(KeyCode::S) {
        control_events.send(ControlEvent {
            action: ControlAction::Backwards,
        });
    }
    if keys.pressed(KeyCode::A) {
        control_events.send(ControlEvent {
            action: ControlAction::Left,
        });
    } else if keys.pressed(KeyCode::D) {
        control_events.send(ControlEvent {
            action: ControlAction::Right,
        });
    }
    if keys.just_pressed(KeyCode::F3) {
        let mut debug_info = debug_query.single_mut();
        debug_info.enabled = !debug_info.enabled;
    }
}
