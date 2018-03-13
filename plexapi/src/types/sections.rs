use types::library::PlexLibrary;
use types::media::video::*;
use types::media::audio::*;
use types::settings::X_PLEX_CONTAINER_SIZE;
use types::device::Connection;
use types::{PlexToken, PlexTokenProvider};
use types::filter::*;
use client::{PlexClient, PlexClientProvider};
use errors::APIError;
use futures::Future;
use std::ops::FnMut;
use std::cmp::min;
use futures::future::{loop_fn, Loop};
use std::rc::Rc;
use log::LogLevel;

///
///
///
///
///
///
///
#[derive(Debug, Clone)]
pub struct PlexLibSection<'a> {
    pub inner: Section,
    client: Rc<PlexClient<'a>>,
    conn: Connection,
}

impl<'a> PlexClientProvider<'a> for PlexLibSection<'a> {
    fn client(&self) -> &Rc<PlexClient<'a>> {
        &self.client
    }
}

impl<'a> PlexLibSection<'a> {
    pub fn new(inner: Section, client: Rc<PlexClient<'a>>, conn: Connection) -> Self {
        PlexLibSection {
            inner,
            client,
            conn,
        }
    }

    fn format_path(&self, part: &str) -> String {
        let path = format!("{}{}", self.inner.path(), part);
        self.conn.format_url(path.as_str(), self.client.token())
    }

    pub fn get(&self, title: &'a str) {
        let url = self.format_path("all");
    }

    pub fn into<T>(self) -> Option<T>
    where
        T: From<PlexLibSection<'a>> + LibrarySection<'a>,
    {
        let section = self.inner.section_type()?;
        if section == T::section_type() {
            return Some(T::from(self));
        }
        log!(
            LogLevel::Error,
            "Type Mismatch for Library section, expected {:?}, got {:?}",
            section,
            T::section_type()
        );
        None
    }
}

//let fetch_items = |(lib, elements):
//(Self, Vec<Self::Content>)|
//{
////                    let x = lib.fetch("");
////                    Box::new(lib.fetch_container("", elements.len(), 200usize).and_then(|items| {
//////                    let fetched_size = items.len();
//////                    elements.extend(items.into_iter());
////
//////                    match max_results {
//////                        Some(s) => {
//////                            if elements.len() >= s { return Ok(Loop::Break(elements)); }
//////                        }
//////                        _ => ()
//////                    };
//////                    if fetched_size < ssize {
//////                        Ok(Loop::Break(elements))
//////                    } else {
////                        Ok(Loop::Break(Vec::new()))
//////                    }
////                    }))
//};

///
pub trait LibrarySection<'a>: Clone + Sized + PlexClientProvider<'a> {
    type Content: 'a;
    type Error: 'static;
    type Filter: LibraryFilter;

    fn get<F>(&self, filter: F) -> Box<Future<Item = Vec<Self::Content>, Error = Self::Error> + 'a>
    where
        F: 'a + FnMut(&Self::Content) -> bool,
    {
        Box::new(
            self.all()
                .map(|content| content.into_iter().filter(filter).collect::<Vec<_>>()),
        )
    }

    fn all(&self) -> Box<Future<Item = Vec<Self::Content>, Error = Self::Error> + 'a> {
        let url = format!(
            "{}{}/{}/all",
            self.connection().endpoint(),
            PlexLibrary::SECTIONS,
            self.key()
        );
        self.fetch(url.as_str())
    }

    /// Returns a list of media items on deck from this library section.
    fn on_deck(&self) -> Box<Future<Item = Vec<Self::Content>, Error = Self::Error> + 'a> {
        let url = format!(
            "{}{}/{}/onDeck",
            self.connection().endpoint(),
            PlexLibrary::SECTIONS,
            self.key()
        );
        self.fetch(url.as_str())
    }

    // -> Box<Future<Item=Vec<Self::Content>, Error=Self::Error>>
    fn search(&'a self, filter: Vec<Self::Filter>, max_results: Option<usize>) {
        let container_size = min(
            max_results.unwrap_or(X_PLEX_CONTAINER_SIZE),
            X_PLEX_CONTAINER_SIZE,
        );
        let query = match filter.is_empty() {
            false => format!(
                "?{}",
                filter
                    .iter()
                    .map(|f| f.format())
                    .filter(|p| !p.is_empty())
                    .collect::<Vec<_>>()
                    .join("&")
            ),
            _ => "".to_string(),
        };
        let url = format!(
            "{}{}/{}/all{}",
            self.connection().endpoint(),
            PlexLibrary::SECTIONS,
            self.key(),
            query
        );

        println!("{}", url);
        //
        // let x: Box<Future<Item=Vec<Self::Content>, Error=Self::Error>> =
        //     Box::new(loop_fn((self.clone(), Vec::new()),
        //                     |(s, elements)| {
        //                         if elements.is_empty() {
        //                             return Ok(Loop::Break(elements));
        //                         } else {
        //                             s.clone()
        // .fetch_container("", elements.len(), 200usize).and_then(|items| {
        //                                 Ok(())
        //                             });
        //                         }
        //                         Ok(Loop::Continue((s, elements)))
        //                     }));
        // x
    }

    /// need to be implemented in order to support custom deserialization
    fn fetch(&self, url: &str) -> Box<Future<Item = Vec<Self::Content>, Error = Self::Error> + 'a>;
    fn fetch_container(
        &self,
        url: &str,
        start: usize,
        max: usize,
    ) -> Box<Future<Item = Vec<Self::Content>, Error = Self::Error> + 'a>;

    fn section_type() -> SectionType;
    fn connection(&self) -> &Connection;
    fn key(&self) -> String;
}

