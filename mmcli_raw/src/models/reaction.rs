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
                pub struct Reaction {
                        /// The ID of the user that made this reaction
                    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
                    pub user_id: Option<String>,
                        /// The ID of the post to which this reaction was made
                    #[serde(rename = "post_id", skip_serializing_if = "Option::is_none")]
                    pub post_id: Option<String>,
                        /// The name of the emoji that was used for this reaction
                    #[serde(rename = "emoji_name", skip_serializing_if = "Option::is_none")]
                    pub emoji_name: Option<String>,
                        /// The time in milliseconds this reaction was made
                    #[serde(rename = "create_at", skip_serializing_if = "Option::is_none")]
                    pub create_at: Option<i64>,
                }

                impl Reaction {
                pub fn new() -> Reaction {
                Reaction {
                    user_id: None,
                    post_id: None,
                    emoji_name: None,
                    create_at: None,
                }
                }
                }


