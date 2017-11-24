use std::result::Result;
use std::sync::{Arc, Mutex};

pub struct SessionState {}

pub struct Session {
    session_state: Arc<Mutex<SessionState>>
}