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
                pub struct InlineObject47 {
                        /// The ID of user to add into the channel
                    #[serde(rename = "user_id")]
                    pub user_id: String,
                        /// The ID of root post where link to add channel member originates
                    #[serde(rename = "post_root_id", skip_serializing_if = "Option::is_none")]
                    pub post_root_id: Option<String>,
                }

                impl InlineObject47 {
                pub fn new(user_id: String) -> InlineObject47 {
                InlineObject47 {
                    user_id,
                    post_root_id: None,
                }
                }
                }


