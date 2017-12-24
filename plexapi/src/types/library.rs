use types::server::Directory;
use client::PlexClient;
use errors::APIError;
use types::device::Connection;
use types::sections::*;
use types::{PlexToken, PlexTokenProvider};
use futures::{Future, future};
use std::rc::Rc;

#[derive(Debug)]
pub struct PlexLibrary<'a> {
    inner: Library,
    client: Rc<PlexClient<'a>>,
    conn: Connection,
}

impl<'a> PlexLibrary<'a> {
    pub const PATH: &'static str = "/library";
    pub const SECTIONS: &'static str = "/library/sections";

    pub fn new(inner: Library, client: Rc<PlexClient<'a>>, conn: Connection) -> Self { PlexLibrary { inner, client, conn } }

    pub fn sections(&self) -> impl Future<Item=Vec<PlexLibSection<'a>>, Error=APIError> {
        let client = Rc::clone(&self.client);
        let url = format!("{}{}", self.conn.endpoint(), PlexLibrary::SECTIONS);
        let conn = self.conn.clone();
        self.client.get_xml::<Sections>(url.as_str()).map(move |container| {
            container.sections
                .into_iter()
                .map(|section| PlexLibSection::new(section, Rc::clone(&client), conn.clone())).collect::<Vec<_>>()
        })
    }
    //
    pub fn section(&self, title: &'a str) -> impl Future<Item=PlexLibSection<'a>, Error=APIError> {
        self.sections().and_then(move |sections|
            match sections.into_iter().find(|p| p.inner.title.eq(title)) {
                Some(s) => future::ok(s),
                _ => future::err(APIError::ReadError)
            }
        )
    }

    fn sections_by_type(&self, section_type: SectionType) -> impl Future<Item=Vec<PlexLibSection<'a>>, Error=APIError> {
        self.sections().map(move |sections| {
            let type_name = section_type.as_str();
            sections.into_iter()
                .filter(|p| p.inner.type_.eq(type_name))
                .collect::<Vec<_>>()
        })
    }

    pub fn section_by_id(&self, id: &'a str) -> impl Future<Item=PlexLibSection<'a>, Error=APIError> {
        self.sections().and_then(move |sections|
            match sections.into_iter().find(|p| p.inner.uuid.eq(id)) {
                Some(s) => future::ok(s),
                _ => future::err(APIError::ReadError)
            })
    }

    pub fn movie_sections(&self) -> impl Future<Item=Vec<MovieSection<'a>>, Error=APIError> {
        self.sections_by_type(SectionType::Movie)
            .map(|sections| sections.into_iter()
                .map(|s| MovieSection::from(s))
                .collect::<Vec<_>>())
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
    #[serde(rename = "title1", default)]
    title: String,
    #[serde(rename = "Directory", default)]
    directories: Vec<Directory>,
}




#[cfg(test)]
mod test {
    use super::*;
    use types::sections::*;
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

    #[test]
    fn section_desirialize() {
        let xml = r##"<?xml version="1.0" encoding="UTF-8"?>
<MediaContainer size="3" allowSync="0" identifier="com.plexapp.plugins.library"
mediaTagPrefix="/system/bundle/media/flags/" mediaTagVersion="1495837492" title1="Plex Library">
<Directory allowSync="0" art="/:/resources/movie-fanart.jpg"
composite="/library/sections/4/composite/1500101897"
filters="1" refreshing="0" thumb="/:/resources/video.png"
key="4" type="movie" title="fullmovies"
agent="com.plexapp.agents.none" scanner="Plex Video Files Scanner" language="xn"
uuid="3ed60fb5-4c77-4400-9713-3d7fb36a944d" updatedAt="1500101897" createdAt="1500101894">
<Location id="4" path="/shares/Volume8TB/Movies" />
</Directory>
<Directory allowSync="0" art="/:/resources/movie-fanart.jpg" composite="/library/sections/3/composite/1499984524"
filters="1" refreshing="0" thumb="/:/resources/video.png" key="3" type="movie" title="Sites"
agent="com.plexapp.agents.none" scanner="Plex Video Files Scanner" language="xn" uuid="30e88909-60eb-4790-b88e-8b61aaa62c99"
updatedAt="1499984524" createdAt="1499984523">
<Location id="3" path="/shares/Volume8TB/Sites" />
</Directory>
<Directory allowSync="0" art="/:/resources/movie-fanart.jpg"
composite="/library/sections/1/composite/1499899721" filters="1"
refreshing="0" thumb="/:/resources/movie.png" key="1"
type="movie" title="Test" agent="com.plexapp.agents.imdb"
scanner="Plex Movie Scanner" language="en"
uuid="4d051575-b6f2-4691-b83b-520fe1e97ef1"
updatedAt="1499899721" createdAt="1499898810">
<Location id="1" path="/shares/Volume8TB/[toOrder" />
</Directory>
</MediaContainer>"##;

        let sections: Result<Sections, Error> = deserialize(xml.as_bytes());
        assert!(sections.is_ok());
    }
}