use std::sync::{Arc, Mutex};

pub mod session;

use self::session::{Session};

pub struct Client {
    config: ClientConfig,
    sessions: Vec<Arc<Mutex<Session>>>
}


pub struct ClientConfig {}