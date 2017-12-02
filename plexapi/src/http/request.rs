use hyper::header::{Headers, Authorization, Basic};
use hyper::Method;
use reqwest::{Request,Response};
use tokio_service::{Service, NewService};
use url::Url;
use plex::account::Login;

pub use super::routes::*;
pub use super::{basic_plex_headers};



pub trait PlexRequest<'a> {

    type Response : From<Response>;

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
    type Response = Response;

    fn method() -> Method {
        Method::Post
    }
    fn url(&self) -> Url {
        // save to unwrap
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
