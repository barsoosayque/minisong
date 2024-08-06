use std::{fmt::Display, time::Duration};

use crate::{
    mpd::MpdCurrentTrack,
    ui::{Align, Block, Throbber, WidgetAppExt, WidgetBundle, WidgetDrawContext},
};
use bevy::prelude::*;
use chrono::Timelike;
use mpd_client::responses::PlayState;
use ratatui::{
    layout::{Constraint, Direction},
    style::{Color, Style, Stylize},
    symbols,
    widgets::LineGauge,
};
use ratatui_macros::{horizontal, span, text, vertical};

use super::AppState;

/// Plugin for running `[AppState::Client]`.
pub struct ClientStatePlugin;

impl Plugin for ClientStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Client), client_startup_system);

        app.register_widget::<CurrentTrack, _>(current_track_draw_system);
    }
}

/// Tag for the current track playback widget.
#[derive(Component, Default, Clone, Copy)]
pub struct CurrentTrack;

/// System to spawn all client UI.
pub fn client_startup_system(mut commands: Commands) {
    commands
        .spawn(WidgetBundle::<Block>::new().content_direction(Direction::Horizontal))
        .with_children(|children| {
            children.spawn(
                WidgetBundle::from(Throbber::new("Loading album art.."))
                    .align_horizontal(Align::Center)
                    .align_vertical(Align::Center)
                    .constraint(Constraint::Percentage(50)),
            );
            children.spawn((WidgetBundle::<CurrentTrack>::new()
                .align_horizontal(Align::Center)
                .align_vertical(Align::Center)
                .constraint(Constraint::Percentage(50)),));
        });
}

/// System to draw current track info.
fn current_track_draw_system(
    In(mut ctx): In<WidgetDrawContext>,
    current_track: Option<Res<MpdCurrentTrack>>,
) {
    let height = if current_track.is_some() { 4 } else { 1 };
    ctx.draw_sized((u16::MAX, height), |frame, rect| {
        let Some(current_track) = current_track else {
            frame.render_widget(
                text![span![Style::new().italic(); "No track is playing"]].centered(),
                rect,
            );
            return;
        };

        let [title_area, other_info_area, progress_area] = vertical![==2, ==1, ==1].areas(rect);

        let title_widget = text![span![Style::new().bold(); current_track.track.title]];
        frame.render_widget(title_widget, title_area);

        let [album_area, artists_area] = horizontal![==50%, ==50%].areas(other_info_area);

        let album_widget = text![span![Style::new().italic(); current_track.track.album]];
        frame.render_widget(album_widget, album_area);

        let artists_widget =
            text![span![Style::new().italic(); current_track.track.artists.join(", ")]]
                .right_aligned();
        frame.render_widget(artists_widget, artists_area);

        let time_widget = {
            fn format_time(duration: Duration) -> impl Display {
                let chrono_time =
                    chrono::DateTime::from_timestamp_millis(duration.as_millis() as i64).unwrap();
                if chrono_time.hour() > 0 {
                    chrono_time.format("%H:%M:%S")
                } else {
                    chrono_time.format("%M:%S")
                }
            }

            let current_time = format_time(current_track.cur_time);
            let total_time = format_time(current_track.total_time);

            text![span![Style::new().italic().fg(Color::Yellow); " {current_time}/{total_time}"]]
        };

        let progress_bar_widget = {
            let ratio = (current_track.cur_time.as_secs_f64()
                / current_track.total_time.as_secs_f64())
            .clamp(0.0, 1.0);

            LineGauge::default()
                .filled_style(Style::default().fg(Color::Magenta))
                .ratio(ratio)
                .line_set(symbols::line::THICK)
                .label(span![
                    Style::new().fg(Color::Green);
                    match current_track.state {
                        PlayState::Stopped => "⏹",
                        PlayState::Playing => "⏵",
                        PlayState::Paused => "⏸",
                    }
                ])
        };

        let [progress_bar_area, time_area] =
            horizontal![==100%, ==time_widget.width() as u16].areas(progress_area);

        frame.render_widget(progress_bar_widget, progress_bar_area);
        frame.render_widget(time_widget, time_area);
    });
}

// From times before I had this crazy idea to marry bevy and raratui,
// here lies my old minisong ui, which I will totally port over.

//     fn draw_header(&mut self, frame: &mut Frame, area: Rect) -> anyhow::Result<()> {
//         let [left_area, right_area] =
//             *Layout::horizontal([Constraint::Percentage(80), Constraint::Percentage(20)])
//                 .split(area)
//         else {
//             anyhow::bail!("Should never happen");
//         };

//         let widget = Tabs::new(TABS_STR).select(self.tab as usize);
//         frame.render_widget(widget, left_area);

