#[macro_use] extern crate log;
extern crate env_logger;
extern crate dotenv;
extern crate reqwest;
#[macro_use] extern crate hyper;
extern crate url;
extern crate uname;
extern crate tokio_service;
extern crate tokio_core;
extern crate futures;

use hyper::header::Headers;

pub mod server;
pub mod plex;
pub mod http;

pub mod prelude {
    pub use super::plex::prelude::*;
    pub use super::server::*;
}