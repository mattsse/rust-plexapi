#[macro_use] extern crate log;
extern crate env_logger;
extern crate dotenv;

pub mod client;
pub mod server;
pub mod auth;
pub mod plex;

pub mod prelude {
    pub use super::auth::*;
    pub use super::plex::*;
    pub use super::client::prelude::*;
    pub use super::server::*;
}