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
                pub struct InlineObject9 {
                        /// Use `true` to activate, `false` to deactivate
                    #[serde(rename = "activate", deserialize_with = "crate::de::parse_bool")]
                    pub activate: bool,
                        /// The code produced by your MFA client. Required if `activate` is true
                    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
                    pub code: Option<String>,
                }

                impl InlineObject9 {
                pub fn new(activate: bool) -> InlineObject9 {
                InlineObject9 {
                    activate,
                    code: None,
                }
                }
                }


