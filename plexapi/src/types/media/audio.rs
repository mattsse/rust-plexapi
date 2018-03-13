use client::PlexClient;
use types::device::Connection;
use std::rc::Rc;
use futures::Future;
use errors::APIError;

pub trait Audio {}

pub struct PlexAlbum<'a> {
    pub inner: Album,
    conn: Connection,
    client: Rc<PlexClient<'a>>,
}

impl<'a> PlexAlbum<'a> {
    pub fn new(inner: Album, conn: Connection, client: Rc<PlexClient<'a>>) -> Self {
        PlexAlbum {
            inner,
            conn,
            client,
        }
    }

    pub fn tracks(&self) -> impl Future<Item = Vec<Track>, Error = APIError> {
        let url = format!("{}/{}/children", self.conn.endpoint(), self.inner.key);
        self.client
            .get_xml::<TrackContainer>(url.as_str())
            .map(move |container| container.tracks)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct AlbumContainer {
    pub size: String,
    pub allow_sync: String,
    pub art: String,
    pub identifier: String,
    pub media_tag_prefix: String,
    pub media_tag_version: String,
    pub mixed_parents: String,
    pub nocache: String,
    pub thumb: String,
    pub title1: String,
    pub title2: String,
    pub view_group: String,
    pub view_mode: String,
    #[serde(rename = "Directory", default)]
    pub albums: Vec<Album>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Album {
    pub allow_sync: String,
    #[serde(rename = "librarySectionID")]
    pub library_section_id: String,
    pub library_section_title: String,
    #[serde(rename = "librarySectionUUID")]
    pub library_section_uuid: String,
    pub rating_key: String,
    pub key: String,
    pub parent_rating_key: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub title: String,
    pub parent_key: String,
    pub parent_title: String,
    pub summary: String,
    pub index: String,
    pub year: String,
    pub thumb: String,
    pub parent_thumb: String,
    pub originally_available_at: String,
    pub leaf_count: String,
    pub added_at: String,
    pub updated_at: String,
    pub last_viewed_at: String,
    pub view_count: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct TrackContainer {
    pub size: String,
    pub allow_sync: String,
    pub art: String,
    pub identifier: String,
    pub grandparent_rating_key: String,
    pub grandparent_thumb: String,
    pub grandparent_title: String,
    #[serde(rename = "librarySectionID")]
    pub library_section_id: String,
    pub library_section_title: String,
    #[serde(rename = "librarySectionUUID")]
    pub library_section_uuid: String,
    pub media_tag_prefix: String,
    pub media_tag_version: String,
    pub nocache: String,
    pub parent_index: String,
    pub parent_title: String,
    pub parent_year: String,
    pub thumb: String,
    pub title1: String,
    pub title2: String,
    pub view_group: String,
    pub view_mode: String,
    #[serde(rename = "Track")]
    pub tracks: Vec<Track>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Track {
    pub rating_key: String,
    pub key: String,
    pub parent_rating_key: String,
    pub grandparent_rating_key: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub title: String,
    pub grandparent_key: String,
    pub parent_key: String,
    pub grandparent_title: String,
    pub parent_title: String,
    pub original_title: String,
    pub summary: String,
    pub index: String,
    pub parent_index: String,
    pub rating_count: String,
    pub thumb: String,
    pub parent_thumb: String,
    pub grandparent_thumb: String,
    pub duration: String,
    pub added_at: String,
    pub updated_at: String,
    #[serde(rename = "Media")]
    pub media: Vec<TrackMedia>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct TrackMedia {
    pub id: String,
    pub duration: String,
    pub bitrate: String,
    pub audio_channels: String,
    pub audio_codec: String,
    pub container: String,
    #[serde(rename = "Part")]
    pub part: TrackMediaPart,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct TrackMediaPart {
    pub id: String,
    pub key: String,
    pub duration: String,
    pub file: String,
    pub size: String,
    pub container: String,
    pub has_thumbnail: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_xml_rs::{deserialize, Error};

    #[test]
    fn album_container_deserialize() {
        let xml = r##"<MediaContainer size="2" allowSync="0"
        art="/:/resources/artist-fanart.jpg" identifier="com.plexapp.plugins.library"
        mediaTagPrefix="/system/bundle/media/flags/" mediaTagVersion="1513137264" mixedParents="1"
        nocache="1" thumb="/:/resources/artist.png" title1="Music" title2="By Album"
        viewGroup="album" viewMode="65592">
<Directory allowSync="1" librarySectionID="3" librarySectionTitle="Music"
librarySectionUUID="d4069239-bad6-41d3-ab69aasdasdasd3" ratingKey="14"
key="/library/metadata/14/children" parentRatingKey="13" type="album"
title="Title" parentKey="/library/metadata/13" parentTitle="Artist1 and Artist2"
summary="" index="1" year="2016" thumb="/library/metadata/14/thumb/1514065023"
parentThumb="/library/metadata/13/thumb/asdasdas" originallyAvailableAt="2016-2-23"
leafCount="6" addedAt="1514064996" updatedAt="234123412">
</Directory>
        </MediaContainer>
"##;
        let server: Result<AlbumContainer, Error> = deserialize(xml.as_bytes());
        assert!(server.is_ok());
    }

    #[test]
    fn track_container_deserialize() {
        let xml = r##"<MediaContainer size="6" allowSync="1"
        art="/:/resources/artist-fanart.jpg"
        grandparentRatingKey="13" grandparentThumb="/library/metadata/13/thumb/1514065011"
        grandparentTitle="Artist1 and Artist2" identifier="com.plexapp.plugins.library" key="14"
        librarySectionID="3" librarySectionTitle="Musik"
        librarySectionUUID="d4069239-bad6-41d3-ab69-f2887d5f09b3"
        mediaTagPrefix="/system/bundle/media/flags/"
        mediaTagVersion="1513137264" nocache="1" parentIndex="1" parentTitle="title"
        parentYear="2016" thumb="/library/metadata/14/thumb/1514065023" title1="Artist1 and Artist2"
        title2="title2" viewGroup="track" viewMode="65593">

        <Track ratingKey="15" key="/library/metadata/15" parentRatingKey="14"
        grandparentRatingKey="13" type="track" title="title"
        grandparentKey="/library/metadata/13" parentKey="/library/metadata/14"
        grandparentTitle="title" parentTitle="title"
        originalTitle="Aasdasd" summary="" index="1" parentIndex="1" ratingCount="1452"
        thumb="/library/metadata/14/thumb/1514065023"
        parentThumb="/library/metadata/14/thumb/1514065023"
        grandparentThumb="/library/metadata/13/thumb/1514065011" duration="197899"
        addedAt="1514064996" updatedAt="1514065023">
<Media id="6" duration="197899" bitrate="275" audioChannels="2" audioCodec="mp3" container="mp3">
<Part id="6" key="/library/parts/6/123123/file.mp3" duration="197899"
file="/a.mp3" size="6797425" container="mp3" hasThumbnail="1"/>
</Media>
</Track>
        </MediaContainer>
"##;
        let server: Result<TrackContainer, Error> = deserialize(xml.as_bytes());
        assert!(server.is_ok());
    }
}
