use hyper::Headers;
use http::headers::XPlexToken;
use http::request::*;
use super::account::PlexAccount;

use http::request::PlexError;
pub use super::server::*;
pub use super::library::*;
pub use types::*;

#[derive(Debug, PartialEq, Clone)]
pub enum PlexDeviceType {
    PlexMediaServer,
    PlexMediaPlayer,
    PlexForiOS,
    PlexForAndroid,
    PlexWeb
}

impl PlexDeviceType {
    pub fn as_str(&self) -> &'static str {
        match self {
            &PlexDeviceType::PlexMediaServer => "Plex Media Server",
            &PlexDeviceType::PlexWeb => "Plex Web",
            &PlexDeviceType::PlexMediaPlayer => "Plex Media Player",
            &PlexDeviceType::PlexForiOS => "Plex for iOS",
            &PlexDeviceType::PlexForAndroid => "Plex for Android"
        }
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
    secret: Option<String>
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Services {
    service: Vec<Service>
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Subscription {
    active: String,
    status: String,
    plan: String,
    feature: Vec<Feature>
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Feature {
    id: String
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ProfileSettings {
    default_audio_language: String,
    default_subtitle_language: String,
    auto_select_subtitle: String,
    auto_select_audio: String
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    pub name: String,
    pub product: String,
    pub product_version: String,
    pub platform: String,
    //
    pub platform_version: String,
    pub device: String,
    pub client_identifier: String,
    pub created_at: String,
    pub last_seen_at: String,
    pub provides: String,
    pub owned: Option<String>,
    pub public_address: String,
    pub public_address_matches: Option<String>,
    pub access_token: Option<String>,
    pub presence: Option<String>,

    #[serde(rename = "Connection")]
    pub connection: Option<Vec<Connection>>,

    pub version: Option<String>,
    pub id: Option<String>,
    pub token: Option<String>,
    pub model: Option<String>,
    pub vendor: Option<String>,
    pub https_required: Option<String>,
    pub synced: Option<String>,
    pub relay: Option<String>,
    pub screen_resolution: Option<String>,
    pub screen_density: Option<String>
}

impl Device {}

#[derive(Debug)]
pub struct PlexDevice<'a> {
    inner: Device,
    pub account: &'a PlexAccount<'a>
}

impl<'a> PlexDevice<'a> {
    pub fn new(inner: Device, account: &'a PlexAccount<'a>) -> Self {
        PlexDevice {
            inner,
            account
        }
    }

    /// prioritize local connections over remote
    pub fn connect(&self) -> Result<PlexServer, PlexError> {
        let con = self.inner.connection.as_ref().unwrap().first().unwrap();
        let req = ConnectPlexDeviceRequest::new(self, con);
        self.account.session.submit(req)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ConnectionProtocol {
    Https,
    Http,
}

impl ConnectionProtocol {
    pub fn as_str(&self) -> &'static str {
        match self {
            &ConnectionProtocol::Https => "https",
            &ConnectionProtocol::Http => "http"
        }
    }
    pub fn from_str(s: &str) -> Option<ConnectionProtocol> {
        match s {
            "https" => Some(ConnectionProtocol::Https),
            "http" => Some(ConnectionProtocol::Http),
            _ => None
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Connection {
    protocol: Option<String>,
    pub address: Option<String>,
    pub port: Option<String>,
    pub uri: String,
    local: Option<String>
}

impl Connection {
    pub fn is_local(&self) -> bool {
        match self.local {
            Some(ref s) => "1".eq(s),
            _ => false
        }
    }

    pub fn protocol(&self) -> Option<ConnectionProtocol> {
        match self.protocol {
            Some(ref s) => ConnectionProtocol::from_str(s),
            _ => None
        }
    }

    pub fn endpoint(&self) -> String {
        match self.address {
            Some(ref s) => format!("{}:{}", s, self.port.as_ref().unwrap()),
            _ => self.uri.clone()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeviceContainer {
    pub size: Option<String>,
    pub public_address: Option<String>,
    #[serde(rename = "Device", default)]
    pub devices: Vec<Device>
}

pub trait PlexDeviceFilter {
    fn select(&self, name: &str) -> Option<&PlexDevice>;
    fn select_type(&self, device_type: PlexDeviceType) -> Vec<&PlexDevice>;

    fn first(&self, device_type: PlexDeviceType) -> Option<&PlexDevice> {
        match self.select_type(device_type).first() {
            Some(d) => {
                Some(d)
            }
            _ => None
        }
    }
}

impl<'a> PlexDeviceFilter for Vec<PlexDevice<'a>> {
    fn select(&self, name: &str) -> Option<&PlexDevice> {
        self.into_iter().find(|p| p.inner.name.eq(name))
    }

    fn select_type(&self, device_type: PlexDeviceType) -> Vec<&PlexDevice> {
        let type_name = device_type.as_str();
        self.into_iter()
            .filter(|p| p.inner.product.eq(type_name))
            .collect::<Vec<_>>()
    }
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