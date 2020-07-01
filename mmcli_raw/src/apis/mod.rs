use reqwest;
use serde_json;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl <T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl <T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl <T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub mod bleve_api;
pub mod bots_api;
pub mod brand_api;
pub mod channels_api;
pub mod cluster_api;
pub mod commands_api;
pub mod compliance_api;
pub mod dataretention_api;
pub mod elasticsearch_api;
pub mod emoji_api;
pub mod integration_actions_api;
pub mod jobs_api;
pub mod o_auth_api;
pub mod open_graph_api;
pub mod posts_api;
pub mod preferences_api;
pub mod reactions_api;
pub mod roles_api;
pub mod root_api;
pub mod saml_api;
pub mod schemes_api;
pub mod status_api;
pub mod system_api;
pub mod teams_api;
pub mod terms_of_service_api;
pub mod users_api;
pub mod webhooks_api;

pub mod configuration;
