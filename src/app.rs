use std::sync::Arc;

use crate::{
    bar,
    components::Spinner,
    mpd::MpdClient,
    playback,
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

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum AppTab {
    #[default]
    Playback,
    Queue,
}

impl AppTab {
    fn text(&self) -> &'static str {
        match self {
            AppTab::Playback => "Playback",
            AppTab::Queue => "Queue",
        }
    }
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
                    bar::PlayerStatusBar()
                    AppTabs()
                }
            }
        }
        .into_any(),
        TaskStatus::InProgress => element! {
            View(
                width, height,
                gap: 1,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Row
            ) {
                Spinner()
                Text(content: "Connecting..")
            }
        }
        .into_any(),
    }
}

#[component]
fn AppTabs(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let mut tab = hooks.use_state_default::<AppTab>();
    hooks.use_terminal_events({
        move |event| match event {
            TerminalEvent::Key(KeyEvent { code, kind, .. }) if kind != KeyEventKind::Release => {
                match code {
                    KeyCode::Char('1') => {
                        tab.set(AppTab::Playback);
                    },
                    KeyCode::Char('2') => {
                        tab.set(AppTab::Queue);
                    },
                    _ => {},
                }
            },
            _ => {},
        }
    });

    element! {
        Fragment(
        ) {
            View(
                width: Percent(100.0),
                height: Percent(100.0),
                padding: 1,
            ) {
                #(match tab.get() {
                    AppTab::Playback => element! { playback::PlaybackScreen() }.into_any(),
                    AppTab::Queue => element! {
                        View(
                            width: Percent(100.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                        ) {
                            Text(content: "TODO: Queue Tab")
                        }
                    }.into_any(),
                })
            }
            View(
                width: Percent(100.0),
                height: 1,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                background_color: Color::Black,
                padding_left: 1,
                padding_right: 1,
            ) {
                SelectableTab(current_tab: tab, self_tab: AppTab::Playback)
                SelectableTab(current_tab: tab, self_tab: AppTab::Queue)
            }
        }
    }
}

#[derive(Default, Props)]
struct SelectableTabProps {
    current_tab: Option<State<AppTab>>,
    self_tab: AppTab,
}

#[component]
fn SelectableTab(_hooks: Hooks, props: &SelectableTabProps) -> impl Into<AnyElement<'static>> {
    let is_selected = props.current_tab.is_some_and(|tab| tab == props.self_tab);
    let color = if is_selected { Color::White } else { Color::Grey };

    let current_tab = props.current_tab.clone();
    let self_tab = props.self_tab.clone();
    let on_click = move |_| {
        let Some(mut current_tab) = current_tab else {
            return;
        };

        current_tab.set(self_tab);
    };

    element! {
        Fragment {
            Button(handler: on_click) {
                Text(color: color, content: props.self_tab.text())
            }
        }
    }
}
