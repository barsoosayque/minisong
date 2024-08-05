use bevy::prelude::*;

use super::AppState;

/// Plugin for running `[AppState::Help]`.
pub struct HelpStatePlugin;

impl Plugin for HelpStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Help), help_startup_system);
    }
}

/// System to build `[AppState::Help]` UI.
pub fn help_startup_system(mut _commands: Commands) {
    // TODO: help table
}
