use hyper::{Body, Client, Method, Request, Uri};
use hyper::client::HttpConnector;
use hyper::header::{Authorization, Basic};
use hyper_tls::HttpsConnector;
use http::set_basic_plex_headers;
use futures::Future;
use errors::APIError;
use client::PlexClient;
use std::str::FromStr;
use http::routes::SIGNIN;
use types::PlexToken;

// TODO remove token attr
#[derive(Debug, PartialEq, Clone)]
pub struct Login {
    pub username: String,
    pub password: String,
}

impl Login {
    pub fn new(username: &str, password: &str) -> Login {
        Login {
            username: username.to_owned(),
            password: password.to_owned(),
        }
    }
    pub fn get_token<'a>(
        &self,
        client: &'a Client<HttpsConnector<HttpConnector>, Body>,
    ) -> impl Future<Item = PlexToken, Error = APIError> {
        let url = Uri::from_str(SIGNIN).unwrap();
        let mut request = Request::new(Method::Post, url);
        set_basic_plex_headers(request.headers_mut());
        let header: Authorization<Basic> = self.into();
        request.headers_mut().set(header);
        let resp = client.request(request);
        PlexClient::from_xml_response::<User>(resp).map(|u| u.auth_token)
    }
}

impl<'a> Into<Authorization<Basic>> for &'a Login {
    fn into(self) -> Authorization<Basic> {
        Authorization(Basic {
            username: self.username.clone(),
            password: Some(self.password.clone()),
        })
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct User {
    #[serde(skip_deserializing, skip_serializing)]
    pub email: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub username: String,
    pub id: String,
    // u32
    pub uuid: String,
    pub mailing_list_status: String,
    pub thumb: String,
    pub title: String,
    #[serde(rename = "cloudSyncDevice")]
    pub cloud_sync_device: String,
    pub locale: String,
    #[serde(rename = "authenticationToken")]
    pub authentication_token: String,
    #[serde(rename = "authToken")]
    pub auth_token: String,
    #[serde(rename = "scrobbleTypes")]
    pub scrobble_types: String,
    pub restricted: String,
    //u32,
    pub home: String,
    //u32,
    pub guest: String,
    //u32,
    #[serde(rename = "queueEmail")]
    pub queue_email: String,
    #[serde(rename = "queueUid")]
    pub queue_uid: String,
    #[serde(rename = "hasPassword")]
    pub has_password: bool,
    #[serde(rename = "homeSize")]
    pub home_size: String,
    //u32,
    #[serde(rename = "rememberMe")]
    pub remember_me: bool,
    pub secure: String,
    //u32,
    #[serde(rename = "certificateVersion")]
    pub certificate_version: String,
    //u32
    pub subscription: Option<Subscription>,
    pub profile_settings: Option<ProfileSettings>,
    pub services: Vec<Services>,
    //    #[serde(skip_deserializing,skip_serializing)]
//    _joined_at : Option<String>,

//    #[serde(rename = "authentication-token",skip_deserializing,skip_serializing)]
//    _authentication__token: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Email(String);

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Service {
    identifier: String,
    endpoint: String,
    token: Option<String>,
    status: Option<String>,
    secret: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Services {
    service: Vec<Service>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Subscription {
    active: String,
    status: String,
    plan: String,
    feature: Vec<Feature>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Feature {
    id: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ProfileSettings {
    default_audio_language: String,
    default_subtitle_language: String,
    auto_select_subtitle: String,
    auto_select_audio: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_xml_rs::{deserialize, Error};

    #[test]
    fn user_deserialize() {
        let xml = r##"<user email="first.last@mail.com" id="1234567"
        uuid="897654a6a7897b1b" mailing_list_status="active"
        thumb="https://plex.tv/users/adsadsasd34423da/avatar?c=234323"
        username="User" title="username" cloudSyncDevice="" locale=""
        authenticationToken="SomeToke" authToken="SomeToke"
        scrobbleTypes="" restricted="0" home="0" guest="0"
        queueEmail="queue+asdhsa823hdsf8@save.plex.tv"
        queueUid="c1c1f7c183dcc6fe" hasPassword="true" homeSize="1" rememberMe="false"
        secure="1" certificateVersion="2">
        <services>
            <service identifier="nominatim" endpoint="https://locationiq.org/v1" token="adklasdasjdasdas=" status="online"/>
            <service identifier="imagga" endpoint="https://api.imagga.com/v1" token="394790jdasdjr89jasd=" secret="2asdaskdjaosdjas09du38uhasd" status="online"/>
        </services>
        <username>Name</username>
    <email>first.last@mail.com</email>
    <joined-at type="datetime">Some Date</joined-at>
    <authentication-token>SomeToke</authentication-token>
        </user>"##;
        let user: Result<User, Error> = deserialize(xml.as_bytes());
        assert!(user.is_ok());
    }
}
