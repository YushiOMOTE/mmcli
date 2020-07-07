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
                pub struct InlineObject35 {
                        /// List of emails
                    #[serde(rename = "emails")]
                    pub emails: Vec<String>,
                        /// List of channel ids
                    #[serde(rename = "channels")]
                    pub channels: Vec<String>,
                        /// Message to include in the invite
                    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
                    pub message: Option<String>,
                }

                impl InlineObject35 {
                pub fn new(emails: Vec<String>, channels: Vec<String>) -> InlineObject35 {
                InlineObject35 {
                    emails,
                    channels,
                    message: None,
                }
                }
                }


