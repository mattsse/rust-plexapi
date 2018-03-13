use client::PlexClient;
use types::library::{Library, PlexLibrary};
use futures::Future;
use errors::APIError;
use types::device::Connection;
use types::PlexTokenProvider;
use std::rc::Rc;

#[derive(Debug)]
pub struct PlexServer<'a> {
    pub inner: Server,
    client: Rc<PlexClient<'a>>,
    conn: Connection,
}

impl<'a> PlexServer<'a> {
    pub fn new(inner: Server, client: Rc<PlexClient<'a>>, conn: Connection) -> Self {
        PlexServer {
            inner,
            client,
            conn,
        }
    }

    pub fn library(&self) -> impl Future<Item = PlexLibrary<'a>, Error = APIError> {
        let client = Rc::clone(&self.client);
        let url = self.conn.format_url(PlexLibrary::PATH, self.client.token());
        let conn = self.conn.clone();
        client
            .get_xml::<Library>(url.as_str())
            .map(move |library| PlexLibrary::new(library, client, conn))
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Server {
    size: String,
    allow_camera_upload: String,
    allow_channel_access: String,
    allow_sharing: String,
    allow_sync: String,
    background_processing: String,
    certificate: String,
    companion_proxy: String,
    country_code: String,
    diagnostics: String,
    event_stream: String,
    friendly_name: String,
    hub_search: String,
    item_clusters: String,
    machine_identifier: String,
    media_providers: String,
    multiuser: String,
    my_plex: String,
    my_plex_mapping_state: String,
    my_plex_signin_state: String,
    my_plex_subscription: String,
    my_plex_username: String,
    owner_features: String,
    photo_auto_tag: String,
    platform: String,
    platform_version: String,
    plugin_host: String,
    read_only_libraries: String,
    request_parameters_in_cookie: String,
    #[serde(rename = "streamingBrainABRVersion")]
    streaming_brain_abr_version: String,
    streaming_brain_version: String,
    sync: String,
    transcoder_active_video_sessions: String,
    transcoder_audio: String,
    transcoder_lyrics: String,
    transcoder_photo: String,
    transcoder_subtitles: String,
    transcoder_video: String,
    transcoder_video_bitrates: String,
    transcoder_video_qualities: String,
    transcoder_video_remux_only: String,
    transcoder_video_resolutions: String,
    updated_at: String,
    updater: String,
    version: String,
    voice_search: String,

    #[serde(rename = "Directory")]
    pub directories: Vec<Directory>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Directory {
    count: String,
    key: String,
    title: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_xml_rs::{deserialize, Error};

    #[test]
    fn device_connect_deserialize() {
        let xml = r##"<?xml version="1.0" encoding="UTF-8"?>
<MediaContainer size="21" allowCameraUpload="0" allowChannelAccess="1"
allowSharing="1" allowSync="0" backgroundProcessing="1"
certificate="1" companionProxy="1" countryCode="deu"
diagnostics="logs,databases" eventStream="1" friendlyName="Cloud"
hubSearch="1" itemClusters="1"
machineIdentifier="asdasdasdasdas" mediaProviders="1" multiuser="1"
myPlex="1" myPlexMappingState="mapped"
myPlexSigninState="ok" myPlexSubscription="0"
myPlexUsername="max.muster@mail.org"
ownerFeatures="Android - PiP,adaptive_bitrate,download_certificates,federated-auth,
ios-rating-modal,loudness,news,radio" photoAutoTag="1"
platform="Linux"
platformVersion="3.2.40 (#4 Fri Jul 31 16:04:18 CST 2015)"
pluginHost="1" readOnlyLibraries="0"
requestParametersInCookie="1" streamingBrainABRVersion="1"
streamingBrainVersion="2" sync="1" transcoderActiveVideoSessions="0"
transcoderAudio="1" transcoderLyrics="1" transcoderPhoto="1"
 transcoderSubtitles="1" transcoderVideo="1"
 transcoderVideoBitrates="64,96,208,320,720,1500,2000,3000,4000,8000,10000,12000,20000"
 transcoderVideoQualities="0,1,2,3,4,5,6,7,8,9,10,11,12"
 transcoderVideoRemuxOnly="1"
 transcoderVideoResolutions="128,128,160,240,320,480,768,720,720,1080,1080,1080,1080"
 updatedAt="1512345212" updater="1" version="1.7.2.3878-8088811b8" voiceSearch="1">
</MediaContainer>
"##;

        let server: Result<Server, Error> = deserialize(xml.as_bytes());
        assert!(server.is_ok());
    }

    #[test]
    fn device_connect_deserialize() {
        let xml = r##"<?xml version="1.0" encoding="UTF-8"?>
<MediaContainer size="21" allowCameraUpload="0" allowChannelAccess="1"
allowSharing="1" allowSync="0" backgroundProcessing="1"
certificate="1" companionProxy="1" countryCode="deu"
diagnostics="logs,databases" eventStream="1" friendlyName="Cloud"
hubSearch="1" itemClusters="1"
machineIdentifier="asdasdasdasdas" mediaProviders="1" multiuser="1"
myPlex="1" myPlexMappingState="mapped"
myPlexSigninState="ok" myPlexSubscription="0"
myPlexUsername="max.muster@mail.org"
ownerFeatures="Android - PiP,adaptive_bitrate,download_certificates,federated-auth,
ios-rating-modal,loudness,news,radio" photoAutoTag="1"
platform="Linux"
platformVersion="3.2.40 (#4 Fri Jul 31 16:04:18 CST 2015)"
pluginHost="1" readOnlyLibraries="0"
requestParametersInCookie="1" streamingBrainABRVersion="1"
streamingBrainVersion="2" sync="1" transcoderActiveVideoSessions="0"
transcoderAudio="1" transcoderLyrics="1" transcoderPhoto="1"
 transcoderSubtitles="1" transcoderVideo="1"
 transcoderVideoBitrates="64,96,208,320,720,1500,2000,3000,4000,8000,10000,12000,20000"
 transcoderVideoQualities="0,1,2,3,4,5,6,7,8,9,10,11,12"
 transcoderVideoRemuxOnly="1"
 transcoderVideoResolutions="128,128,160,240,320,480,768,720,720,1080,1080,1080,1080"
 updatedAt="1512345212" updater="1" version="1.7.2.3878-8088811b8" voiceSearch="1">
</MediaContainer>
"##;

        let server: Result<Server, Error> = deserialize(xml.as_bytes());
        assert!(server.is_ok());
    }
}
