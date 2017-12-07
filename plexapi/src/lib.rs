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

#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_xml_rs;

use hyper::header::Headers;

pub mod server;
pub mod plex;
pub mod http;
pub mod client;
//pub mod prelude;

pub mod prelude {
    pub use super::plex::prelude::*;
    pub use super::server::*;
}