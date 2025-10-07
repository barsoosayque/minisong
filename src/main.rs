use smol::future::FutureExt;
use std::sync::OnceLock;
use tracing_error::ErrorLayer;
use tracing_subscriber::{self, layer::SubscriberExt, util::SubscriberInitExt};

mod app;
mod components;
mod mpd;
mod status;
mod task;

static PANIC: OnceLock<String> = OnceLock::new();

fn main() {
    let log_file = std::fs::File::create("log").expect("Log file created");
    let file_subscriber = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_writer(log_file)
        .with_target(false)
        .with_ansi(false)
        .with_level(true);
    tracing_subscriber::registry().with(file_subscriber).with(ErrorLayer::default()).init();

    std::panic::set_hook(Box::new(move |info| {
        let _ = PANIC.set(info.to_string());
    }));

    let result = smol::block_on(std::panic::AssertUnwindSafe(app::run()).catch_unwind());
    match result {
        Ok(Err(app_error)) => println!("{app_error}"),
        Err(_panic) => println!("{}", PANIC.get().map(AsRef::as_ref).unwrap_or("Unknown error !")),
        _ => {},
    }
}
