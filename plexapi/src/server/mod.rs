use std::io;
use std::borrow::Cow;
use auth::Authentication;
use std::sync::{Arc, Mutex};

use client::Client;

/// Main Entrypoint to communicate with the actual Plex Server
pub struct Server {
    pub auth: Authentication,
    pub base_url: String,
    pub clients: Vec<Arc<Mutex<Client>>>
}

impl Server {
    pub fn new(auth: Authentication, url: String) -> Self {
        Server {
            auth,
            base_url: url,
            clients: vec![]
        }
    }
}