//         let widget = Paragraph::new(Line::from(vec![
//             // Span::raw(
//             //     "🔀",
//             // ),
//             // Span::raw(" "),
//             // Span::styled(
//             //     "🔁",
//             //     Style::new().fg(if self.player.repeat { Color::Green } else { Color::Black }),
//             // ),
//             // Span::raw(" "),
//             // Span::styled(
//             //     "💠",
//             //     Style::new().fg(if self.player.consume { Color::Red } else { Color::Black }),
//             // ),
//             // Span::raw(" "),
//             Span::styled(format!("🔊{}%", (self.player.volume * 100.0) as usize), Style::new()),
//         ]))
//         .right_aligned();
//         frame.render_widget(widget, right_area);

//         Ok(())
//     }

//     fn draw_footer(&mut self, frame: &mut Frame, area: Rect) -> anyhow::Result<()> {
//         let text = vec![('Q', "Quit"), ('N', "Next"), ('P', "Prev")]
//             .into_iter()
//             .flat_map(|(hotkey, command)| {
//                 vec![
//                     Span::styled(
//                         format!(" {hotkey} "),
//                         Style::new().bg(Color::DarkGray).fg(Color::Black),
//                     ),
//                     Span::styled(
//                         format!(" {command} "),
//                         Style::new().add_modifier(Modifier::ITALIC),
//                     ),
//                 ]
//             })
//             .collect::<Vec<_>>();
//         let widget = Paragraph::new(Line::from(text)).alignment(Alignment::Center);
//         frame.render_widget(widget, area);

//         Ok(())
//     }

//     fn draw_queue(&mut self, frame: &mut Frame, area: Rect) -> anyhow::Result<()> {
//         let rows = self
//             .queue
//             .q
//             .iter()
//             .map(|track| {
//                 Row::new(vec![
//                     Span::styled(track.artists.join(", "), Style::default().fg(Color::Blue)),
//                     Span::styled(track.album.clone(), Style::default().fg(Color::Magenta)),
//                     Span::raw(track.title.clone()),
//                 ])
//             })
//             .collect::<Vec<_>>();
//         let widths =
//             [Constraint::Percentage(20), Constraint::Percentage(20), Constraint::Percentage(60)];

//         let widget = Table::new(rows, widths).column_spacing(1).header(
//             Row::new(vec!["Artist", "Album", "Title"]).style(Style::new().bold()).underlined(),
//         );

//         frame.render_widget(widget, area);

//         Ok(())
//     }

//     fn draw_playback(&mut self, frame: &mut Frame, area: Rect) -> anyhow::Result<()> {
//         let Self { playback, track, .. } = self;
//         if let Some((playback, track)) = playback.as_mut().zip(track.as_mut()) {
//             let main_layout = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)])
//                 .flex(Flex::Center)
//                 .split(area);

//             let right_layout = Layout::vertical([Constraint::Length(3), Constraint::Length(2)])
//                 .flex(Flex::Center)
//                 .spacing(1)
//                 .split(main_layout[1]);

//             if let Some(album_art) = track.album_art.as_mut() {
//                 frame.render_stateful_widget(StatefulImage::new(None), main_layout[0], album_art);
//             }

//             let format = format_description!("[hour]:[minute]:[second]");
//             frame.render_widget(
//                 Paragraph::new(vec![
//                     Line::from(vec![
//                         Span::raw("Title: "),
//                         Span::styled(&track.title, Style::new().bold()),
//                     ]),
//                     Line::from(vec![
//                         Span::raw("Album: "),
//                         Span::styled(&track.album, Style::new().bold()),
//                     ]),
//                     Line::from(vec![
//                         Span::raw("Artists: "),
//                         Span::styled(track.artists.join(", "), Style::new().bold()),
//                     ]),
//                 ])
//                 .alignment(Alignment::Center),
//                 right_layout[0],
//             );

//             let ratio = (playback.cur_time.as_secs_f64() / playback.total_time.as_secs_f64())
//                 .clamp(0.0, 1.0);

//             let progress_area =
//                 Rect::new(right_layout[1].x, right_layout[1].y, right_layout[1].width, 1);
//             frame.render_widget(
//                 LineGauge::default()
//                     .gauge_style(Style::default().fg(Color::Magenta))
//                     .ratio(ratio)
//                     .line_set(symbols::line::THICK)
//                     .label(Span::styled(
//                         match playback.state {
//                             PlayState::Stopped => "⏹",
//                             PlayState::Playing => "⏵",
//                             PlayState::Paused => "⏸",
//                         },
//                         Style::new().fg(Color::Green),
//                     )),
//                 progress_area,
//             );

//             let playback_status_area =
//                 Rect::new(right_layout[1].x, right_layout[1].y + 1, right_layout[1].width, 1);
//             frame.render_widget(
//                 Paragraph::new(Span::styled(
//                     format!(
//                         "{}/{}",
//                         (Time::MIDNIGHT + playback.cur_time).format(format)?,
//                         (Time::MIDNIGHT + playback.total_time).format(format)?
//                     ),
//                     Style::new().italic().fg(Color::Yellow),
//                 ))
//                 .centered(),
//                 playback_status_area,
//             );
//         } else {
//             frame.render_widget(
//                 Paragraph::new(Span::styled(
//                     "Nothing in queue...",
//                     Style::new().italic().fg(Color::DarkGray),
//                 ))
//                 .centered(),
//                 Layout::default()
//                     .flex(Flex::Center)
//                     .constraints([Constraint::Length(1)])
//                     .split(area)[0],
//             );
//         }

