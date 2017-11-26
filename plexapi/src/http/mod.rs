use hyper::header::{Headers, Header};
use hyper::Method;
use reqwest::Request;

use std::borrow::Cow;
use url::{Url, Host};
use plex::account::Login;

/// @see https://github.com/Arcanemagus/plex-api/wiki/Plex-Web-API-Overview#request-headers

/// Platform name, eg iOS, MacOSX, Android, LG, etc
header! { (XPlexPlatform, "X-Plex-Platform") => [String] }

/// Operating system version, eg 4.3.1, 10.6.7, 3.2
header! { (XPlexPlatformVersion, "X-Plex-Platform-Version") => [String] }

/// one or more of [player, controller, server]
header! { (XPlexProviders, "X-Plex-Provides") => [String] }

///UUID, serial number, or other number unique per device
header! { (XPlexClientIdentifier, "X-Plex-Client-Identifier") => [String] }

/// Plex application name, eg Laika, Plex Media Server, Media Link
header! { (XPlexProduct, "X-Plex-Product") => [String] }

/// Plex application version number
header! { (XPlexVersion, "X-Plex-Version") => [String] }

/// Device name and model number, eg iPhone3,2, Motorola XOOM™, LG5200TV
header! { (XPlexDevice, "X-Plex-Device") => [String] }

/// Paging Size, eg Plex-Container-Size=1
header! { (XPlexContainerSize, "X-Plex-Container-Size") => [String] }

/// Paging Start, eg X-Plex-Container-Start=0
header! { (XPlexContainerStart, "X-Plex-Container-Start") => [String] }

/// Authentication token
header! { (XPlexToken, "X-Plex-Token") => [String] }


use self::routes::*;

pub trait Service {
    type Request;
    type Response;
    type Error;

    fn call(&self, req: Self::Request) -> Result<Self::Response, Self::Error>;
}

pub trait PlexRequest<'a> {
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

/// fn construct_headers() -> Headers {
   ///     let mut headers = Headers::new();
   ///     headers.set(UserAgent::new("reqwest"));
   ///     headers.set(ContentType::png());
   ///     headers
   /// }

// TODO create into_plexrequest macro

impl<'a> PlexRequest<'a> for SignInRequest<'a> {
    fn method() -> Method {
        Method::Post
    }
    fn url(&self) -> Url {
        // save to unwrap
        Url::parse(SIGNIN).unwrap()
    }
    fn header(&self) -> Headers {
        let mut headers = Headers::new();
        headers.set(XPlexToken("".to_owned()));
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



/// Some basic plex routes
pub mod routes {
    pub const ACCOUNT: &'static str = "https://plex.tv/users/account";
    pub const FRIENDINVITE: &'static str = "https://plex.tv/api/servers/{machineId}/shared_servers"; // post with data
    pub const FRIENDSERVERS: &'static str = "https://plex.tv/api/servers/{machineId}/shared_servers/{serverId}";// put with data
    pub const PLEXSERVERS: &'static str = "https://plex.tv/api/servers/{machineId}";// get
    pub const FRIENDUPDATE: &'static str = "https://plex.tv/api/friends/{userId}";// put with args, delete
    pub const REMOVEINVITE: &'static str = "https://plex.tv/api/invites/requested/{userId}?friend=0&server=1&home=0";// delete
    pub const REQUESTED: &'static str = "https://plex.tv/api/invites/requested";// get
    pub const REQUESTS: &'static str = "https://plex.tv/api/invites/requests";// get
    pub const SIGNIN: &'static str = "https://my.plexapp.com/users/sign_in.xml";// get with auth
    pub const WEBHOOKS: &'static str = "https://plex.tv/api/v2/user/webhooks";
}