use types::server::{PlexServer, Directory};
use self::sections::*;

#[derive(Debug)]
pub struct PlexLibrary<'a> {
    inner: Library,
    server: &'a PlexServer<'a>,
}

impl<'a> PlexLibrary<'a> {
    pub const PATH: &'static str = "/library";
    pub const SECTIONS: &'static str = "/library/sections";

    pub fn new(inner: Library, server: &'a PlexServer<'a>) -> Self { PlexLibrary { inner, server } }

//    pub fn sections(&self) -> Result<Vec<Section>, PlexError> {
//        let req = PlexLibrarySectionsRequest::new(self.server);
//        self.server.submit(req)
//    }
//
//    pub fn section(self, title: &str) -> Option<Section> {
//        match self.sections() {
//            Ok(s) => {
//                s.into_iter()
//                    .find(|p| p.title.eq(title))
//            }
//            _ => None
//        }
//    }
//
//    pub fn sections_by_type(self, t: SectionType) -> Option<Vec<Section>> {
//        match self.sections() {
//            Ok(s) => {
//                let type_name = t.as_str();
//                Some(s.into_iter()
//                    .filter(|p| p.type_.eq(type_name))
//                    .collect::<Vec<_>>())
//            }
//            _ => None
//        }
//    }
//
//
//    pub fn section_by_id(self, id: &str) -> Option<Section> {
//        match self.sections() {
//            Ok(s) => {
//                s.into_iter().find(|p| p.uuid.eq(id))
//            }
//            _ => None
//        }
//    }
}

//impl<'a> PlexTokenProvider for PlexLibrary<'a> {
//    fn token(&self) -> &PlexToken {
//        self.server.token()
//    }
//}

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
    directories: Vec<Directory>,
}


pub mod sections {
    use super::*;

    pub struct MediaFilter(String);

    pub trait LibrarySection {
        fn from_section(section: Section) -> Self;
        fn allowed_filters() -> Vec<String>;
    }

    #[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct Sections {
        size: String,
        allow_sync: String,
        identifier: String,
        media_tag_prefix: String,
        media_tag_version: String,
        title1: String,
        #[serde(rename = "Directory", default)]
        pub sections: Vec<Section>,
    }

    impl Sections {}

    #[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
    #[serde(tag = "type")]
    pub enum SectionType {
        Movie,
        Photo,
        Music,
        Show,
    }

    impl SectionType {
        pub fn as_str<'a>(self) -> &'a str {
            match self {
                SectionType::Movie => "movie",
                SectionType::Photo => "photo",
                SectionType::Music => "music",
                SectionType::Show => "show",
            }
        }
    }

//    #[derive(Debug, Clone)]
//    pub struct PlexSection {
//        inner: Section,
//        session: Arc<Session>
//    }
//
//    impl PlexSection {
//        pub fn new(inner: Section, session: Arc<Session>) -> Self {
//            PlexSection { inner, session }
//        }
//    }


    #[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct Section {
        pub allow_sync: String,
        pub art: String,
        pub composite: String,
        pub filters: String,
        pub refreshing: String,
        pub thumb: String,
        pub key: String,
        #[serde(rename = "type")]
        pub type_: String,
        pub  title: String,
        pub  agent: String,
        pub  scanner: String,
        pub  language: String,
        pub  uuid: String,
        pub  updated_at: String,
        pub  created_at: String,
        #[serde(rename = "Location")]
        pub location: Location,
    }


    #[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
    pub struct Location {
        id: String,
        path: String,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct MovieSection {
        inner: Section
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum MovieFilter {
        Unwatched,
        Duplicate,
        Year,
        Decade,
        Genre,
        ContentRating,
        Collection,
        Director,
        Actor,
        Country,
        Studio,
        Resolution,
        Guid,
        Label,
    }

    trait SectionFilter {
        type Filter: SectionFilter;
        fn as_str<'a>(self) -> &'a str;
        fn from_str(filter: &str) -> Option<Self::Filter>;
    }


    impl SectionFilter for MovieFilter {
        type Filter = MovieFilter;

        fn as_str<'a>(self) -> &'a str {
            match self {
                MovieFilter::Unwatched => "unwatched",
                MovieFilter::Duplicate => "duplicate",
                MovieFilter::Year => "year",
                MovieFilter::Decade => "decade",
                MovieFilter::Genre => "genre",
                MovieFilter::ContentRating => "contentRating",
                MovieFilter::Collection => "collection",
                MovieFilter::Director => "director",
                MovieFilter::Actor => "actor",
                MovieFilter::Country => "country",
                MovieFilter::Studio => "studio",
                MovieFilter::Resolution => "resolution",
                MovieFilter::Guid => "guid",
                MovieFilter::Label => "label"
            }
        }
        fn from_str(filter: &str) -> Option<MovieFilter> {
            let s = filter.to_lowercase();
            match s.as_str() {
                "unwatched" => Some(MovieFilter::Unwatched),
                "duplicate" => Some(MovieFilter::Duplicate),
                "year" => Some(MovieFilter::Year),
                "decade" => Some(MovieFilter::Decade),
                "genre" => Some(MovieFilter::Genre),
                "contentRating" => Some(MovieFilter::ContentRating),
                "collection" => Some(MovieFilter::Collection),
                "director" => Some(MovieFilter::Director),
                "actor" => Some(MovieFilter::Actor),
                "country" => Some(MovieFilter::Country),
                "studio" => Some(MovieFilter::Studio),
                "resolution" => Some(MovieFilter::Resolution),
                "guid" => Some(MovieFilter::Guid),
                "label" => Some(MovieFilter::Label),
                _ => None
            }
        }
    }

    impl LibrarySection for MovieSection {
        fn allowed_filters() -> Vec<String> {
            vec!["".to_owned()]
        }
        fn from_section(section: Section) -> Self {
            unimplemented!()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::sections::*;
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