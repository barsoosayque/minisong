use std::{net::ToSocketAddrs, sync::Arc};

use mpd::Client;

/// Connection to the MPD server and a point of MPD request's configuration.
///
/// Can be shared between threads and cloned, but uses RwLock inside to manage the client.
#[derive(Debug, Clone)]
pub struct MpdClient {
    client: Arc<smol::lock::RwLock<Client>>,
}

impl MpdClient {
    pub fn new(addr: impl ToSocketAddrs, password: Option<impl AsRef<str>>) -> eyre::Result<Self> {
        let mut client = Client::connect(addr)?;
        if let Some(password) = password {
            client.login(password.as_ref())?;
        }
        Ok(Self { client: Arc::new(smol::lock::RwLock::new(client)) })
    }

    pub async fn client(&self) -> smol::lock::RwLockWriteGuardArc<Client> {
        self.client.write_arc().await
    }
}
