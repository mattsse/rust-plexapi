use hyper::Headers;
use http::headers::XPlexToken;

pub mod settings {
    pub const PROJECT: &'static str = env!("CARGO_PKG_NAME");
    pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    pub const X_PLEX_CONTAINER_SIZE: usize = 100;
}

pub trait PlexTokenProvider {
    fn token(&self) -> PlexToken;
}

pub type PlexToken = String;

pub trait PlexHeaders {
    fn headers(&self) -> Headers;
}

impl<'a> PlexHeaders for PlexToken {
    fn headers(&self) -> Headers {
        let mut headers = Headers::new();
        let xtoken: XPlexToken = self.into();
        headers.set(xtoken);
        headers
    }
}


impl<'a> Into<XPlexToken> for &'a PlexToken {
    fn into(self) -> XPlexToken {
        XPlexToken(self.clone())
    }
}


pub mod account;
pub mod device;
pub mod server;
pub mod library;
pub mod media;
pub mod playlist;