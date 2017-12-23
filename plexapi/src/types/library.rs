use types::server::Directory;
use client::PlexClient;
use errors::APIError;
use types::device::Connection;
use self::sections::*;
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


///
///
///
///
///
///
///
///
pub mod sections {
    use super::*;
    use types::media::video::*;
    use types::settings::X_PLEX_CONTAINER_SIZE;
    use std::ops::FnMut;
    use std::cmp::min;
    use std::borrow::Cow;
    use futures::future::{Loop, loop_fn};

    pub fn encode_utf8(input: Cow<str>) -> Cow<[u8]> {
        match input {
            Cow::Borrowed(s) => Cow::Borrowed(s.as_bytes()),
            Cow::Owned(s) => Cow::Owned(s.into_bytes())
        }
    }

    pub trait LibSectionFilter {
        fn format(&self) -> String;
    }


    struct Fetcher<T> {
        pub items: Vec<T>,
        pub max: u32,
        pub size: u32,
        pub start: u32,
    }

    ///
    pub trait LibSection: PlexTokenProvider {
        type Content;
        type Error;
        type Filter: LibSectionFilter;


        fn get<'a, F>(&'a self, filter: F) -> Box<Future<Item=Vec<Self::Content>, Error=Self::Error> + 'a>
            where F: 'a + FnMut(&Self::Content) -> bool {
            Box::new(self.all()
                .map(|content|
                    content.into_iter()
                        .filter(filter)
                        .collect::<Vec<_>>()))
        }

        fn all(&self) -> Box<Future<Item=Vec<Self::Content>, Error=Self::Error>> {
            let url = format!("{}{}/{}/all", self.connection().endpoint(), PlexLibrary::SECTIONS, self.key());
            self.fetch(url.as_str())
        }

        /// Returns a list of media items on deck from this library section.
        fn on_deck(&self) -> Box<Future<Item=Vec<Self::Content>, Error=Self::Error>> {
            let url = format!("{}{}/{}/onDeck", self.connection().endpoint(), PlexLibrary::SECTIONS, self.key());
            self.fetch(url.as_str())
        }

        fn search<'a>(&'a self, filter: Vec<Self::Filter>, max_results: Option<usize>) -> Box<Future<Item=Vec<Self::Content>, Error=Self::Error> + 'a> {
            let container_size = min(max_results.unwrap_or(X_PLEX_CONTAINER_SIZE), X_PLEX_CONTAINER_SIZE);

            let header = format!("={}", container_size);
            let query = match filter.is_empty() {
                false => format!("?{}&{}", filter.iter().map(|f| f.format()).filter(|p| !p.is_empty()).collect::<Vec<_>>().join("&"), header),
                _ => header
            };
            let url = format!("{}{}/{}/all{}", self.connection().endpoint(), PlexLibrary::SECTIONS, self.key(), query);

            println!("{}", url);
            let fetch_items = move |mut elements: Vec<Self::Content>| {
                self.fetch_container(url.as_str(), elements.len(), container_size).and_then(move |items| {
                    let fetched_size = items.len();
                    elements.extend(items.into_iter());

                    match max_results {
                        Some(s) => {
                            if elements.len() >= s { return Ok(Loop::Break(elements)); }
                        }
                        _ => ()
                    };
                    if fetched_size < container_size {
                        Ok(Loop::Break(elements))
                    } else {
                        Ok(Loop::Continue(elements))
                    }
                })
            };
            Box::new(loop_fn(Vec::new(), fetch_items))
        }


        /// need to be implemented in order to support custom deserialization
        fn fetch(&self, url: &str) -> Box<Future<Item=Vec<Self::Content>, Error=Self::Error>>;
        fn fetch_container(&self, url: &str, start: usize, max: usize) -> Box<Future<Item=Vec<Self::Content>, Error=Self::Error>>;

        fn section_type() -> SectionType;
        fn connection(&self) -> &Connection;
        fn key(&self) -> String;
    }

    #[derive(Debug)]
    pub struct PlexLibSection<'a> {
        pub inner: Section,
        client: Rc<PlexClient<'a>>,
        conn: Connection,
    }

    impl<'a> PlexLibSection<'a> {
        pub fn new(inner: Section, client: Rc<PlexClient<'a>>, conn: Connection) -> Self { PlexLibSection { inner, client, conn } }

        fn format_path(&self, part: &str) -> String {
            let path = format!("{}{}", self.inner.path(), part);
            self.conn.format_url(path.as_str(), self.client.token())
        }

        pub fn get(&self, title: &'a str) {
            let url = self.format_path("all");
            println!("{}", url);
        }

        pub fn into<T>(self) -> Option<T>
            where T: From<PlexLibSection<'a>> + LibSection {
            let section = self.inner.section_type()?;
            if section == T::section_type() {
                return Some(T::from(self));
            }
            None
        }
    }

    impl<'a> From<PlexLibSection<'a>> for MovieSection<'a> {
        fn from(inner: PlexLibSection<'a>) -> Self { MovieSection { inner } }
    }

    /// All allowed filters for a movie section
    pub enum MovieSectionFilter {
        Unwachted(bool),
        Duplicate(bool),
        Year(Vec<u16>),
        Decade(Vec<u8>),
        Genre(Vec<String>),
        ContentRating(u8),
        Collection(Vec<String>),
        Director(Vec<String>),
        Actor(Vec<String>),
        Country(String),
        Studio(Vec<String>),
        Resolution(String),
        Guid(String),
        Label(String),
    }

    fn encode(v: &Vec<String>) -> Cow<[u8]> { Cow::Owned(v.join(",").into_bytes()) }

    /// trait to supply the search query with adequate filters
    impl LibSectionFilter for MovieSectionFilter {
        fn format(&self) -> String {
            match *self {
                MovieSectionFilter::Unwachted(v) => format!("unwatched={}", v),
                MovieSectionFilter::Duplicate(v) => format!("duplicate={}", v),
                MovieSectionFilter::Year(ref v) => format!("year={}", v.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(",")),
                MovieSectionFilter::Decade(ref v) => format!("decade={}", v.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(",")),
                MovieSectionFilter::Genre(ref v) => format!("genre={:?}", encode(v)),
                MovieSectionFilter::Collection(ref v) => format!("collection={:?}", encode(v)),
                MovieSectionFilter::Director(ref v) => format!("director={:?}", encode(v)),
                MovieSectionFilter::Actor(ref v) => format!("actor={:?}", encode(v)),
                MovieSectionFilter::Country(ref v) => format!("country={}", v),
                MovieSectionFilter::Studio(ref v) => format!("studio={:?}", encode(v)),
                MovieSectionFilter::Resolution(ref v) => format!("resolution={}", v),
                MovieSectionFilter::Guid(ref v) => format!("guid={}", v),
                MovieSectionFilter::Label(ref v) => format!("label={}", v),
                _ => "".to_string()
            }
        }
    }

    #[derive(Debug)]
    pub struct MovieSection<'a> {
        inner: PlexLibSection<'a>
    }

    impl<'a> PlexTokenProvider for MovieSection<'a> {
        fn token(&self) -> PlexToken {
            self.inner.client.token()
        }
    }

    impl<'a> LibSection for MovieSection<'a> {
        type Content = Video;
        type Error = APIError;
        type Filter = MovieSectionFilter;

        fn fetch(&self, url: &str) -> Box<Future<Item=Vec<Self::Content>, Error=Self::Error>> {
            println!("{}", url);
            Box::new(self.inner.client
                .get_xml::<VideoContainer>(url)
                .map(move |container| container.videos))
        }

        fn fetch_container(&self, url: &str, start: usize, max: usize) -> Box<Future<Item=Vec<Self::Content>, Error=Self::Error>> {
            Box::new(self.inner.client
                .get_xml_container::<VideoContainer>(url, start, max)
                .map(move |container| container.videos))
        }

        fn connection(&self) -> &Connection { &self.inner.conn }

        fn key(&self) -> String { self.inner.inner.key.clone() }

        fn section_type() -> SectionType { SectionType::Movie }
    }


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

    #[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
    #[serde(tag = "type")]
    pub enum SectionType {
        Movie,
        Photo,
        Music,
        Show,
    }

    impl SectionType {
        pub fn as_str(&self) -> &str {
            match *self {
                SectionType::Movie => "movie",
                SectionType::Photo => "photo",
                SectionType::Music => "music",
                SectionType::Show => "show",
            }
        }

        pub fn from_str(type_name: &str) -> Option<SectionType> {
            let s = type_name.to_lowercase();
            match s.as_str() {
                "movie" => Some(SectionType::Movie),
                "photo" => Some(SectionType::Photo),
                "music" => Some(SectionType::Music),
                "show" => Some(SectionType::Show),
                _ => None
            }
        }
    }

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


    impl Section {
        pub const PATH: &'static str = "/library/sections/";

        pub fn path(&self) -> String { format!("{}{}/", Section::PATH, self.key) }

        pub fn section_type(&self) -> Option<SectionType> { SectionType::from_str(self.type_.as_str()) }
    }

    #[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
    pub struct Location {
        id: String,
        path: String,
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
        fn as_str(&self) -> &str;
        fn from_str(filter: &str) -> Option<Self::Filter>;
    }


    impl SectionFilter for MovieFilter {
        type Filter = MovieFilter;

        fn as_str(&self) -> &str {
            match *self {
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

    impl<'a> LibrarySection for MovieSection<'a> {
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