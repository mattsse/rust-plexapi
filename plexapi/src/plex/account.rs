use super::types::*;

// TODO remove token attr
#[derive(Debug, PartialEq, Clone)]
pub struct Login {
    pub username: String,
    pub password: String,
    pub token: Option<PlexToken>
}

impl Login {
}

pub struct PlexAccount {
    pub login: Login,
    pub token: Option<PlexToken>
}

impl PlexAccount {

    pub fn sign_in(&mut self) {

    }



}

