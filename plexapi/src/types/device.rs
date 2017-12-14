use client::PlexClient;
use futures::{Future, future, Map, MapErr};
use errors::APIError;
use serde_xml_rs::Error;
use types::server::{Server, PlexServer};

#[derive(Debug)]
pub struct PlexDevice<'a> {
    inner: Device,
    client: &'a PlexClient<'a>,
}

impl<'a> PlexDevice<'a> {
    pub fn new(inner: Device, client: &'a PlexClient) -> Self {
        PlexDevice {
            inner,
            client,
        }
    }

    /// Connects to the device's connection, prefers local connections
    pub fn connect(&'a self) -> impl Future<Item=PlexServer<'a>, Error=APIError> {
        let con = match self.inner.connections.len() {
            0 => None,
            1 => self.inner.connections.first(),
            _ => match self.inner.connections.iter().find(|p| p.is_local()) {
                Some(c) => Some(c),
                _ => self.inner.connections.first()
            }
        };
        // boxing necessary to unify return type...
        let res: Box<Future<Item=PlexServer, Error=APIError>> = match con {
            Some(c) => Box::new(self.client.get_xml::<Server>(c.endpoint().as_str()).map(move |server| PlexServer::new(server, self.client, c))),
            _ => Box::new(future::err(APIError::from(Error::Custom("No Connection Present for this Device".to_string()))))
        };
        res
    }
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

    #[serde(rename = "Connection", default)]
    pub connections: Vec<Connection>,

    pub version: Option<String>,
    pub id: Option<String>,
    pub token: Option<String>,
    pub model: Option<String>,
    pub vendor: Option<String>,
    pub https_required: Option<String>,
    pub synced: Option<String>,
    pub relay: Option<String>,
    pub screen_resolution: Option<String>,
    pub screen_density: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeviceContainer {
    pub size: Option<String>,
    pub public_address: Option<String>,
    #[serde(rename = "Device", default)]
    pub devices: Vec<Device>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Connection {
    protocol: Option<String>,
    pub address: Option<String>,
    pub port: Option<String>,
    pub uri: String,
    local: Option<String>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_xml_rs::{deserialize, Error};

    #[test]
    fn device_container_1_test() {
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
        let res: Result<DeviceContainer, Error> = deserialize(xml.as_bytes());
        assert!(res.is_ok());
        let container = res.unwrap();
        assert_eq!(container.devices.len(), 2);
    }

    #[test]
    fn device_container_2_test() {
        let xml = r##"<?xml version="1.0" encoding="UTF-8"?>
<MediaContainer publicAddress="233.12321123">
  <Device name="plexapi" publicAddress="213.12.213.123" product="plexapi" productVersion="0.1.0" platform="Darwin" platformVersion="Darwin Kernel Version 17.0.0: Thu Aug 24 21:48:19 PDT 2017; root:xnu-4570.1.46~2/RELEASE_X86_64" device="Darwin" model="" vendor="" provides="" clientIdentifier="adasd.local" version="0.1.0" id="170878205" token="asdasdasda" createdAt="1511723047" lastSeenAt="1512241680" screenResolution="" screenDensity="">
  </Device>
  <Device name="Plex Web (Chrome)" publicAddress="213.12.213.123" product="Plex Web" productVersion="3.29.7" platform="Chrome" platformVersion="62.0" device="OSX" model="" vendor="" provides="client,player,pubsub-player" clientIdentifier="asdasdasda" version="3.29.7" id="169922920" token="asdasdasd" createdAt="1511557920" lastSeenAt="1512223248" screenResolution="720x738,1920x1080" screenDensity="">
  </Device>
  <Device name="Plex Web (Chrome)" publicAddress="213.12.213.123" product="Plex Web" productVersion="3.7.0" platform="Chrome" platformVersion="62.0" device="OSX" model="" vendor="" provides="client,player,pubsub-player" clientIdentifier="asdasdasd" version="3.7.0" id="167466380" token="asdasdasda" createdAt="1511038016" lastSeenAt="1512222815" screenResolution="1440x826,1440x900" screenDensity="">
  </Device>
  <Device name="WDMyCloudEX2" publicAddress="34.234.324.234" product="Plex Media Server" productVersion="1.7.2.324234" platform="Linux" platformVersion="3.2.40 (#4 Fri Jul 31 16:04:18 CST 2015)" device="PC" model="armv7" vendor="wd" provides="server" clientIdentifier="asdasdasdasdasd" version="1.7.2.3878-8088811b8" id="126466798" token="eK4pHRr5aJ4qoyHsoPxr" createdAt="1499898574" lastSeenAt="1512208843" screenResolution="" screenDensity="">
    <Connection uri="http://123.213.231:2344"/>
  </Device>
  <Device name="iPhone" publicAddress="813.12.213.123" product="Plex for iOS" productVersion="4.21" platform="iOS" platformVersion="11.1.2" device="iPhone" model="10,4" vendor="Apple" provides="client,controller,sync-target,player,pubsub-player" clientIdentifier="54BE1FC3-B6FD-4009-9FFC-AC7D7074F594" version="4.21" id="167471005" token="Hy36FcLXsu5TCBTYjPyV" createdAt="1511038600" lastSeenAt="1512174703" screenResolution="750x1334" screenDensity="2">
    <SyncList itemsCompleteCount="0" totalSize="0" version="2"/>
    <Connection uri="http://1234234sdasd:324234"/>
  </Device>
  <Device name="1234.local" publicAddress="21312.1231.12" product="PlexAPI" productVersion="3.0.4" platform="Darwin" platformVersion="17.0.0" device="Darwin" model="" vendor="" provides="controller" clientIdentifier="asddqwedas342" version="3.0.4" id="170804357" token="zwsW4byxiqRfwVn8rzqq" createdAt="1511713660" lastSeenAt="adsdsaasd" screenResolution="" screenDensity="">
  </Device>
  <Device name="Plex Web (Safari)" publicAddress="123.123.123" product="Plex Web" productVersion="3.7.0" platform="Safari" platformVersion="604.3" device="iOS" model="" vendor="" provides="client,player,pubsub-player" clientIdentifier="sads4asd32adsa" version="3.7.0" id="171015913" token="asdsadsda" createdAt="sadasd" lastSeenAt="1511741938" screenResolution="980x1445,375x667" screenDensity="">
  </Device>
  <Device name="Plex Web (Safari)" publicAddress="123.21.213" product="Plex Web" productVersion="3.7.0" platform="Safari" platformVersion="604.3" device="iOS" model="" vendor="" provides="client,player,pubsub-player" clientIdentifier="asdasdasdr32ad" version="3.7.0" id="169993398" token="asdasdasdfs" createdAt="1511567452" lastSeenAt="1511567461" screenResolution="980x1445,375x667" screenDensity="">
  </Device>
  <Device name="MSMBP" publicAddress="123.213.213" product="Plex Media Player" productVersion="3.26.2" platform="Konvergo" platformVersion="1.3.11.729-4be89b5c" device="OSX" model="10.13" vendor="" provides="client,player,pubsub-player" clientIdentifier="adsasdasdasdasd2" version="3.26.2" id="126467814" token="asddasdasd" createdAt="1499899100" lastSeenAt="1511556799" screenResolution="1280x720,1920x1080" screenDensity="">
    <Connection uri="http://12312312:43234"/>
    <Connection uri="http://1324.23.32423:3324"/>
  </Device>
  <Device name="Plex Web (Chrome)" publicAddress="321.123.233" product="Plex Web" productVersion="3.2.1" platform="Chrome" platformVersion="61.0" device="OSX" model="" vendor="" provides="client,player,pubsub-player" clientIdentifier="asdasdasdasd" version="3.2.1" id="159185640" token="asdasafafa" createdAt="1509217238" lastSeenAt="1509217294" screenResolution="1280x960,1920x1080" screenDensity="">
  </Device>
</MediaContainer>"##;
        let res: Result<DeviceContainer, Error> = deserialize(xml.as_bytes());
        assert!(res.is_ok());
    }
}