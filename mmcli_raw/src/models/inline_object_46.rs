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
pub struct InlineObject46 {
    /// The search term to match against the name or display name of archived channels
    #[serde(rename = "term")]
    pub term: String,
}

impl InlineObject46 {
    pub fn new(term: String) -> InlineObject46 {
        InlineObject46 {
            term,
        }
    }
}


