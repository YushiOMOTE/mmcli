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
pub struct InlineObject28 {
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "company_name", skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    #[serde(rename = "invite_id", skip_serializing_if = "Option::is_none")]
    pub invite_id: Option<String>,
    #[serde(rename = "allow_open_invite", skip_serializing_if = "Option::is_none")]
    pub allow_open_invite: Option<bool>,
}

impl InlineObject28 {
    pub fn new() -> InlineObject28 {
        InlineObject28 {
            display_name: None,
            description: None,
            company_name: None,
            invite_id: None,
            allow_open_invite: None,
        }
    }
}


