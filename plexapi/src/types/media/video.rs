#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Video {
    pub rating_key: String,
    pub key: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub title: String,
    pub summary: String,
    pub view_offset: String,
    pub last_viewed_at: String,
    pub year: String,
    pub thumb: String,
    pub art: String,
    pub duration: String,
    pub originally_available_at: String,
    pub added_at: String,
    pub updated_at: String,
    pub created_at_accuracy: String,
    #[serde(rename = "createdAtTZOffset")]
    pub created_at_tz_offset: String,
    #[serde(rename = "Media")]
    pub media: Vec<VideoMedia>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct VideoMedia {
    video_resolution: String,
    id: String,
    duration: String,
    bitrate: String,
    width: String,
    height: String,
    aspect_ratio: String,
    audio_channels: String,
    audio_codec: String,
    video_codec: String,
    container: String,
    video_frame_rate: String,
    optimized_for_streaming: Option<String>,
    audio_profile: String,
    has64bit_offsets: Option<String>,
    video_profile: String,
    #[serde(rename = "Part")]
    media: Part,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Part {
    id: String,
    key: String,
    duration: String,
    file: String,
    size: String,
    audio_profile: String,
    container: String,
    has64bit_offsets: String,
    optimized_for_streaming: String,
    video_profile: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VideoContainer {
    pub size: String,
    pub allow_sync: String,
    pub art: String,
    pub identifier: String,
    #[serde(rename = "librarySectionID")]
    pub library_section_id: String,
    pub library_section_title: String,
    #[serde(rename = "librarySectionUUID")]
    pub library_section_uuid: String,
    pub media_tag_prefix: String,
    pub media_tag_version: String,
    pub thumb: String,
    pub title1: String,
    pub title2: String,
    pub view_group: String,
    pub view_mode: String,
    #[serde(rename = "Video", default)]
    pub videos: Vec<Video>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_xml_rs::{deserialize, Error};

    #[test]
    fn video_container_deserialize() {
        let xml = r##"<MediaContainer size="1068" allowSync="1"
        art="/:/resources/movie-fanart.jpg"
        identifier="com.plexapp.plugins.library" librarySectionID="3"
        librarySectionTitle="Name" librarySectionUUID="4324234234234239"
        mediaTagPrefix="/system/bundle/media/flags/" mediaTagVersion="1495837492"
        thumb="/:/resources/video.png"
        title1="Title" title2="All Title" viewGroup="movie" viewMode="65592">
        <Video ratingKey="444" key="/library/metadata/444"
        type="movie" title="title" summary="" year="2015" thumb="/library/metadata/213/thumb/123"
        art="/library/metadata/1232/art/21321" duration="123123" originallyAvailableAt="212312"
        addedAt="123123" updatedAt="213123" createdAtAccuracy="epoch,local"
        createdAtTZOffset="0">
  <Media videoResolution="123312" id="213" duration="2132" bitrate="123" width="123" height="1231"
  aspectRatio="1.78" audioChannels="2" audioCodec="aac" videoCodec="h264"
  container="mp4" videoFrameRate="NTSC" optimizedForStreaming="1" audioProfile="lc"
  has64bitOffsets="0" videoProfile="high">
    <Part id="443" key="/library/parts/443/234123/file.mp4" duration="2170971" file="file.mp4"
    size="123123" audioProfile="lc" container="mp4" has64bitOffsets="0" optimizedForStreaming="1"
    videoProfile="high"/>
  </Media>
</Video>
        </MediaContainer>
"##;
        let server: Result<VideoContainer, Error> = deserialize(xml.as_bytes());
        assert!(server.is_ok());
    }

    #[test]
    fn video_deserialize() {
        let xml = r##"<Video ratingKey="1067" key="/library/metadata/1067"
        type="movie" title="Some Title" summary="" year="2017" thumb="/library/metadata/1067/thumb/234234"
         art="/library/metadata/1067/art/234234" duration="2361563"
         originallyAvailableAt="2017-07-16" addedAt="1511730328"
         updatedAt="1511735264" createdAtAccuracy="epoch" createdAtTZOffset="-25200">
<Media videoResolution="1080" id="1065" duration="2361563" bitrate="6911" width="1920" height="1080"
aspectRatio="1.78" audioChannels="2" audioCodec="aac" videoCodec="h264" container="mp4"
videoFrameRate="NTSC" optimizedForStreaming="0" audioProfile="lc" has64bitOffsets="0"
videoProfile="high">
<Part id="1065" key="/library/parts/1065/1500211236/file.mp4" duration="2361563"
file="/path/to/file" size="2040047333" audioProfile="lc" container="mp4" has64bitOffsets="0"
optimizedForStreaming="0" videoProfile="high"/>
</Media>
</Video>
"##;

        let server: Result<Video, Error> = deserialize(xml.as_bytes());
        assert!(server.is_ok());
    }
}
