use reqwest::Response;
use plex::types::{User, MediaContainer,PlexDevice};
use serde_xml_rs::deserialize;
use serde_xml_rs::Error;

pub trait PlexResponse {
    type Data;
    type Error;

    fn from_response(response: Response) -> Result<Self::Data, Self::Error>;
}

pub struct SignInResponse {}


impl PlexResponse for SignInResponse {
    type Data = User;
    type Error = ();

    fn from_response(response: Response) -> Result<Self::Data, Self::Error> {
        match deserialize(response) {
            Ok(data) => Ok(data),
            _ => Err(())
        }
    }
}

pub struct DeviceResponse {}

impl PlexResponse for DeviceResponse {
    type Data = Vec<PlexDevice>;
    type Error = ();

    fn from_response(response: Response) -> Result<Self::Data, Self::Error> {
        let res: Result<MediaContainer, Error> = deserialize(response);
        match res {
            Ok(data) => Ok(data.devices),
            _ => {
                println!("desirialize error");
                Err(())
            }
        }
    }
}