use hyper::header::{Headers, Authorization, Basic};
use hyper::Method;
use reqwest::{Request, Response};
use url::Url;
use serde_xml_rs::{deserialize, Error};
use plex::account::Login;
use super::routes::*;
use super::headers::XPlexToken;
use plex::types::*;
use plex::library::sections::*;


#[derive(Debug, PartialEq, Clone)]
pub enum PlexError {
    ResponseDeserializeError,
    RequestFailed,
    UnknownError
}

impl Default for PlexError {
    fn default() -> Self {
        PlexError::UnknownError
    }
}

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

pub trait PlexRequestExecutor {

    fn submit<T>(&self, req: T) -> Result<T::Response, T::Error>
        where T: PlexRequest, T::Error : Default;

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
    type Error = PlexError;

    fn method() -> Method { Method::Post }
    fn url(&self) -> Url { Url::parse(SIGNIN).unwrap() }
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
            _ => Err(PlexError::ResponseDeserializeError)
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
    type Response = Vec<Device>;
    type Error = PlexError;

    fn method() -> Method { Method::Get }
    fn url(&self) -> Url { Url::parse(DEVICES).unwrap() }
    fn header(&self) -> Headers { self.token.headers() }

    fn from_response(&self, response: Response) -> Result<Self::Response, Self::Error> {
        let res: Result<MediaContainer, Error> = deserialize(response);
        match res {
            Ok(data) => Ok(data.devices),
            _ => {
                error!("desirialize error");
                Err(PlexError::ResponseDeserializeError)
            }
        }
    }
}

#[derive(Debug)]
pub struct ConnectPlexDeviceRequest<'a> {
    device: &'a PlexDevice<'a>,
    connection: &'a Connection
}

impl<'a> ConnectPlexDeviceRequest<'a> {
    pub fn new(device: &'a PlexDevice, connection: &'a Connection) -> Self {
        ConnectPlexDeviceRequest { device, connection }
    }
}

impl<'a> PlexRequest for ConnectPlexDeviceRequest<'a> {
    type Response = PlexServer<'a>;
    type Error = PlexError;

    fn method() -> Method { Method::Get }

    fn url(&self) -> Url { Url::parse(self.connection.endpoint().as_str()).unwrap() }

    fn header(&self) -> Headers { self.device.account.token().headers() }

    fn from_response(&self, response: Response) -> Result<Self::Response, Self::Error> {
        let res: Result<Server, Error> = deserialize(response);
        match res {
            Ok(data) => Ok(PlexServer::new(data, self.device, self.connection)),
            _ => {
                error!("desirialize error");
                Err(PlexError::ResponseDeserializeError)
            }
        }
    }
}

#[derive(Debug)]
pub struct PlexLibraryRequest<'a> {
    server: &'a PlexServer<'a>
}

impl<'a> PlexLibraryRequest<'a> {
    pub fn new(server: &'a PlexServer<'a>) -> Self { PlexLibraryRequest { server } }
}

impl<'a> PlexRequest for PlexLibraryRequest<'a> {
    type Response = PlexLibrary<'a>;
    type Error = PlexError;

    fn method() -> Method { Method::Get }

    fn url(&self) -> Url {
        Url::parse(self.server.format_url(PlexLibrary::PATH).as_str()).unwrap()
    }

    fn header(&self) -> Headers { self.server.token().headers() }

    fn from_response(&self, response: Response) -> Result<Self::Response, Self::Error> {
        let res: Result<Library, Error> = deserialize(response);
        match res {
            Ok(data) => Ok(PlexLibrary::new(data, self.server)),
            _ => {
                error!("desirialize error");
                Err(PlexError::ResponseDeserializeError)
            }
        }
    }
}

pub struct PlexLibrarySectionsRequest<'a> {
    server: &'a PlexServer<'a>
}

impl<'a> PlexLibrarySectionsRequest<'a> {
    pub fn new(server: &'a PlexServer<'a>) -> Self { PlexLibrarySectionsRequest { server } }
}

impl<'a> PlexRequest for PlexLibrarySectionsRequest<'a> {
    type Response = Vec<Section>;
    type Error = PlexError;

    fn method() -> Method { Method::Get }

    fn url(&self) -> Url { Url::parse(self.server.format_url(PlexLibrary::SECTIONS).as_str()).unwrap() }

    fn header(&self) -> Headers { self.server.token().headers() }

    fn from_response(&self, response: Response) -> Result<Self::Response, Self::Error> {
        let res: Result<Sections, Error> = deserialize(response);
        match res {
            Ok(data) => Ok(data.sections),
            _ => {
                error!("desirialize error");
                Err(PlexError::ResponseDeserializeError)
            }
        }
    }
}
