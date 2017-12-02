use hyper::header::Headers;
use uname::uname;
use plex::settings::*;
use futures::Future;
use self::request::PlexRequest;
use self::response::PlexResponse;

/// @see https://github.com/Arcanemagus/plex-api/wiki/Plex-Web-API-Overview#request-headers

/// Platform name, eg iOS, MacOSX, Android, LG, etc
header! { (XPlexPlatform, "X-Plex-Platform") => [String] }

/// Operating system version, eg 4.3.1, 10.6.7, 3.2
header! { (XPlexPlatformVersion, "X-Plex-Platform-Version") => [String] }

/// one or more of [player, controller, server]
header! { (XPlexProvides, "X-Plex-Provides") => [String] }

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

pub trait DummyService<'a> {
    type Request;
    type Response;
    type Error;
    type Future: Future<Item = Self::Response, Error = Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future;
}

pub trait PlexService<'a, T, S> {
    type Error;

    fn submit(&self, req: T) -> Result<S, Self::Error>;
}

/// Basic Headers for requests to plex
pub fn basic_plex_headers() -> Headers {
    let mut headers = Headers::new();
    // TODO is this completely safe?
    let info = uname().unwrap();
    headers.set(XPlexPlatform(info.sysname.clone()));
    headers.set(XPlexPlatformVersion(info.version.clone()));
    headers.set(XPlexProduct(PROJECT.to_owned()));
    headers.set(XPlexVersion(VERSION.to_owned()));
    headers.set(XPlexDevice(info.sysname.clone()));
    headers.set(XPlexClientIdentifier(info.nodename.clone()));

    headers
}


pub mod request;
pub mod response;


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

pub mod prelude {
    pub use super::*;
    pub use super::request::*;
    pub use super::response::*;
}