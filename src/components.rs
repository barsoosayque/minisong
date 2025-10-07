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

#[derive(Debug, Default, Props)]
pub struct ProgressBarProps {
    pub amount: f32,
}

#[component]
pub fn ProgressBar(mut hooks: Hooks, props: &ProgressBarProps) -> impl Into<AnyElement<'static>> {
    let (width, _) = hooks.use_terminal_size();
    let amount = props.amount.clamp(0.0, 1.0);

    element! {
        View(width: Percent(100.0), height: 1, flex_direction: FlexDirection::Row) {
            View(width: Percent(100.0 * amount), overflow: Overflow::Hidden) {
                Text(content: "—".repeat(width as usize), color: Color::Magenta)
            }
            View(width: Percent(100.0 - 100.0 * amount), overflow: Overflow::Hidden) {
                Text(content: "·".repeat(width as usize), weight: Weight::Light)
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
