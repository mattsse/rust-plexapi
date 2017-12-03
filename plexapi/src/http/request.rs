use hyper::header::{Headers, Authorization, Basic};
use hyper::Method;
use reqwest::{Request, Response};
use url::Url;
use serde_xml_rs::{deserialize, Error};
use plex::account::Login;
use super::routes::*;
use super::headers::XPlexToken;
use plex::types::{PlexToken, PlexDevice, User, MediaContainer};


pub trait PlexRequest {
    type Response;
    type Error;

    fn method() -> Method;
    fn url(&self) -> Url;
    fn header(&self) -> Headers;
    fn from_response(&self, response: Response) -> Result<Self::Response, Self::Error>;

    fn to_request(&self) -> Request {
        let mut req = Request::new(Self::method(), self.url());
        *req.headers_mut() = self.header();
        req
    }
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

impl<'a> PlexRequest for SignInRequest<'a> {
    type Response = User;
    type Error = ();

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
    fn from_response(&self, response: Response) -> Result<Self::Response, Self::Error> {
        match deserialize(response) {
            Ok(data) => Ok(data),
            _ => Err(())
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct DevicesRequest<'a> {
    pub token: &'a PlexToken
}

impl<'a> DevicesRequest<'a> {
    pub fn new(token: &'a PlexToken) -> Self {
        DevicesRequest { token }
    }
}

impl<'a> PlexRequest for DevicesRequest<'a> {
    type Response = Vec<PlexDevice>;
    type Error = ();

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
    fn from_response(&self, response: Response) -> Result<Self::Response, Self::Error> {
        let res: Result<MediaContainer, Error> = deserialize(response);
        match res {
            Ok(data) => Ok(data.devices),
            _ => {
                println!("desirialize error");
                Err(())
            }
        }
    }
}
