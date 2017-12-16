#![feature(conservative_impl_trait)]
#![warn(dead_code)]
#[macro_use] extern crate log;
extern crate env_logger;
extern crate dotenv;
extern crate reqwest;
#[macro_use] extern crate hyper;
extern crate hyper_tls;
extern crate hyper_native_tls;
extern crate url;
extern crate uname;
extern crate tokio_service;
extern crate tokio_core;
extern crate futures;

#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_xml_rs;


pub mod http;
pub mod client;
pub mod types;
pub mod errors;

pub mod prelude {
}