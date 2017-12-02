use reqwest::Response;
pub trait PlexResponse {
    type Data;
    type Error;

    // TODO add reqwest::Response as param
    fn data(&self) -> Result<Self::Data, Self::Error>;
}

pub struct DummyPlexResponse {}

impl PlexResponse for DummyPlexResponse {
    type Data = ();
    type Error = ();

    fn data(&self) -> Result<Self::Data, Self::Error> {
        Ok(())
    }
}

impl From<Response> for DummyPlexResponse {
    fn from(_: Response) -> Self {
        unimplemented!()
    }
}