use iocraft::prelude::*;

use crate::app::AppContext;

/// Global MPD status state for [`PlayerStatusBar`].
#[derive(Default)]
struct PlayerStatus {
    volume: f32,
    queue: (usize, usize),
    repeat: bool,
    random: bool,
    single: bool,
    consume: bool,
}

/// Actions for [`PlayerStatusBar`].
#[derive(Debug, Clone, Copy)]
enum Action {
    ChangeVolume(f32),
    ToggleRepeat,
    ToggleRandom,
    ToggleSingle,
    ToggleConsume,
}

#[component]
pub fn PlayerStatusBar(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let ctx = hooks.use_context::<AppContext>();

    let mut mpd_status: State<PlayerStatus> = hooks.use_state_default();
    let mut mpd = ctx.mpd.clone();
    hooks.use_future(async move {
        loop {
            {
                let mut client = mpd.bind().await;
                let status = client.status().unwrap();
                mpd_status.set(PlayerStatus {
                    volume: status.volume as f32 / 100.0,
                    queue: (
                        status.song.map(|song| song.pos as usize + 1).unwrap_or_default(),
                        status.queue_len as usize,
                    ),
                    repeat: status.repeat,
                    random: status.random,
                    single: status.single,
                    consume: status.consume,
                });
            }

            mpd.wait_for_update().await;
        }
    });

    let mpd = ctx.mpd.clone();
    let action = hooks.use_async_handler(move |action: Action| {
        let mut mpd = mpd.clone();
        async move {
            let mut client = mpd.bind_then_notify().await;
            match action {
                Action::ChangeVolume(amount) => {
                    client
                        .volume(
                            ((mpd_status.read().volume + amount) * 100.0).clamp(0.0, i8::MAX as f32)
                                as i8,
                        )
                        .unwrap();
                },
                Action::ToggleRepeat => client.repeat(!mpd_status.read().repeat).unwrap(),
                Action::ToggleRandom => client.random(!mpd_status.read().random).unwrap(),
                Action::ToggleSingle => client.single(!mpd_status.read().single).unwrap(),
                Action::ToggleConsume => client.consume(!mpd_status.read().consume).unwrap(),
            }
        }
    });

    element! {
        View(
            width: Percent(100.0),
            height: 1,
            padding_left: 1,
            padding_right: 1,
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
        ) {
            VolumeSlider(handler: action.clone(), volume: mpd_status.read().volume)
            Text(content: format!("≡ {} / {}", mpd_status.read().queue.0, mpd_status.read().queue.1))
            View(
                flex_direction: FlexDirection::Row,
                gap: 1,
            ) {
                Button(handler: action.bind(Action::ToggleRepeat)) {
                    Text(content: if mpd_status.read().repeat { "🔁" } else { "·" })
                }
                Button(handler: action.bind(Action::ToggleRandom)) {
                    Text(content: if mpd_status.read().random { "🔀" } else { "·" })
                }
                Button(handler: action.bind(Action::ToggleSingle)) {
                    Text(content: if mpd_status.read().single { "🔂" } else { "·" })
                }
                Button(handler: action.bind(Action::ToggleConsume)) {
                    Text(content: if mpd_status.read().consume { "🗑️" } else { "·" })
                }
            }
        }
    }
}

#[derive(Default, Props)]
struct VolumeSliderProps {
    handler: RefHandler<Action>,
    volume: f32,
}

#[component]
fn VolumeSlider(mut hooks: Hooks, props: &VolumeSliderProps) -> impl Into<AnyElement<'static>> {
    hooks.use_local_terminal_events({
        let handler = props.handler.clone();
        move |event| match event {
            TerminalEvent::FullscreenMouse(FullscreenMouseEvent { kind, .. }) => match kind {
                MouseEventKind::ScrollUp => handler(Action::ChangeVolume(0.05)),
                MouseEventKind::ScrollDown => handler(Action::ChangeVolume(-0.05)),
                _ => {},
            },
            _ => {},
        }
    });

    element! {
        Text(content: {
            let volume = props.volume * 100.0;
            let icon = if volume <= 33.0 {
                "🔈"
            } else if volume <= 66.0 {
                "🔉"
            } else {
                "🔊"
            };
            format!("{icon} {volume:.0}%")
        })
    }
}
