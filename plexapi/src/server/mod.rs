use std::io;
use std::borrow::Cow;
use std::sync::{Arc, Mutex};

use plex::prelude::*;
/// Main Entrypoint to communicate with the actual Plex Server
pub struct Server {
    pub auth: Login,
    pub base_url: String,
    pub clients: Vec<Arc<Mutex<Client>>>
}

impl Server {
    pub fn new(auth: Login, url: String) -> Self {
        Server {
            auth,
            base_url: url,
            clients: vec![]
        }
    }
}