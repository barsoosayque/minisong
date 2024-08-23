use std::{sync::Arc, time::Duration};

use anyhow::Context;
use bevy::prelude::*;
use bevy_tokio_tasks::TokioTasksRuntime;
use mpd_client::{
    commands::{self, SongId},
    responses::{PlayState, SongInQueue},
    Client,
};
use tokio::net::{TcpStream, ToSocketAddrs};

/// Plugin for `[MpdClient]` and its associated resources.
pub struct MpdPlugin;

impl Plugin for MpdPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            First,
            (mpd_client_updates_system, mpd_update_album_art_system)
                .run_if(resource_exists::<MpdClient>),
        );
    }
}

/// Connection to the MPD server and a point of MPD request's configuration.
///
/// If this resource exist, it will update info about MPD's status once in
/// a while (either by a 500ms timer, or as soon as possible if requested by
/// calling `[MpdClient::request_update]`) and insert different associated
/// resources with actual info:
/// - `[MpdPlayerState]`: general info
/// - `[MpdCurrentTrack]`: current track info
#[derive(Resource, Debug)]
pub struct MpdClient {
    client: Arc<Client>,
    update_timer: Timer,
    update_required: bool,
}

impl MpdClient {
    /// Request a force update (without waiting for a timeout, as soon as possible).
    pub fn request_update(&mut self) {
        self.update_required = true;
    }
}

/// General info of MPD player.
#[derive(Resource, Debug, PartialEq)]
pub struct MpdPlayerState {
    pub volume: f32,
    pub repeat: bool,
    pub random: bool,
    pub consume: bool,
}

/// Info about the current track.
#[derive(Component, Debug, PartialEq)]
pub struct MpdCurrentTrack {
    pub cur_time: Duration,
    pub total_time: Duration,
    pub state: PlayState,
}

/// Album art of the current track.
#[derive(Component, Debug, PartialEq)]
pub struct MpdAlbumArt {
    pub data: Vec<u8>,
}

/// Song in MPD database.
#[derive(Component, Debug, PartialEq, Eq)]
pub struct MpdTrack {
    pub id: SongId,
    pub url: String,
    pub title: String,
    pub album: String,
    pub artists: Vec<String>,
}

impl MpdTrack {
    fn from_song_in_queue(song: SongInQueue) -> Self {
        Self {
            id: song.id,
            url: song.song.url.clone(),
            title: song.song.title().unwrap_or("none").to_owned(),
            album: song.song.album().unwrap_or("none").to_owned(),
            artists: song.song.artists().into_iter().cloned().collect(),
        }
    }
}

impl MpdClient {
    /// Try to establish a connection to `addr` (with optional `password`).
    pub async fn try_connect(
        addr: impl ToSocketAddrs,
        password: Option<String>,
    ) -> anyhow::Result<Self> {
        let connection = TcpStream::connect(addr)
            .await
            .context("Creating a Tcp connection failed. Wrong host/port ?")?;

        let (client, _events) = if let Some(password) = password {
            Client::connect_with_password(connection, &password)
                .await
                .context("Can't connect to an MPD server with password")?
        } else {
            Client::connect(connection).await.context("Can't connect to an MPD server")?
        };

        Ok(Self {
            client: Arc::new(client),
            update_required: true,
            update_timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
        })
    }
}

/// System to update MPD info if it was requested or by timer.
fn mpd_client_updates_system(
    time: Res<Time>,
    mut client: ResMut<MpdClient>,
    runtime: ResMut<TokioTasksRuntime>,
) {
    let client = client.bypass_change_detection();
    client.update_timer.tick(time.delta());
    if !client.update_required && !client.update_timer.just_finished() {
        return;
    }
    client.update_required = false;

    let client = client.client.clone();
    runtime.spawn_background_task(move |mut ctx| async move {
        let status = client.command(commands::Status).await.unwrap();

        let player_state = MpdPlayerState {
            volume: status.volume as f32 / 100.0,
            repeat: status.repeat,
            random: status.random,
            consume: status.consume,
        };

        let current_track_bundle = if let Some((_, _current_song_id)) = status.current_song {
            let current_song = client.command(commands::CurrentSong).await.unwrap();
            match (current_song, status.elapsed, status.duration) {
                (Some(song), Some(cur_time), Some(total_time)) => Some((
                    MpdTrack::from_song_in_queue(song),
                    MpdCurrentTrack { cur_time, total_time, state: status.state },
                )),
                _ => None,
            }
        } else {
            None
        };

        ctx.run_on_main_thread(move |ctx| {
            if ctx.world.get_resource() != Some(&player_state) {
                ctx.world.insert_resource(player_state);
            }

            if let Some((new_track, new_current)) = current_track_bundle {
                let mut current_track_query =
                    ctx.world.query::<(Entity, &MpdTrack, &MpdCurrentTrack)>();
                let (entity, update_track, update_current) =
                    match current_track_query.get_single(&ctx.world) {
                        Ok((entity, track, current)) => {
                            (Some(entity), &new_track != track, &new_current != current)
                        },
                        Err(_) => (None, true, true),
                    };

                if !update_track && !update_current {
                    return;
                }

                let entity = entity.unwrap_or_else(|| ctx.world.spawn_empty().id());
                let mut entity_mut = ctx.world.entity_mut(entity);
                if update_track {
                    entity_mut.insert(new_track);
                }
                if update_current {
                    entity_mut.insert(new_current);
                }
            } else {
                let mut current_track_query =
                    ctx.world.query_filtered::<Entity, With<MpdCurrentTrack>>();
                let Ok(entity) = current_track_query.get_single(&ctx.world) else {
                    return;
                };
                ctx.world.despawn(entity);
            }
        })
        .await;
    });
}

/// System to update album art of the current track.
fn mpd_update_album_art_system(
    client: Res<MpdClient>,
    runtime: ResMut<TokioTasksRuntime>,
    current_track_query: Query<(Entity, &MpdTrack), (With<MpdCurrentTrack>, Changed<MpdTrack>)>,
) {
    let Ok((entity, track)) = current_track_query.get_single() else {
        return;
    };

    let client = client.client.clone();
    let url = track.url.clone();
    runtime.spawn_background_task(move |mut ctx| async move {
        let Some((data, _)) = client
            .album_art(&url)
            .await
            .inspect_err(|err| error!("Error getting an album art for '{url}': {err}"))
            .ok()
            .flatten()
        else {
            return;
        };

        ctx.run_on_main_thread(move |ctx| {
            let data = data.freeze().into();
            ctx.world.entity_mut(entity).insert(MpdAlbumArt { data });
        })
        .await;
    });
}
