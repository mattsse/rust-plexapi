use std::sync::{Arc, Mutex};

pub mod session;

use self::session::*;

pub struct ClientConfig {}

impl ClientConfig {
    pub fn new() -> Self { ClientConfig {} }
}

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
        Err(format!(""))
    }
}

pub mod prelude {
    pub use super::*;
    pub use super::session::*;
}