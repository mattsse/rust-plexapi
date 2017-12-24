use std::borrow::Cow;

fn encode(v: &Vec<String>) -> Cow<[u8]> { Cow::Owned(v.join(",").into_bytes()) }

pub trait LibraryFilter {
    fn format(&self) -> String;
}

/// All allowed filters for a movie section
#[derive(Debug, Clone, PartialEq)]
pub enum MovieLibraryFilter {
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

impl LibraryFilter for MovieLibraryFilter {
    fn format(&self) -> String {
        match *self {
            MovieLibraryFilter::Unwachted(v) => format!("unwatched={}", v),
            MovieLibraryFilter::Duplicate(v) => format!("duplicate={}", v),
            MovieLibraryFilter::Year(ref v) => format!("year={}", v.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(",")),
            MovieLibraryFilter::Decade(ref v) => format!("decade={}", v.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(",")),
            MovieLibraryFilter::Genre(ref v) => format!("genre={:?}", encode(v)),
            MovieLibraryFilter::Collection(ref v) => format!("collection={:?}", encode(v)),
            MovieLibraryFilter::Director(ref v) => format!("director={:?}", encode(v)),
            MovieLibraryFilter::Actor(ref v) => format!("actor={:?}", encode(v)),
            MovieLibraryFilter::Country(ref v) => format!("country={}", v),
            MovieLibraryFilter::Studio(ref v) => format!("studio={:?}", encode(v)),
            MovieLibraryFilter::Resolution(ref v) => format!("resolution={}", v),
            MovieLibraryFilter::Guid(ref v) => format!("guid={}", v),
            MovieLibraryFilter::Label(ref v) => format!("label={}", v),
            MovieLibraryFilter::ContentRating(v) => format!("contentRating={}", v)
        }
    }
}

/// All allowed filters for a movie section
#[derive(Debug, Clone, PartialEq)]
pub enum MusicLibraryFilter {
    Genre(Vec<String>),
    Country(String),
    Collection(Vec<String>),
    Mood(String),
}

/// trait to supply the search query with adequate filters
impl LibraryFilter for MusicLibraryFilter {
    fn format(&self) -> String {
        match *self {
            MusicLibraryFilter::Genre(ref v) => format!("genre={:?}", encode(v)),
            MusicLibraryFilter::Country(ref v) => format!("country={}", v),
            MusicLibraryFilter::Collection(ref v) => format!("collection={:?}", encode(v)),
            MusicLibraryFilter::Mood(ref v) => format!("mood={}", v)
        }
    }
}