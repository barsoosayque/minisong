use iocraft::prelude::*;

const SPINNER_FRAMES: [&str; 10] = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

#[component]
pub fn Spinner(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let mut frame = hooks.use_state(|| 0);

    hooks.use_future(async move {
        loop {
            smol::Timer::after(std::time::Duration::from_millis(100)).await;
            frame.set((frame.get() + 1) % SPINNER_FRAMES.len());
        }
    });

    element! {
        Fragment {
            Text(content: SPINNER_FRAMES[frame.get()], color: Color::Yellow)
        }
    }
}

#[derive(Default, Props)]
pub struct ProgressBarProps {
    pub amount: f32,
    pub handler: Handler<'static, f32>,
}

#[component]
pub fn ProgressBar<'a>(
    mut hooks: Hooks,
    props: &mut ProgressBarProps,
) -> impl Into<AnyElement<'a>> {
    let rect = hooks.use_component_rect().get().unwrap_or_default();
    let width = rect.right - rect.left;
    let (full_width, _) = hooks.use_terminal_size();

    hooks.use_local_terminal_events({
        let mut handler = props.handler.take();
        move |event| match event {
            TerminalEvent::FullscreenMouse(FullscreenMouseEvent {
                column,
                kind: MouseEventKind::Down(_),
                ..
            }) => handler(column as f32 / width as f32),
            _ => {},
        }
    });

    element! {
        View(width: Percent(100.0), height: 1, overflow: Overflow::Hidden) {
            View(position: Position::Absolute) {
                Text(content: "·".repeat(full_width as usize), weight: Weight::Light)
            }
            View(width: Percent(100.0 * props.amount), position: Position::Absolute) {
                Text(content: "—".repeat(full_width as usize), color: Color::Magenta)
            }
        }
    }
}

#[derive(Debug, Default, Props)]
pub struct DurationProps {
    pub duration: chrono::Duration,
    pub weight: Weight,
}

#[component]
pub fn Duration(props: &DurationProps) -> impl Into<AnyElement<'static>> {
    match (props.duration.num_hours(), props.duration.num_minutes(), props.duration.num_seconds()) {
        (0, 0, seconds) => {
            element! { Text(weight: props.weight, content: format!("00:{seconds:-02}")) }
        },
        (0, minutes, seconds) => {
            let seconds = seconds - minutes * 60;
            element! { Text(weight: props.weight, content: format!("{minutes:-02}:{seconds:-02}")) }
        },
        (hours, minutes, seconds) => {
            let minutes = minutes - hours * 60;
            let seconds = seconds - minutes * 60;
            element! { Text(weight: props.weight, content: format!("{hours:-02}:{minutes:-02}:{seconds:-02}")) }
        },
    }
}
