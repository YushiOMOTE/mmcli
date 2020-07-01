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
pub struct InlineObject49 {
    #[serde(rename = "scheme_admin")]
    pub scheme_admin: bool,
    #[serde(rename = "scheme_user")]
    pub scheme_user: bool,
}

impl InlineObject49 {
    pub fn new(scheme_admin: bool, scheme_user: bool) -> InlineObject49 {
        InlineObject49 {
            scheme_admin,
            scheme_user,
        }
    }
}


