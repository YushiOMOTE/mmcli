/*
 * Mattermost API Reference
 *
 * There is also a work-in-progress [Postman API reference](https://documenter.getpostman.com/view/4508214/RW8FERUn). 
 *
 * The version of the OpenAPI document: 4.0.0
 * Contact: feedback@mattermost.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserAutocomplete {
    /// A list of users that are the main result of the query
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<crate::models::User>>,
    /// A special case list of users returned when autocompleting in a specific channel. Omitted when empty or not relevant
    #[serde(rename = "out_of_channel", skip_serializing_if = "Option::is_none")]
    pub out_of_channel: Option<Vec<crate::models::User>>,
}

impl UserAutocomplete {
    pub fn new() -> UserAutocomplete {
        UserAutocomplete {
            users: None,
            out_of_channel: None,
        }
    }
}


