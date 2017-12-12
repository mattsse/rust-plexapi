use hyper::Headers;
use http::headers::XPlexToken;

pub trait PlexTokenProvider {
    fn token(&self) -> &PlexToken;
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