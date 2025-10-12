use std::sync::Arc;

use crate::{
    components::Spinner,
    mpd::MpdClient,
    status,
    task::{TaskStatus, UseTask},
};
use clap::Parser;
use iocraft::prelude::*;

/// App configuration, created via command line arguments.
#[derive(Debug, Parser, Clone)]
#[command(version, about)]
pub struct Config {
    #[arg(long, default_value_t = String::from("localhost"))]
    pub host: String,
    #[arg(long, default_value_t = 8080)]
    pub port: u16,
    #[arg(long)]
    pub password: Option<String>,
}

/// Context for the whole app, set in [`Minisong`].
#[derive(Debug)]
pub struct AppContext {
    pub mpd: MpdClient,
}

#[derive(Debug, Clone)]
struct RunContext {
    config: Arc<Config>,
}

/// Main app entry point.
pub async fn run() -> eyre::Result<()> {
    let config = Config::try_parse()?;

    element!(ContextProvider(value: Context::owned(RunContext { config: Arc::new(config) })) {
        Minisong()
    })
    .fullscreen()
    .await?;

    Ok(())
}

/// Main app UI. It defines app context, app-wide key-bindings and sets terminal window.
#[component]
pub fn Minisong(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let (width, height) = hooks.use_terminal_size();
    let mut system = hooks.use_context_mut::<SystemContext>();

    let mut should_exit = hooks.use_state(|| false);
    hooks.use_terminal_events({
        move |event| match event {
            TerminalEvent::Key(KeyEvent { code, kind, .. }) if kind != KeyEventKind::Release => {
                match code {
                    KeyCode::Char('q') => should_exit.set(true),
                    _ => {},
                }
            },
            _ => {},
        }
    });
    if should_exit.get() {
        system.exit();
    }

    let ctx = hooks.use_context::<RunContext>().clone();
    let client_task = hooks.use_task(move || {
        MpdClient::new((ctx.config.host.clone(), ctx.config.port), ctx.config.password.clone())
    });

    let status = client_task.status();
    match &*status {
        TaskStatus::Error(err) => panic!("Connecting to MPD: {}", err),
        TaskStatus::Done(mpd) => element! {
            ContextProvider(value: Context::owned(AppContext { mpd: mpd.clone() })) {
                View(width, height, flex_direction: FlexDirection::Column) {
                    status::PlayerStatusBar()
                    status::CurrentSongScreen()
                }
            }
        }
        .into_any(),
        TaskStatus::InProgress => element! {
            View(
                width, height,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Row
            ) {
                Spinner()
                Text(content: " Connecting..")
            }
        }
        .into_any(),
    }
}