//         Ok(())
//     }

//     fn draw(&mut self, frame: &mut Frame) -> anyhow::Result<()> {
//         let [header_area, main_area, footer_area] = *Layout::vertical([
//             Constraint::Length(1),
//             Constraint::Length(frame.size().height - 2),
//             Constraint::Length(1),
//         ])
//         .spacing(1)
//         .horizontal_margin(1)
//         .split(frame.size()) else {
//             anyhow::bail!("Failed to create a main layout");
//         };

//         self.draw_header(frame, header_area)?;
//         self.draw_footer(frame, footer_area)?;
//         match self.tab {
//             AppTab::Playback => self.draw_playback(frame, main_area)?,
//             AppTab::Queue => self.draw_queue(frame, main_area)?,
//         }

//         Ok(())
//     }

//     async fn handle_event(&mut self, event: Event) -> anyhow::Result<()> {
//         match event {
//             Event::Key(e) if e.kind == KeyEventKind::Press => match e.code {
//                 KeyCode::Char('1') => self.tab = AppTab::Playback,
//                 KeyCode::Char('2') => self.tab = AppTab::Queue,
//                 KeyCode::Char('q') => self.quit = true,
//                 KeyCode::Char('n') => self.client.command(commands::Next).await?,
//                 KeyCode::Char('p') => self.client.command(commands::Previous).await?,
//                 _ => {},
//             },
//             _ => {},
//         };

//         Ok(())
//     }

//     async fn run(&mut self) -> anyhow::Result<()> {
//         self.startup_ui()?;

//         let backend = CrosstermBackend::new(stdout());
//         let mut terminal = Terminal::new(backend)?;

//         let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
//         let event_task: JoinHandle<anyhow::Result<()>> = tokio::spawn(async move {
//             let tick_rate = Duration::from_millis(100);
//             loop {
//                 if !crossterm::event::poll(tick_rate)? {
//                     continue;
//                 }

//                 match crossterm::event::read()? {
//                     crossterm::event::Event::Key(key) => {
//                         tx.send(Event::Key(key))?;
//                     },
//                     _ => {},
//                 };
//             }
//         });

//         loop {
//             if self.quit {
//                 break;
//             }

//             let sleep = tokio::time::sleep(Duration::from_millis(1000));
//             tokio::pin!(sleep);
//             tokio::select! {
//                 _ = &mut sleep => {}
//                 _ = tokio::signal::ctrl_c() => {
//                     self.quit = true;
//                 }
//                 event = rx.recv() => {
//                     if let Some(event) = event {
//                         self.handle_event(event).await?;
//                     }
//                 }
//             }

//             self.update().await?;

//             terminal.draw(|frame| {
//                 // FIXME error handling
//                 let _ = self.draw(frame);
//             })?;
//         }

//         self.destroy_ui()?;
//         terminal.show_cursor()?;

//         Ok(())
//     }

//     async fn update(&mut self) -> anyhow::Result<()> {
//         match self.tab {
//             AppTab::Playback => self.update_status().await,
//             AppTab::Queue => self.update_queue().await,
//         }
//     }

//     async fn update_status(&mut self) -> anyhow::Result<()> {
//         let status = self.client.command(commands::Status).await?;

//         self.player.volume = status.volume as f32 / 100.0;
//         self.player.repeat = status.repeat;
//         self.player.random = status.random;
//         self.player.consume = status.consume;

//         let Some((_, current_song_id)) = status.current_song else {
//             self.update_track(None).await?;
//             return Ok(());
//         };

//         if self.track.is_none()
//             || self.track.as_ref().is_some_and(|track| track.id != current_song_id)
//         {
//             self.update_track(Some(current_song_id)).await?;
//         }

//         self.playback = match (status.elapsed, status.duration) {
//             (Some(elapsed), Some(duration)) => {
//                 Playback { cur_time: elapsed, total_time: duration, state: status.state }.into()
//             },
//             _ => None,
//         };

//         Ok(())
//     }

//     async fn update_track(&mut self, id: Option<SongId>) -> anyhow::Result<()> {
//         let Some(id) = id else {
//             self.track = None;
//             return Ok(());
//         };
//         let Some(current_song) = self.client.command(commands::CurrentSong).await? else {
//             return Ok(());
//         };

//         let album_art = match self.client.album_art(&current_song.song.url).await? {
//             Some((bytes, _mime)) => {
//                 let dyn_image = image::load_from_memory(&bytes)?;
//                 Some(self.image_picker.new_resize_protocol(dyn_image))
//             },
//             None => None,
//         };

//         self.track = Track { id, album_art, ..Track::from_song_in_queue(current_song) }.into();

//         Ok(())
//     }

//     async fn update_queue(&mut self) -> anyhow::Result<()> {
//         let songs = self.client.command(commands::Queue).await?;

//         self.queue.q = songs.into_iter().map(Track::from_song_in_queue).collect();

//         Ok(())
//     }
