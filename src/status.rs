use iocraft::prelude::*;

use crate::{
    app::AppContext,
    components::{Duration, ProgressBar},
};

struct CurrentSong {
    artist: String,
    title: String,
    elapsed: chrono::Duration,
    duration: chrono::Duration,
    is_paused: bool,
}

pub enum Action {
    Rewind(f32),
    Next,
    Prev,
    Toggle,
}

/// Current MPD status screen.
#[component]
pub fn Status(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let ctx = hooks.use_context::<AppContext>();

    let mut current = hooks.use_state(|| None);
    let mut mpd = ctx.mpd.clone();
    hooks.use_future(async move {
        loop {
            {
                let mut client = mpd.client().await;
                let song = client.currentsong().unwrap();
                let status = client.status().unwrap();

                if let Some(((song, elapsed), duration)) =
                    song.zip(status.elapsed).zip(status.duration)
                {
                    current.set(Some(CurrentSong {
                        artist: song.artist.unwrap_or_default(),
                        title: song.title.unwrap_or_default(),
                        elapsed: chrono::Duration::from_std(elapsed).unwrap(),
                        duration: chrono::Duration::from_std(duration).unwrap(),
                        is_paused: status.state != mpd::State::Play,
                    }));
                } else {
                    current.set(None);
                }
            }

            mpd.wait_an_update().await;
        }
    });

    let mut mpd = ctx.mpd.clone();
    hooks.use_future(async move {
        loop {
            smol::Timer::interval(std::time::Duration::from_millis(500)).await;
            mpd.notify_update().await;
        }
    });

    let mpd = ctx.mpd.clone();
    let change_postion_to = hooks.use_async_handler(move |amount: f32| {
        let mpd = mpd.clone();
        let duration = current.read().as_ref().map(|current| current.duration).unwrap_or_default();
        async move {
            let mut client = mpd.client_with_notify().await;
            client.rewind((duration.as_seconds_f32() * amount) as f64).unwrap();
        }
    });
    let mpd = ctx.mpd.clone();
    let mut key_action = hooks.use_async_handler(move |action: Action| {
        let mpd = mpd.clone();
        async move {
            let mut client = mpd.client_with_notify().await;
            match action {
                Action::Rewind(amount) => {
                    let elapsed =
                        current.read().as_ref().map(|current| current.elapsed).unwrap_or_default();
                    client.rewind((elapsed.as_seconds_f32() + amount) as f64).unwrap();
                },
                Action::Next => client.next().unwrap(),
                Action::Prev => client.prev().unwrap(),
                Action::Toggle => client.toggle_pause().unwrap(),
            }
        }
    });

    hooks.use_terminal_events(move |event| match event {
        TerminalEvent::Key(KeyEvent { code, kind: KeyEventKind::Press, .. }) => match code {
            KeyCode::Char('p') => (key_action)(Action::Toggle),
            KeyCode::Char('>') => (key_action)(Action::Next),
            KeyCode::Char('<') => (key_action)(Action::Prev),
            KeyCode::Left => (key_action)(Action::Rewind(-5.0)),
            KeyCode::Right => (key_action)(Action::Rewind(5.0)),
            _ => {},
        },
        _ => {},
    });

    element! {
        View(
            width: Percent(100.0),
            height: Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
        ) {
            #(match &*current.read() {
                Some(song) => element!{
                    View(
                        width: Percent(66.0),
                        height: Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Column,
                    ) {
                        Text(color: Color::Blue, weight: Weight::Bold, content: &song.artist)
                        Text(color: Color::DarkBlue, decoration: TextDecoration::Underline, content: &song.title)
                        Text()
                        ProgressBar(
                            amount: song.elapsed.as_seconds_f32() / song.duration.as_seconds_f32(),
                            handler: change_postion_to,
                        )
                        View(width: Percent(100.0), justify_content: JustifyContent::SpaceBetween) {
                            View(gap: 1) {
                                Text(weight: Weight::Light, content: if song.is_paused { "⏸" } else { "▶" })
                                Duration(weight: Weight::Light, duration: song.elapsed)
                            }
                            Duration(weight: Weight::Light, duration: song.duration)
                        }
                    }
                }.into_any(),
                None => element!{
                    Text(content: "Nothing is playing...")
                }.into_any(),
            })
        }
    }
}
