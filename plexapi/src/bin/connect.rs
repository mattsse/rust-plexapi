#![feature(conservative_impl_trait)]
#![allow(warnings)]
extern crate dotenv;
extern crate env_logger;
extern crate futures;
extern crate hyper;
extern crate hyper_native_tls;
extern crate hyper_tls;
#[macro_use]
extern crate log;
#[macro_use]
extern crate plexapi;
extern crate reqwest;
extern crate serde_json;
extern crate serde_xml_rs;
extern crate tokio_core;
extern crate tokio_service;
extern crate uname;

use dotenv::dotenv;
use std::env;
use futures::Future;
use hyper::Client;
use plexapi::types::account::*;
use plexapi::errors::APIError;
use tokio_core::reactor::Core;
use plexapi::client::*;
use plexapi::types::media::audio::*;
use plexapi::types::device::*;
use plexapi::types::library::*;
use hyper_tls::HttpsConnector;
use plexapi::types::sections::*;
use plexapi::types::media::video::*;
use serde_xml_rs::deserialize;
use std::io::prelude::*;
use std::fs::File;

//plex_client_wrapper!(PlexAlbum {inner -> Album}, inner {
//    title -> String
//
//});

#[allow(dead_code)]
fn main() {
    dotenv().ok();
    env_logger::init().unwrap();

    let auth = Login {
        username: env::var("plex_user").unwrap(),
        password: env::var("plex_pass").unwrap(),
    };

    connect_local(auth.clone());
}

fn connect_local(auth: Login) {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let c = Client::configure()
        .connector(HttpsConnector::new(4, &handle).unwrap())
        .build(&handle);
    let token = env::var("server_token").unwrap();
    let plex = Plex::new(&c, token.clone());

    let work = plex.connect("127.0.0.1:32400").and_then(|server| {
        //                let library = server.library();
        //                library.and_then(|lib| lib.section("Musik"))
        //                    .and_then(|section| {
        //                        let site = section.into::<MusicSection>().unwrap();
        //                        site.albums().and_then(|albums| {
        //                            let album = albums.first().unwrap();
        //                            album.tracks().and_then(|tracks| {
        //                                println!("{}", tracks.len());
        //                                Ok(())
        //                            })
        //                        })

        //                site.search(vec![], Some(2)).and_then(|s| {
        //                    println!("{:?}", s);
        //
        println!("{:?}", server);
        Ok(())
        //                })
        //                Ok(())
        //                println!("{:?}", sites);
        //            })
        //                })
    });

    core.run(work).unwrap();
}

fn hyper_client(_auth: Login) {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let c = Client::configure()
        .connector(HttpsConnector::new(4, &handle).unwrap())
        .build(&handle);

    //    let work1 =
    //        _auth.get_token(&c).and_then(|token| {

    let token = env::var("server_token").unwrap();
    let plex = Plex::new(&c, token.clone());

    //        plex.devices().and_then(|devices| {
    //            for dev in devices.iter() {
    //                if dev.inner.name.contains("MSMBP") {
    //                    println!("\n\n{:?}", dev);
    //                }
    //            }
    //            Ok(())
    //        })
    let work1 = plex.select_device("MSMBP")
        .and_then(|device| device.connect())
        .and_then(|server| {
            let library = server.library();
            library
                .and_then(|lib| lib.section("Musik"))
                .and_then(|section| {
                    let site = section.into::<MusicSection>().unwrap();
                    site.albums().and_then(|albums| {
                        let album = albums.first().unwrap();
                        album.tracks().and_then(|tracks| {
                            println!("{}", tracks.len());
                            Ok(())
                        })
                    })

                    //                site.search(vec![], Some(2)).and_then(|s| {
                    //                    println!("{:?}", s);
                    //
                    //                    Ok(())
                    //                })
                    //                Ok(())
                    //                println!("{:?}", sites);
                })
            //                })
        });

    core.run(work1).unwrap();
}

/// restructure -> Single PlexClient that executes Requests
/// pass this PlexClient as referenc around
/// use hyper instead of reqwest
#[allow(dead_code)]
fn req_test(_auth: Login) {
    //    let base_url = env::var("server_baseurl").unwrap();
    //    let url = format!("{}servers", base_url);
    //    let client = reqwest::Client::new();
    //    let sign_in = SignInRequest::new(&auth);
    // "zwsW4byxiqRfwVn8rzqq".to_owned()
    //    let token = auth.token.as_ref().unwrap().clone();
    //    let req: Request = sign_in.into();

    //    match client.
    ////            execute(req) {
    //        get(base_url.as_str())
    //        .header(XPlexToken("Q5QsMoKFv58vXcBax5q6".to_owned())).send() {
    //        Ok(ref mut resp) => {
    ////            let mut content = String::new();
    ////            resp.read_to_string(&mut content);
    //
    //            // resp.status()
    //            println!("");
    //        }
    //        Err(e) => println!("{:?}", e)
    //    }
}

#[allow(dead_code)]
fn devices(_auth: Login) {
    //    let mut client = Client::new(ClientConfig::new());
    //    let base_url = env::var("server_baseurl").unwrap();
    //
    //    let session_info = SessionInfo::new(env::var("server_token").unwrap(), base_url);
    //
    //    if let Ok(session) = client.new_session(session_info) {
    //        let mut session = session.lock().unwrap();
    //
    //        let mut account = session.sign_in(auth).unwrap();
    //        let devices = account.devices().unwrap();
    //        let device = devices.first(PlexDeviceType::PlexMediaServer).unwrap();
    //
    //        let server = device.connect().unwrap();
    //        let lib = server.library().unwrap();
    //        lib.sections();
    //        println!("done");
    //        let mut s = String::new();
    //        resp.read_to_string(&mut s);

    //        println!("{:?}", server);
    ////        let device = devices.select_type(PlexDeviceType::PlexMediaServer).first().to_owned();
    //        println!("{:?}", account);
    //    }
}
