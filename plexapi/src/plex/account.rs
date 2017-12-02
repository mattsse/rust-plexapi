use super::types::*;
use hyper::header::{Authorization, Basic};
use std::sync::Arc;
use super::types::User;
use super::session::Session;

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

impl <'a>PlexAccount<'a> {
    pub fn new(login: Login, session: &'a Session, user: User) -> Self {
        PlexAccount {
            login,
            session,
            user
        }
    }

    pub fn token(&self) -> PlexToken {
        self.user.authentication_token.clone().into()
    }

    pub fn sign_in(&mut self) {}
}

