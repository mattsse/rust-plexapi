#![feature(conservative_impl_trait)]
#![feature(splice)]
#![warn(dead_code)]
extern crate dotenv;
extern crate env_logger;
extern crate futures;
#[macro_use]
extern crate hyper;
extern crate hyper_native_tls;
extern crate hyper_tls;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate regex;
extern crate reqwest;
extern crate tokio_core;
extern crate tokio_service;
extern crate uname;
extern crate url;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;

pub mod http;
#[macro_use]
pub mod client;
pub mod types;
pub mod errors;

pub mod prelude {}
