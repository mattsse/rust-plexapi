use types::PlexToken;
use errors::APIError;
use hyper::{Body, Client, Request, Method, Uri};
use hyper::client::HttpConnector;
use hyper::header::{Authorization, Basic};
use hyper_tls::HttpsConnector;
use types::account::Login;
use http::set_basic_plex_headers;
use http::routes::SIGNIN;
use plex::types::User;
use client::PlexClient;
use std::str::FromStr;
use futures::{Future};

pub trait PlexTokenizer {
    fn token(&self, client: &Client<HttpsConnector<HttpConnector>, Body>) -> Result<PlexToken, APIError>;
}

#[allow(unused_variables)]
impl PlexTokenizer for PlexToken {
    fn token(&self, client: &Client<HttpsConnector<HttpConnector>, Body>) -> Result<PlexToken, APIError> {
        Ok(self.to_owned())
    }
}

impl PlexTokenizer for Login {
    fn token(&self, client: &Client<HttpsConnector<HttpConnector>, Body>) -> Result<PlexToken, APIError> {
        let url = Uri::from_str(SIGNIN).unwrap();
        let mut request = Request::new(Method::Post, url);
        set_basic_plex_headers(request.headers_mut());
        request.headers_mut().set(
            Authorization(
                Basic {
                    username: self.username.clone(),
                    password: Some(self.password.clone())
                }
            )
        );
        let response = client.request(request);
        Err(APIError::ReadError)
//        let de: Result<User, APIError> = PlexClient::from_xml_response(response);
//        de.map(|user| user.authentication_token)
    }
}
