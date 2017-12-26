use hyper::client::{Client, HttpConnector, FutureResponse, Request};
use hyper::{Body, Uri, Method, Headers};
use hyper_tls::HttpsConnector;
use futures::{Future, Stream, future};
use serde::Deserialize;
use serde_xml_rs::deserialize;
use std::str::FromStr;
use types::{PlexToken, PlexTokenProvider};
use types::device::{DeviceContainer, PlexDevice, PlexDeviceType, Connection};
use types::server::{PlexServer, Server};
use http::headers::*;
use errors::APIError;
use http::basic_plex_headers;
use http::routes::DEVICES;
use std::rc::Rc;
use regex::Regex;
use std::net::SocketAddr;


#[macro_export]
macro_rules! plex_client_wrapper {

    ($name :ident) => {
        pub struct $name {}
    };
    ($name :ident { $($field:ident -> $field_type:ty),*},
    $inner:ident { $($getter:tt -> $getter_type:ty),* }) => {
    #[derive(Debug, Clone)]
    pub struct $name<'a> {
        client : PlexClient<'a>,
        $($field : $field_type ),*
    }

    impl <'a> $name<'a> {
        $(pub fn $getter(&self) -> &$getter_type {&self.$inner.$getter} )*
    }

    };
}


#[derive(Debug, Clone)]
pub struct PlexClient<'a> {
    pub client: &'a Client<HttpsConnector<HttpConnector>, Body>,
    pub headers: Headers,
    token: PlexToken,
}

/// plex does not escape chars:
/// "   &quot;
/// '   &apos;
/// <   &lt;
/// >   &gt;
/// &   &amp;
///
///
///
impl<'a> PlexClient<'a> {
    pub fn new(client: &'a Client<HttpsConnector<HttpConnector>, Body>, token: PlexToken) -> Self {
        let mut headers = basic_plex_headers();
        headers.set(XPlexToken(token.clone()));
        PlexClient { client, headers, token }
    }

    pub fn get_xml<'de, T: Deserialize<'de>>(&self, dest: &str) -> impl Future<Item=T, Error=APIError> {
        let url = Uri::from_str(dest).unwrap();
        let mut request = Request::new(Method::Get, url);
        request.headers_mut().extend(self.headers.iter());
        self.submit_request(request)
    }

    pub fn get_xml_container<'de, T: Deserialize<'de>>(&self, dest: &str, start: usize, max: usize) -> impl Future<Item=T, Error=APIError> {
        let url = Uri::from_str(dest).unwrap();
        let mut request = Request::new(Method::Get, url);
        request.headers_mut().extend(self.headers.iter());
        request.headers_mut().set(XPlexContainerStart(start.to_string()));
        request.headers_mut().set(XPlexContainerSize(max.to_string()));
        Self::from_xml_response(self.client.request(request))
    }


    fn submit_request<'de, T: Deserialize<'de>>(&self, request: Request) -> impl Future<Item=T, Error=APIError> {
        Self::from_xml_response(self.client.request(request))
    }


    pub fn from_xml_response<'de, T: Deserialize<'de>>(fut_response: FutureResponse) -> impl Future<Item=T, Error=APIError> {
        fut_response.map_err(|_| ()).and_then(|res| {
            let body = res.body().map_err(|_| ()).fold(vec![], |mut acc, chunk| {
                acc.extend_from_slice(&chunk);
                Ok(acc)
            }).and_then(|v| String::from_utf8(v).map_err(|_| ()))
                .and_then(|s| {
                    // escaped the & char which may break deserialization
                    let escaped = s.replace("&", "&amp;");
                    deserialize::<_, T>(escaped.as_bytes()).map_err(|_| ())
                }
                );
            body
        }).map_err(|_| APIError::ReadError)
    }


    pub fn escape_xml(s: &mut String) {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"&((quot|apos|lt|gt|amp);)?").unwrap();
        }
        for m in RE.find_iter(s.clone().as_str()) {
            println!("{:?}", m);
            if (m.end() - m.start()) == 1 {
                s.splice(m.start()..m.end(), "&amp;");
            }
        }
    }


    /// for dev purposes to get the response as string
    pub fn text_response(&self, dest: &str) -> impl Future<Item=String, Error=APIError> {
        let url = Uri::from_str(dest).unwrap();
        let mut request = Request::new(Method::Get, url);
        request.headers_mut().extend(self.headers.iter());

        self.client.request(request).map_err(|_| ()).and_then(|res| {
            let body = res.body().map_err(|_| ()).fold(vec![], |mut acc, chunk| {
                acc.extend_from_slice(&chunk);
                Ok(acc)
            }).and_then(|v| String::from_utf8(v).map_err(|_| ()));

            body
        }).map_err(|_| APIError::ReadError)
    }

    #[inline]
    pub fn headers_mut(&mut self) -> &mut Headers { &mut self.headers }
}


pub trait PlexClientProvider<'a> {
    fn client(&self) -> &Rc<PlexClient<'a>>;
}


impl<'a> PlexTokenProvider for PlexClient<'a> {
    fn token(&self) -> PlexToken { self.token.clone() }
}

#[derive(Debug, Clone)]
pub struct Plex<'a> {
    client: Rc<PlexClient<'a>>
}

impl<'a> Plex<'a> {
    pub fn new(c: &'a Client<HttpsConnector<HttpConnector>, Body>, token: PlexToken) -> Self {
        let client = Rc::new(PlexClient::new(c, token));
        Plex { client }
    }

    pub fn devices(&self) -> impl Future<Item=Vec<PlexDevice<'a>>, Error=APIError> {
        let client = Rc::clone(&self.client);
        client.get_xml::<DeviceContainer>(DEVICES).map(move |d|
            d.devices.into_iter().map(|m| PlexDevice::new(m, Rc::clone(&client))).collect::<Vec<_>>()
        )
    }

    pub fn select_device(&self, name: &'a str) -> impl Future<Item=PlexDevice<'a>, Error=APIError> {
        self.devices().and_then(move |dev| {
            match dev.into_iter().find(|p| p.inner.name.eq(name)) {
                Some(d) => future::ok(d),
                _ => future::err(APIError::ReadError)
            }
        })
    }

    pub fn select_device_type(&self, device_type: PlexDeviceType) -> impl Future<Item=Vec<PlexDevice<'a>>, Error=APIError> {
        self.devices().map(move |dev| {
            let type_name = device_type.as_str();
            dev.into_iter()
                .filter(|p| p.inner.product.eq(type_name))
                .collect::<Vec<_>>()
        })
    }

    pub fn connect(&self, server_url: &str) -> impl Future<Item=PlexServer<'a>, Error=APIError> {
        match server_url.parse::<SocketAddr>() {
            Ok(socket) => {
                let client = Rc::clone(&self.client);
                let conn = Connection::from_endoint(socket);
                Box::new(client.get_xml::<Server>(conn.endpoint().as_str())
                    .map(move |server| PlexServer::new(server.clone(), Rc::clone(&client), conn)))
            }
            _ => {
                Box::new(future::err(APIError::ParseError(format!("Could not parse the server url: {}", server_url))))
                    as Box<Future<Item=PlexServer, Error=APIError>>
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn escape_test() {
        let mut before = r##"<V t="&" v="&amp;"/>"##.to_string();

        PlexClient::escape_xml(&mut before);
        let after = r##"<V t="&amp;" v="&amp;"/>"##.to_string();
        assert_eq!(before, after);
    }
}
