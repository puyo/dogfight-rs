use bevy::{
    app::AppExit,
    input::{
        keyboard::{KeyCode, KeyboardInput},
        ElementState,
    },
    prelude::*,
};

/// Exits app when following keys are pressed: Esc, Command-Q, Windows-Q, Alt-F4
pub fn exit_system(
    input: Res<Input<KeyCode>>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for event in keyboard_input_events.iter() {
        if let Some(key_code) = event.key_code {
            if event.state == ElementState::Pressed
                && (key_code == KeyCode::Escape
                    || key_code == KeyCode::Q
                        && (input.pressed(KeyCode::LWin) || input.pressed(KeyCode::RWin))
                    || key_code == KeyCode::F4
                        && (input.pressed(KeyCode::LAlt) || input.pressed(KeyCode::RAlt)))
            {
                app_exit_events.send(AppExit);
            }
        }
    }
}
