use hyper::client::{Client, HttpConnector, FutureResponse, Request};
use hyper::{Body, Uri, Method, Headers, HttpVersion};
use hyper_tls::HttpsConnector;
use futures::{Future, Stream, future};
use serde::Deserialize;
use serde_xml_rs::deserialize;
use std::str::FromStr;
use types::PlexToken;
use types::device::{DeviceContainer, PlexDevice, Device};
use errors::APIError;
use http::basic_plex_headers;
use http::headers::*;
use http::routes::DEVICES;
use types::account::Login;

#[derive(Debug, Clone)]
pub struct PlexClient {
    pub client: Client<HttpsConnector<HttpConnector>, Body>,
    pub headers: Headers,
}


impl PlexClient {
    pub fn new(client: Client<HttpsConnector<HttpConnector>, Body>) -> Self {
        PlexClient { client, headers: basic_plex_headers() }
    }


    pub fn get_xml<'de, T: Deserialize<'de>>(&self, dest: &str) -> impl Future<Item=T, Error=APIError> {
        let url = Uri::from_str(dest).unwrap();
        let mut request = Request::new(Method::Get, url);
        request.headers_mut().extend(self.headers.iter());
        Self::from_xml_response(self.client.request(request))
    }

    pub fn from_xml_response<'de, T: Deserialize<'de>>(fut_response: FutureResponse) -> impl Future<Item=T, Error=APIError> {
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


    pub fn devices<'a>(&'a self) -> impl Future<Item=Vec<PlexDevice<'a>>, Error=APIError> {
        self.get_xml::<DeviceContainer>(DEVICES).map(move |d|
            d.devices.into_iter().map(|m| PlexDevice::new(m, self)).collect::<Vec<_>>()
        )
    }

    #[inline]
    pub fn headers_mut(&mut self) -> &mut Headers { &mut self.headers }

    pub fn login<'a>(&'a mut self, login: Login) -> Box<Future<Item=PlexToken, Error=APIError> + 'a> {
        Box::new(login.get_token(&self.client).and_then(move |token| {
            let xtoken = XPlexToken(token.clone());
            self.headers_mut().set(xtoken);
            Ok(token)
        }))
    }
}


