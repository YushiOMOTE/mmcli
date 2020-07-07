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
                pub struct InlineObject16 {
                        /// Email of a user
                    #[serde(rename = "email")]
                    pub email: String,
                }

                impl InlineObject16 {
                pub fn new(email: String) -> InlineObject16 {
                InlineObject16 {
                    email,
                }
                }
                }


