use std::result::Result;
use std::sync::{Arc, Mutex};
use super::types::*;
use http::*;
use http::response::*;
use http::request::*;
use plex::account::*;
use reqwest::{Client, Request};

// make generic T : Into<PlexToken>
#[derive(Debug, PartialEq, Clone)]
pub struct SessionInfo {
    pub auth_token: PlexToken,
    // server endpoint url
    pub url: String
}

impl SessionInfo {
    pub fn new(auth_token: PlexToken, url: String) -> Self {
        SessionInfo {
            auth_token,
            url
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SessionState {}

impl SessionState {
    pub fn new() -> Self {
        SessionState {}
    }
}

#[derive(Debug)]
pub struct Session {
    pub session_info: SessionInfo,
    session_state: Arc<Mutex<SessionState>>,
    session_client: Arc<Client>
}

impl Session {
    pub fn new(session_info: SessionInfo) -> Self {
        let session_state = Arc::new(Mutex::new(SessionState::new()));
        let session_client = Arc::new(Client::builder()
            .default_headers(basic_plex_headers())
            .build().unwrap());

        Session {
            session_info,
            session_state,
            session_client
        }
    }
//    fn call<'a, T>(&self, req: T) -> Box<Future<Item=T::Response, Error=()>>
//        where T: PlexRequest<'a> {
//        // TODO for future async impl
//        unimplemented!()
//    }

    // TODO make this so generic that sign in with token or account is possible
    pub fn sign_in(&self, login: Login) -> Result<PlexAccount, PlexError> {
        let req = SignInRequest::new(&login);
        let resp: User = self.submit(req).unwrap();
        let account = PlexAccount::new(login.clone(), &self, resp);
        Ok(account)
    }
}


impl <'a> PlexRequestExecutor for &'a Session {

    fn submit<T>(&self, req: T) -> Result<T::Response, T::Error>
        where T: PlexRequest, T::Error: Default {
        let request = req.to_request();
        let client = self.session_client.clone();
        match client.execute(request) {
            Ok(res) =>
                req.from_response(res),
            _ =>
                Err(T::Error::default())
        }
    }
}

