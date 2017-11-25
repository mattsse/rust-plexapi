use std::result::Result;
use std::borrow::Cow;
use auth::Authentication;
use std::sync::{Arc, Mutex};

pub struct SessionInfo {
    pub auth: Authentication,
    // server endpoint url
    pub url: String,
}

impl SessionInfo {
    pub fn new(auth: Authentication, url: String) -> Self {
        SessionInfo {
            auth,
            url
        }
    }
}

pub struct SessionState {}

impl SessionState {
    pub fn new() -> Self {
        SessionState {}
    }
}

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