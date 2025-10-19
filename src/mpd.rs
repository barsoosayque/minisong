use std::{
    net::ToSocketAddrs,
    ops::{Deref, DerefMut},
    sync::{Arc, RwLock, RwLockWriteGuard},
};

use event_listener::Event;
use mpd::Client;

/// Connection to the MPD server and a point of MPD request's configuration.
///
/// Can be shared between threads and cloned, but uses RwLock inside to manage the client.
#[derive(Debug, Clone)]
pub struct MpdClient {
    client: Arc<RwLock<Client>>,
    event: Arc<Event>,
}

impl MpdClient {
    /// Create a new [`MpdClient`] and connect it to `addr` with optional `password`.
    pub fn new(addr: impl ToSocketAddrs, password: Option<impl AsRef<str>>) -> eyre::Result<Self> {
        let mut client = Client::connect(addr)?;
        if let Some(password) = password {
            client.login(password.as_ref())?;
        }

        Ok(Self { client: Arc::new(RwLock::new(client)), event: Arc::new(Event::new()) })
    }

    /// Wait for write access to the [`MpdClient`] and bind it to the current thread.
    pub async fn bind(&mut self) -> MpdGuard<'_> {
        MpdGuard { guard: self.client.write().unwrap(), event: None }
    }

    /// Wait for write access to the [`MpdClient`], bind it to the current thread, and
    /// send an update to every [`MpdClient::wait_for_update`] when the access ends.
    pub async fn bind_then_notify(&mut self) -> MpdGuard<'_> {
        MpdGuard { guard: self.client.write().unwrap(), event: Some(self.event.clone()) }
    }

    /// Notify every [`MpdClient::wait_for_update`].
    pub async fn notify_update(&self) {
        self.event.notify(usize::MAX);
    }

    /// Wait for updates sent by [`MpdClient::notify_update`] or on drop
    /// of [`MpdGuard`] created by [`MpdClient::client_with_notify`].
    pub async fn wait_for_update(&self) {
        let listener = self.event.listen();
        listener.await;
    }
}

/// RwGuard for [`MpdClient`], which binds the client for the current
/// thread and notifies subscribers for UI updates on drop if enabled.
pub struct MpdGuard<'a> {
    guard: RwLockWriteGuard<'a, Client>,
    event: Option<Arc<Event>>,
}

impl Deref for MpdGuard<'_> {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &*self.guard
    }
}

impl DerefMut for MpdGuard<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.guard
    }
}

impl Drop for MpdGuard<'_> {
    fn drop(&mut self) {
        if let Some(event) = self.event.clone() {
            smol::spawn(async move {
                event.notify(usize::MAX);
            })
            .detach();
        }
    }
}
