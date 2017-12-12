use hyper::client::{Client, HttpConnector, FutureResponse, Request};
use hyper::{Body, Headers, Uri, Method};
use hyper_native_tls::NativeTlsClient;
use hyper_tls::HttpsConnector;
use types::PlexToken;
use serde_xml_rs::{deserialize, Error};
use tokio_core::reactor::Handle;
use errors::APIError;
use http::set_basic_plex_headers;
use auth::PlexTokenizer;
use http::headers::*;
use serde::Deserialize;
use std::str::FromStr;
use futures::{Future, Stream};
use futures::future::FutureResult;

#[derive(Debug, Clone)]
pub struct PlexClient {
    pub client: Client<HttpsConnector<HttpConnector>, Body>,
    pub token: PlexToken
}


impl PlexClient {
    pub fn create<T: Future<Item=PlexToken, Error=APIError>>(client: Client<HttpsConnector<HttpConnector>,
        Body>, to_token: T) -> impl Future<Item=PlexClient, Error=APIError> {
        to_token.map(|token| Self::new(client, token))
    }

    pub fn new(client: Client<HttpsConnector<HttpConnector>, Body>, token: PlexToken) -> Self {
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
                    deserialize::<_, T>(s.as_bytes()).map_err(|e| ())
                );
            body
        }).map_err(|_| APIError::ReadError)
    }
}


