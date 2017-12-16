use hyper::client::{Client, HttpConnector, FutureResponse, Request};
use hyper::{Body, Uri, Method, Headers};
use hyper_tls::HttpsConnector;
use futures::{Future, Stream, future};
use serde::Deserialize;
use serde_xml_rs::deserialize;
use std::str::FromStr;
use types::PlexToken;
use types::device::{DeviceContainer, PlexDevice, PlexDeviceType};
use http::headers::*;
use errors::APIError;
use http::basic_plex_headers;
use http::routes::DEVICES;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct PlexClient<'a> {
    pub client: &'a Client<HttpsConnector<HttpConnector>, Body>,
    pub headers: Headers,
    token: PlexToken
}


impl<'a> PlexClient<'a> {
    pub fn new(client: &'a Client<HttpsConnector<HttpConnector>, Body>, token: PlexToken) -> Self {
        let mut headers = basic_plex_headers();
        headers.set(XPlexToken(token.clone()));
        PlexClient { client, headers, token }
    }

    pub fn token(&self) -> PlexToken { self.token.clone() }

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

    #[inline]
    pub fn headers_mut(&mut self) -> &mut Headers { &mut self.headers }
}

#[derive(Debug, Clone)]
pub struct Plex<'a> {
    client: Rc<PlexClient<'a>>
}

impl<'a> Plex<'a> {
    pub fn new(c: &'a Client<HttpsConnector<HttpConnector>, Body>, token: PlexToken) -> Self {
        let client = Rc::new(PlexClient::new(c, token));
        Plex { client }
    }

    pub fn devices(&self) -> impl Future<Item=Vec<PlexDevice<'a>>, Error=APIError> {
        let client = Rc::clone(&self.client);
        client.get_xml::<DeviceContainer>(DEVICES).map(move |d|
            d.devices.into_iter().map(|m| PlexDevice::new(m, Rc::clone(&client))).collect::<Vec<_>>()
        )
    }

    pub fn select_device(&self, name: &'a str) -> impl Future<Item=PlexDevice<'a>, Error=APIError> {
        self.devices().and_then(move |dev| {
            match dev.into_iter().find(|p| p.inner.name.eq(name)) {
                Some(d) => future::ok(d),
                _ => future::err(APIError::ReadError)
            }
        })
    }

    pub fn select_device_type(&self, device_type: PlexDeviceType) -> impl Future<Item=Vec<PlexDevice<'a>>, Error=APIError> {
        self.devices().map(move |dev| {
            let type_name = device_type.as_str();
            dev.into_iter()
                .filter(|p| p.inner.product.eq(type_name))
                .collect::<Vec<_>>()
        })
    }
}