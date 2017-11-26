use std::result::Result;
use std::borrow::Cow;
use std::sync::{Arc, Mutex};

pub use super::types::*;

// make generic T : Into<PlexToken>
#[derive(Debug, PartialEq, Clone)]
pub struct SessionInfo {
    pub auth_token: PlexToken,
    // server endpoint url
    pub url: String
}

impl SessionInfo {
    pub fn new(auth_token: PlexToken, url: String) -> Self {
        SessionInfo {
            auth_token,
            url
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SessionState {}

impl SessionState {
    pub fn new() -> Self {
        SessionState {}
    }
}
#[derive(Debug, Clone)]
pub struct Session {
    pub session_info: SessionInfo,
    session_state: Arc<Mutex<SessionState>>
}

impl Session {
    pub fn new(session_info: SessionInfo) -> Self {
        let session_state = Arc::new(Mutex::new(SessionState::new()));
        Session {
            session_info,
            session_state
        }
    }
}