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
pub struct InlineObject40 {
    /// The search term to match against the members' usernames of the group channels
    #[serde(rename = "term")]
    pub term: String,
}

impl InlineObject40 {
    pub fn new(term: String) -> InlineObject40 {
        InlineObject40 {
            term,
        }
    }
}


