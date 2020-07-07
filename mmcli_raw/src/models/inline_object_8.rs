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
                pub struct InlineObject8 {
                        /// The recovery code
                    #[serde(rename = "code")]
                    pub code: String,
                        /// The new password for the user
                    #[serde(rename = "new_password")]
                    pub new_password: String,
                }

                impl InlineObject8 {
                pub fn new(code: String, new_password: String) -> InlineObject8 {
                InlineObject8 {
                    code,
                    new_password,
                }
                }
                }


