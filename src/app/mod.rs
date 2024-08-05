use bevy::prelude::*;

use crate::ui::DespawnUI;

mod client;
mod connect_to_mpd;
mod help;

/// App states divided by different UX.
#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    /// Initial state, in which connection to MPD will be established.
    /// It shows a loading label, and in case of failure it shows an error message.
    /// If there is no error, it switches to `[AppState::Client]`.
    ConnectToMpd,
    /// Main state with different tabs, where user can interact with MPD.
    /// From this state, it's possible to access `[AppState::Help]`.
    Client,
    /// A general help state with a table of hotkeys and other helpful info.
    /// It's possible to exit this state and return to `[AppState::Client]`.
    Help,
}

/// Plugin for building UI and providing UX.
pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            connect_to_mpd::ConnectToMpdStatePlugin,
            client::ClientStatePlugin,
            help::HelpStatePlugin,
        ))
        .insert_state(AppState::ConnectToMpd)
        // Global
        .add_systems(OnExit(AppState::ConnectToMpd), despawn_ui_system)
        .add_systems(OnExit(AppState::Client), despawn_ui_system)
        .add_systems(OnExit(AppState::Help), despawn_ui_system)
        .add_systems(Update, global_hotkeys_system);
    }
}

/// System to despawn ui every time state changes.
fn despawn_ui_system(mut commands: Commands) {
    commands.add(DespawnUI);
}

/// System to handle hotkeys regardless of current state.
fn global_hotkeys_system(
    mut exit_event_writer: EventWriter<AppExit>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.pressed(KeyCode::KeyQ) {
        exit_event_writer.send_default();
    }
}