//fn looper<'a, T: LibrarySection + 'static>((lib, elements):(T, Vec<T::Content>))
//-> Box<Future<Item=Loop<Vec<T::Content>, (Box<T>, Vec<T::Content>)>, Error=T::Error>> {
//    Box::new(lib.fetch_container("", elements.len(), 200usize).and_then(|items| {
//        Ok(Loop::Break(elements))
//    }))
//}

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
            SectionType::Music => "artist",
            SectionType::Show => "show",
        }
    }

    pub fn from_str(type_name: &str) -> Option<SectionType> {
        let s = type_name.to_lowercase();
        match s.as_str() {
            "movie" => Some(SectionType::Movie),
            "photo" => Some(SectionType::Photo),
            "artist" => Some(SectionType::Music),
            "show" => Some(SectionType::Show),
            _ => None,
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
    pub title: String,
    pub agent: String,
    pub scanner: String,
    pub language: String,
    pub uuid: String,
    pub updated_at: String,
    pub created_at: String,
    #[serde(rename = "Location")]
    pub location: Location,
}

impl Section {
    pub const PATH: &'static str = "/library/sections/";

    pub fn path(&self) -> String {
        format!("{}{}/", Section::PATH, self.key)
    }

    pub fn section_type(&self) -> Option<SectionType> {
        SectionType::from_str(self.type_.as_str())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Location {
    id: String,
    path: String,
}

macro_rules! plex_lib_section {
    ($s:tt) => {
        impl<'a> From<PlexLibSection<'a>> for $s<'a> {
            fn from(inner: PlexLibSection<'a>) -> Self { $s { inner } }
        }
        impl<'a> PlexTokenProvider for $s<'a> {
            fn token(&self) -> PlexToken { self.inner.client.token()}
        }

        impl<'a> PlexClientProvider<'a> for $s<'a> {
            fn client(&self) -> &Rc<PlexClient<'a>> { self.inner.client() }
        }
    };
}

#[derive(Debug, Clone)]
pub struct MovieSection<'a> {
    inner: PlexLibSection<'a>,
}
plex_lib_section!(MovieSection);

impl<'a> LibrarySection<'a> for MovieSection<'a> {
    type Content = Video;
    type Error = APIError;
    type Filter = MovieLibraryFilter;

    fn fetch(&self, url: &str) -> Box<Future<Item = Vec<Self::Content>, Error = Self::Error> + 'a> {
        Box::new(
            self.client()
                .get_xml::<VideoContainer>(url)
                .map(move |container| container.videos),
        )
    }

    fn fetch_container(
        &self,
        url: &str,
        start: usize,
        max: usize,
    ) -> Box<Future<Item = Vec<Self::Content>, Error = Self::Error> + 'a> {
        Box::new(
            self.client()
                .get_xml_container::<VideoContainer>(url, start, max)
                .map(move |container| container.videos),
        )
    }

    fn connection(&self) -> &Connection {
        &self.inner.conn
    }

    fn key(&self) -> String {
        self.inner.inner.key.clone()
    }

    fn section_type() -> SectionType {
        SectionType::Movie
    }
}

#[derive(Debug, Clone)]
pub struct MusicSection<'a> {
    inner: PlexLibSection<'a>,
}
plex_lib_section!(MusicSection);

impl<'a> MusicSection<'a> {
    pub fn albums(&self) -> impl Future<Item = Vec<PlexAlbum<'a>>, Error = APIError> {
        self.all()
    }
}

impl<'a> LibrarySection<'a> for MusicSection<'a> {
    type Content = PlexAlbum<'a>;
    type Error = APIError;
    type Filter = MusicLibraryFilter;

    fn all(&self) -> Box<Future<Item = Vec<Self::Content>, Error = Self::Error> + 'a> {
        let url = format!(
            "{}{}/{}/albums",
            self.connection().endpoint(),
            PlexLibrary::SECTIONS,
            self.key()
        );
        self.fetch(url.as_str())
    }

    fn fetch(&self, url: &str) -> Box<Future<Item = Vec<Self::Content>, Error = Self::Error> + 'a> {
        let client = Rc::clone(&self.inner.client);
        let conn = self.connection().clone();
        Box::new(client.get_xml::<AlbumContainer>(url).map(move |container| {
            container
                .albums
                .into_iter()
                .map(|album| PlexAlbum::new(album, conn.clone(), Rc::clone(&client)))
                .collect::<Vec<_>>()
        }))
    }

    fn fetch_container(
        &self,
        url: &str,
        start: usize,
        max: usize,
    ) -> Box<Future<Item = Vec<Self::Content>, Error = Self::Error> + 'a> {
        let client = Rc::clone(&self.inner.client);
        let conn = self.connection().clone();
        Box::new(
            client
                .get_xml_container::<AlbumContainer>(url, start, max)
                .map(move |container| {
                    container
                        .albums
                        .into_iter()
                        .map(|album| PlexAlbum::new(album, conn.clone(), Rc::clone(&client)))
                        .collect::<Vec<_>>()
                }),
        )
    }

    fn connection(&self) -> &Connection {
        &self.inner.conn
    }

    fn key(&self) -> String {
        self.inner.inner.key.clone()
    }

    fn section_type() -> SectionType {
        SectionType::Music
    }
}
