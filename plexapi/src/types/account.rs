use plex::types::*;
use hyper::{Body, Client, Request, Method, Uri};
use hyper::client::HttpConnector;
use hyper::header::{Authorization, Basic};
use hyper_tls::HttpsConnector;
use http::set_basic_plex_headers;
use plex::types::User;
use plex::session::Session;
use http::request::{PlexError, DevicesRequest, PlexRequestExecutor};
use futures::Future;
use errors::APIError;
use client::PlexClient;
use std::str::FromStr;
use http::routes::SIGNIN;

// TODO remove token attr
#[derive(Debug, PartialEq, Clone)]
pub struct Login {
    pub username: String,
    pub password: String
}

impl Login {
    pub fn new(username: &str, password: &str) -> Login {
        Login {
            username: username.to_owned(),
            password: password.to_owned(),
        }
    }
    pub fn get_token<'a>(&self, client: &'a Client<HttpsConnector<HttpConnector>, Body>) -> impl Future<Item=PlexToken, Error=APIError> {
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
        let resp = client.request(request);
        PlexClient::from_xml_response::<User>(resp).map(|u| u.auth_token)
    }
}

impl Into<Authorization<Basic>> for Login {
    fn into(self) -> Authorization<Basic> {
        Authorization(
            Basic {
                username: self.username.clone(),
                password: Some(self.password.clone())
            }
        )
    }
}

#[derive(Debug)]
pub struct PlexAccount<'a> {
    pub session: &'a Session,
    pub login: Login,
    user: User
}

impl<'a> PlexAccount<'a> {
    pub fn new(login: Login, session: &'a Session, user: User) -> Self {
        PlexAccount {
            login,
            session,
            user
        }
    }

    pub fn devices(&self) -> Result<Vec<PlexDevice>, PlexError> {
        match self.session.submit(DevicesRequest::new(self.token())) {
            Ok(devices) => {
                let s = devices.into_iter()
//                    .map(|m|m.clone())
                    .map(|m| PlexDevice::new(m, &self))
                    .collect::<Vec<_>>();
                Ok(s)
            }
            Err(e) => Err(e)
        }
    }
}

impl<'a> PlexTokenProvider for PlexAccount<'a> {
    fn token(&self) -> &PlexToken {
        &self.user.authentication_token
    }
}