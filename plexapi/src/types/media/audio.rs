#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MusicContainer {
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
    allow_sync: String,
    #[serde(rename = "librarySectionID")]
    library_section_id: String,
    library_section_title: String,
    #[serde(rename = "librarySectionUUID")]
    library_section_uuid: String,
    rating_key: String,
    key: String,
    parent_rating_key: String,
    #[serde(rename = "type")]
    type_: String,
    title: String,
    parent_key: String,
    parent_title: String,
    summary: String,
    index: String,
    year: String,
    thumb: String,
    parent_thumb: String,
    originally_available_at: String,
    leaf_count: String,
    added_at: String,
    updated_at: String,
    last_viewed_at: String,
    view_count: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_xml_rs::{deserialize, Error};

    #[test]
    fn audio_container_deserialize() {
        let xml = r##"<MediaContainer size="2" allowSync="0"
        art="/:/resources/artist-fanart.jpg" identifier="com.plexapp.plugins.library"
        mediaTagPrefix="/system/bundle/media/flags/" mediaTagVersion="1513137264" mixedParents="1"
        nocache="1" thumb="/:/resources/artist.png" title1="Musik" title2="By Album"
        viewGroup="album" viewMode="65592">
<Directory
allowSync="1"
librarySectionID="3"
librarySectionTitle="Music"
librarySectionUUID="d4069239-bad6-41d3-ab69aasdasdasd3"
ratingKey="14"
key="/library/metadata/14/children"
parentRatingKey="13"
type="album"
title="Title"
parentKey="/library/metadata/13"
parentTitle="Artist1 and Artist2"
summary=""
index="1"
year="2016"
thumb="/library/metadata/14/thumb/1514065023"
parentThumb="/library/metadata/13/thumb/asdasdas"
originallyAvailableAt="2016-2-23"
leafCount="6"
addedAt="1514064996"
updatedAt="234123412">
</Directory>
        </MediaContainer>
"##;
        let server: Result<MusicContainer, Error> = deserialize(xml.as_bytes());
        assert!(server.is_ok());
    }
}
