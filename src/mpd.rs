use std::{
    net::ToSocketAddrs,
    ops::{Deref, DerefMut},
    sync::Arc,
};

use mpd::Client;

/// Connection to the MPD server and a point of MPD request's configuration.
///
/// Can be shared between threads and cloned, but uses RwLock inside to manage the client.
#[derive(Debug, Clone)]
pub struct MpdClient {
    client: Arc<smol::lock::RwLock<Client>>,
    notifier: async_broadcast::Sender<()>,
    subscriber: async_broadcast::Receiver<()>,
}

impl MpdClient {
    pub fn new(addr: impl ToSocketAddrs, password: Option<impl AsRef<str>>) -> eyre::Result<Self> {
        let mut client = Client::connect(addr)?;
        if let Some(password) = password {
            client.login(password.as_ref())?;
        }

        let (mut notifier, subscriber) = async_broadcast::broadcast(4);
        notifier.set_overflow(true);

        Ok(Self { client: Arc::new(smol::lock::RwLock::new(client)), notifier, subscriber })
    }

    pub async fn client(&self) -> MpdGuard {
        MpdGuard { guard: self.client.write_arc().await, notifier: None }
    }

    pub async fn client_with_notify(&self) -> MpdGuard {
        MpdGuard { guard: self.client.write_arc().await, notifier: Some(self.notifier.clone()) }
    }

    pub async fn notify_update(&mut self) {
        self.notifier.broadcast_direct(()).await.unwrap();
    }

    pub async fn wait_an_update(&mut self) {
        self.subscriber.recv().await.unwrap();
    }
}

/// Rw guard for [`MpdClient`], which binds the client for the current
/// thread and notifies subscribers for UI updates on drop if enabled.
pub struct MpdGuard {
    guard: smol::lock::RwLockWriteGuardArc<Client>,
    notifier: Option<async_broadcast::Sender<()>>,
}

impl Deref for MpdGuard {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &*self.guard
    }
}

impl DerefMut for MpdGuard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.guard
    }
}

impl Drop for MpdGuard {
    fn drop(&mut self) {
        let Some(notifier) = self.notifier.clone() else {
            return;
        };
        notifier.broadcast_blocking(()).unwrap();
    }
}
