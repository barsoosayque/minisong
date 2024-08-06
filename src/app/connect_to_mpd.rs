use crate::{
    mpd::MpdClient,
    ui::{Align, DespawnUI, Label, Throbber, WidgetBundle},
    Config,
};
use bevy::prelude::*;
use bevy_tokio_tasks::TokioTasksRuntime;
use ratatui::style::{Color, Style};
use ratatui_macros::{span, text};

use super::AppState;

/// Plugin for running `[AppState::ConnectToMpd]`.
pub struct ConnectToMpdStatePlugin;

impl Plugin for ConnectToMpdStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::ConnectToMpd), connect_to_mpd_startup_system);
    }
}

/// System to spawn a task to establish MPD connection, and spawn UI.
fn connect_to_mpd_startup_system(
    mut commands: Commands,
    config: Res<Config>,
    runtime: ResMut<TokioTasksRuntime>,
) {
    let config = config.clone();
    runtime.spawn_background_task(move |mut ctx| async move {
        info!("Trying to connect to MPD server...");
        let result = MpdClient::try_connect((config.host, config.port), config.password).await;

        ctx.run_on_main_thread(move |ctx| {
            match result {
                Ok(client) => {
                    info!("Successfully connected to MPD !");
                    ctx.world.insert_resource(client);
                    ctx.world.resource_mut::<NextState<AppState>>().set(AppState::Client);
                },
                Err(error) => {
                    error!("Error while connecting to MPD: {error} !");
                    let mut commands = ctx.world.commands();
                    commands.add(DespawnUI);
                    let text = text![
                        "Error while connecting to MPD:",
                        span![Style::new().fg(Color::Red); error.to_string()],
                    ]
                    .centered();
                    commands.spawn(
                        WidgetBundle::from(Label::new(text))
                            .align_horizontal(Align::Center)
                            .align_vertical(Align::Center),
                    );
                },
            };
        })
        .await;
    });

    commands.spawn(
        WidgetBundle::from(Throbber::new("Trying to connect to an MPD server..."))
            .align_horizontal(Align::Center)
            .align_vertical(Align::Center),
    );
}
