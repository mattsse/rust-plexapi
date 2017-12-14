use hyper::client::{Client, HttpConnector, FutureResponse, Request};
use hyper::{Body, Uri, Method};
use hyper_tls::HttpsConnector;
use futures::{Future, Stream};
use serde::Deserialize;
use serde_xml_rs::deserialize;
use std::str::FromStr;
use types::PlexToken;
use types::device::{DeviceContainer, PlexDevice, Device};
use errors::APIError;
use http::set_basic_plex_headers;
use http::headers::*;
use http::routes::DEVICES;

#[derive(Debug, Clone)]
pub struct PlexClient<'a> {
    pub client: &'a Client<HttpsConnector<HttpConnector>, Body>,
    pub token: PlexToken,
}


impl <'a> PlexClient<'a> {

    pub fn new(client: &'a Client<HttpsConnector<HttpConnector>, Body>, token: PlexToken) -> Self {
        PlexClient { client, token }
    }

    pub fn get_xml<'de, T: Deserialize<'de> + 'static>(&self, dest: &str) -> impl Future<Item=T, Error=APIError> {
        let url = Uri::from_str(dest).unwrap();
        let mut request = Request::new(Method::Get, url);
        set_basic_plex_headers(request.headers_mut());
        request.headers_mut().set(XPlexToken(self.token.clone()));
        Self::from_xml_response(self.client.request(request))
    }

    pub fn from_xml_response<'de, T: Deserialize<'de> + 'static>(fut_response: FutureResponse) -> impl Future<Item=T, Error=APIError> {
        fut_response.map_err(|_| ()).and_then(|res| {
            let body = res.body().map_err(|_| ()).fold(vec![], |mut acc, chunk| {
                acc.extend_from_slice(&chunk);
                Ok(acc)
            }).and_then(|v| String::from_utf8(v).map_err(|_| ()))
                .and_then(|s|
                    deserialize::<_, T>(s.as_bytes()).map_err(|_| ())
                );
            body
        }).map_err(|_| APIError::ReadError)
    }


    pub fn devices(&'a self) -> impl Future<Item=Vec<PlexDevice<'a>>, Error=APIError> {
        self.get_xml::<DeviceContainer>(DEVICES).map(move |d|
            d.devices.into_iter().map(|m| PlexDevice::new(m, self)).collect::<Vec<_>>()
        )
    }
}


