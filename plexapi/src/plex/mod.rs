pub mod account;
pub mod library;
pub mod playlist;
pub mod types;
pub mod media;
pub mod client;
pub mod session;

pub mod settings {
    pub const PROJECT: &'static str = env!("CARGO_PKG_NAME");
    pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    pub const X_PLEX_CONTAINER_SIZE: i8 = 100;
}


pub mod prelude {
    pub use super::account::*;
    pub use super::library::*;
    pub use super::playlist::*;
    pub use super::types::*;
    pub use super::client::*;
    pub use super::session::*;
}