use super::types::*;
use hyper::header::{Authorization, Basic};
use super::types::User;
use super::session::Session;
use http::request::{PlexError, DevicesRequest, PlexRequestExecutor};

use std::rc::{Rc, Weak};

// TODO remove token attr
#[derive(Debug, PartialEq, Clone)]
pub struct Login {
    pub username: String,
    pub password: String,
    pub token: Option<PlexToken>
}

impl Login {
    pub fn new(username: &str, password: &str) -> Login {
        Login {
            username: username.to_owned(),
            password: password.to_owned(),
            token: None
        }
    }
}

impl Into<Authorization<Basic>> for Login {
    fn into(self) -> Authorization<Basic> {
        Authorization(
            Basic {
                username: self.username.clone(),
                password: Some(self.password.clone())
            }
        )
    }
}

#[derive(Debug)]
pub struct PlexAccount<'a> {
    pub session: &'a Session,
    pub login: Login,
    user: User
}

impl <'a> PlexAccount<'a> {
    pub fn new(login: Login, session: &'a Session, user: User) -> Self {
        PlexAccount {
            login,
            session,
            user
        }
    }

    pub fn devices(&self) -> Result<Vec<PlexDevice>, PlexError> {
        match self.session.submit(DevicesRequest::new(self.token())) {
            Ok(devices) => {
                let s = devices.into_iter()
//                    .map(|m|m.clone())
                    .map(|m| PlexDevice::new(m, &self))
                    .collect::<Vec<_>>();
                Ok(s)
            }
            Err(e) => Err(e)
        }
    }
}

impl <'a> PlexTokenProvider for PlexAccount<'a> {
    fn token(&self) -> &PlexToken {
        &self.user.authentication_token
    }
}