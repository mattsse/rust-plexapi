use std::sync::{Arc, Mutex};
use super::session::*;

#[derive(Debug, PartialEq, Clone)]
pub struct ClientConfig {}

impl ClientConfig {
    pub fn new() -> Self { ClientConfig {} }
}

#[derive(Debug, Clone)]
pub struct Client {
    config: ClientConfig,
    sessions: Vec<Arc<Mutex<Session>>>
}

impl Client {
    pub fn new(config: ClientConfig) -> Self {
        Client {
            config,
            sessions: Vec::new()
        }
    }

    pub fn new_session<T:>(&mut self, session_info: T) -> Result<Arc<Mutex<Session>>, String>
        where T: Into<SessionInfo> {
        let session_info = session_info.into();
        let session = Arc::new(Mutex::new(Session::new(session_info)));
        self.sessions.push(session.clone());
        Ok(session)

    }
}