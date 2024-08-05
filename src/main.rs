use std::time::Duration;

use bevy_tokio_tasks::TokioTasksPlugin;
use clap::Parser;

mod app;
mod mpd;
mod ui;

use bevy::{app::ScheduleRunnerPlugin, log::LogPlugin, prelude::*};
use tracing_error::ErrorLayer;
use tracing_subscriber::{self, layer::SubscriberExt, util::SubscriberInitExt};

/// App configuration, created via command line arguments.
#[derive(Resource, Debug, Parser, Clone)]
#[command(version, about)]
pub struct Config {
    #[arg(long, default_value_t = String::from("localhost"))]
    host: String,
    #[arg(long, default_value_t = 8080)]
    port: u16,
    #[arg(long)]
    password: Option<String>,
}

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

    App::new()
        .insert_resource(Config::parse())
        .add_plugins(DefaultPlugins.build().disable::<LogPlugin>())
        .add_plugins(TokioTasksPlugin::default())
        .add_plugins((ui::UiPlugin, mpd::MpdPlugin, app::AppPlugin))
        .add_plugins(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(1.0 / 8.0)))
        .run();
}
