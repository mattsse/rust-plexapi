use http::request::*;
use super::types::*;
use std::io::Read;

#[derive(Debug)]
pub struct PlexLibrary<'a> {
    inner: Library,
    server: &'a PlexServer<'a>
}

impl<'a> PlexLibrary<'a> {
    pub const PATH: &'static str = "/library";
    pub const SECTIONS: &'static str = "/library/sections";

    pub fn new(inner: Library, server: &'a PlexServer<'a>) -> Self { PlexLibrary { inner, server } }

    pub fn sections(&self) {
        let req = PlexLibrarySectionsRequest::new(self.server);
        let mut resp = self.server.submit(req).unwrap();
        let mut s = String::new();
        resp.read_to_string(&mut s);
        println!("{}", s)
    }
}

impl<'a> PlexTokenProvider for PlexLibrary<'a> {
    fn token(&self) -> &PlexToken {
        self.server.token()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Library {
    size: String,
    allow_sync: String,
    art: String,
    content: String,
    identifier: String,
    media_tag_prefix: String,
    media_tag_version: String,
    title1: String,
    #[serde(rename = "Directory", default)]
    directories: Vec<Directory>
}


pub mod sections {
    pub trait LibrarySection {
        fn allowed_filters() -> Vec<String>;
    }

    #[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct MovieSection {}
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_xml_rs::{deserialize, Error};

    #[test]
    fn library_deserialize() {
        let xml = r##"<?xml version="1.0" encoding="UTF-8"?>
<MediaContainer size="3" allowSync="0" art="/:/resources/library-art.png" content=""
identifier="com.plexapp.plugins.library"
mediaTagPrefix="/system/bundle/media/flags/"
mediaTagVersion="1495837492" title1="Plex Library" title2="">
<Directory key="sections" title="Library Sections" />
<Directory key="recentlyAdded" title="Recently Added Content" />
<Directory key="onDeck" title="On Deck Content" />
</MediaContainer>"##;
        let lib: Result<Library, Error> = deserialize(xml.as_bytes());
        assert!(lib.is_ok());
    }
}