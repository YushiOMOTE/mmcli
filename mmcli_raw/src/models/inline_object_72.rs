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
pub struct InlineObject72 {
    /// Team ID to where the command should be created
    #[serde(rename = "team_id")]
    pub team_id: String,
    /// `'P'` for post request, `'G'` for get request
    #[serde(rename = "method")]
    pub method: String,
    /// Activation word to trigger the command
    #[serde(rename = "trigger")]
    pub trigger: String,
    /// The URL that the command will make the request
    #[serde(rename = "url")]
    pub url: String,
}

impl InlineObject72 {
    pub fn new(team_id: String, method: String, trigger: String, url: String) -> InlineObject72 {
        InlineObject72 {
            team_id,
            method,
            trigger,
            url,
        }
    }
}


