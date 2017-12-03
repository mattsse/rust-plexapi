use hyper::header::{Headers, Authorization, Basic};
use hyper::Method;
use reqwest::{Request};
use url::Url;
use plex::account::Login;

use super::routes::*;
use super::headers::{XPlexToken};
use super::response::*;
use plex::types::{ PlexToken };


pub trait PlexRequest<'a> {
    type Response: PlexResponse;

    fn method() -> Method;
    fn url(&self) -> Url;

    fn header(&self) -> Headers;
}

#[derive(Debug, PartialEq, Clone)]
pub struct SignInRequest<'a> {
    pub login: &'a Login
}

impl<'a> SignInRequest<'a> {
    pub fn new(login: &'a Login) -> Self {
        SignInRequest {
            login
        }
    }
}

impl<'a> PlexRequest<'a> for SignInRequest<'a> {
    type Response = SignInResponse;

    fn method() -> Method {
        Method::Post
    }
    fn url(&self) -> Url {
        Url::parse(SIGNIN).unwrap()
    }
    fn header(&self) -> Headers {
        let mut headers = Headers::new();
        headers.set(
            Authorization(
                Basic {
                    username: self.login.username.clone(),
                    password: Some(self.login.password.clone())
                }
            )
        );
        headers
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct DevicesRequest<'a> {
    pub token: &'a PlexToken
}

impl <'a> DevicesRequest<'a> {
    pub fn new(token: &'a PlexToken) -> Self{
        DevicesRequest{token}
    }
}

impl<'a> PlexRequest<'a> for DevicesRequest<'a> {
    type Response = DeviceResponse;

    fn method() -> Method {
        Method::Get
    }
    fn url(&self) -> Url {
        Url::parse(DEVICES).unwrap()
    }
    fn header(&self) -> Headers {
        let mut headers = Headers::new();
        let xtoken: XPlexToken = self.token.into();
        headers.set(xtoken);
        headers
    }
}

/// implements Into<Request> for the given type
macro_rules! to_reqwest {
    ($req:tt) => {
    impl<'a> Into<Request> for $req<'a> {
        fn into(self) -> Request {
            let mut req = Request::new($req::method(), self.url());
            *req.headers_mut() = self.header();
            req
        }
    }
    };
}

to_reqwest!(SignInRequest);
to_reqwest!(DevicesRequest);
