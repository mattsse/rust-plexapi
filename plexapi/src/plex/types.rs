use serde_xml_rs::deserialize;
use http::headers::XPlexToken;

#[derive(Debug,PartialEq, Clone)]
pub enum PlexDeviceType {
    PlexMediaServer,
    PlexWeb
}

impl PlexDeviceType {

    pub fn as_str(&self) -> &'static str {
        match self {
            &PlexDeviceType::PlexMediaServer => "Plex Media Server",
            &PlexDeviceType::PlexWeb => "Plex Web",
        }
    }
}

pub type PlexToken = String;

impl<'a> Into<XPlexToken> for &'a PlexToken {
    fn into(self) -> XPlexToken {
        XPlexToken(self.clone())
    }
}


#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct User {
    #[serde(skip_deserializing, skip_serializing)]
    pub email: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub  username: String,
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlexDevice {
    pub name: String,
    pub product: String,
    pub product_version: String,
    pub platform: String,
    pub platform_version: String,
    pub device: String,
    pub client_identifier: String,
    pub created_at: String,
    pub last_seen_at: String,
    pub provides: String,
    pub owned: String,
    pub public_address: String,
    pub public_address_matches: String,
    pub access_token: String,
    pub presence: String,
    #[serde(rename = "Connection")]
    pub connection: Connection,

    pub https_required: Option<String>,
    pub synced: Option<String>,
    pub relay: Option<String>
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Connection {
    pub protocol: String,
    pub address: String,
    pub port: String,
    pub uri: String,
    pub local: String
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct MediaContainer {
    pub size: String,
    #[serde(rename = "Device")]
    pub devices: Vec<PlexDevice>
}


impl MediaContainer {
    pub fn device(&self, name: &str) -> Option<&PlexDevice> {
        self.devices.iter().find(|p| p.name.eq(name))
    }

    pub fn devices(&self, device_type: PlexDeviceType) -> Vec<&PlexDevice> {
        let type_name = device_type.as_str();
        self.devices.iter()
            .filter(|p|p.product.eq(type_name))
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_xml_rs::Error;

    #[test]
    fn media_container_deserialize() {
        let xml = r##"<?xml version="1.0" encoding="UTF-8"?>
<MediaContainer size="2">
  <Device name="Name" product="Plex Media Server" productVersion="1.Version" platform="Linux" platformVersion="1.0.0. Date" device="PC" clientIdentifier="126713identifier" createdAt="12356123" lastSeenAt="123678213" provides="server" owned="1" accessToken="123adjhktoken" publicAddress="12.00.22.00" httpsRequired="0" synced="0" relay="1" publicAddressMatches="0" presence="1">
    <Connection protocol="https" address="123.132.123.132" port="12332" uri="https://123-123-132-132.cad8z34huadhkasdoih.plex.direct:42366" local="1"/>
  </Device>
  <Device name="Plex Web (Chrome)" product="Plex Web" productVersion="1.0.0" platform="Chrome" platformVersion="62.0" device="OSX" clientIdentifier="aduzwdsahidentifier" createdAt="12367843" lastSeenAt="36478413" provides="client,player,pubsub-player" owned="1" publicAddress="195.99.132.27" publicAddressMatches="0" presence="0" accessToken="asdhhudatoken">
    <Connection protocol="https" address="132.123.132.132" port="12323" uri="https://123-312-123-123.c4123hadhiu2h13hd.plex.direct:12312" local="1"/>
  </Device>
</MediaContainer>
"##;
        let res: Result<MediaContainer, Error> = deserialize(xml.as_bytes());
        assert!(res.is_ok());
        let container = res.unwrap();
        assert_eq!(container.devices.len(), 2);
    }

    #[test]
    fn user_deserialize() {
        let xml = r##"<user email="first.last@mail.com" id="1234567"
        uuid="897654a6a7897b1b" mailing_list_status="active"
        thumb="https://plex.tv/users/897654a6a7897b1b/avatar?c=4637463"
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