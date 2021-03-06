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
                pub struct Post {
                    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
                    pub id: Option<String>,
                        /// The time in milliseconds a post was created
                    #[serde(rename = "create_at", skip_serializing_if = "Option::is_none")]
                    pub create_at: Option<i64>,
                        /// The time in milliseconds a post was last updated
                    #[serde(rename = "update_at", skip_serializing_if = "Option::is_none")]
                    pub update_at: Option<i64>,
                        /// The time in milliseconds a post was deleted
                    #[serde(rename = "delete_at", skip_serializing_if = "Option::is_none")]
                    pub delete_at: Option<i64>,
                    #[serde(rename = "edit_at", skip_serializing_if = "Option::is_none")]
                    pub edit_at: Option<i64>,
                    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
                    pub user_id: Option<String>,
                    #[serde(rename = "channel_id", skip_serializing_if = "Option::is_none")]
                    pub channel_id: Option<String>,
                    #[serde(rename = "root_id", skip_serializing_if = "Option::is_none")]
                    pub root_id: Option<String>,
                    #[serde(rename = "parent_id", skip_serializing_if = "Option::is_none")]
                    pub parent_id: Option<String>,
                    #[serde(rename = "original_id", skip_serializing_if = "Option::is_none")]
                    pub original_id: Option<String>,
                    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
                    pub message: Option<String>,
                    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
                    pub _type: Option<String>,
                    #[serde(rename = "props", skip_serializing_if = "Option::is_none")]
                    pub props: Option<serde_json::Value>,
                    #[serde(rename = "hashtag", skip_serializing_if = "Option::is_none")]
                    pub hashtag: Option<String>,
                        /// This field will only appear on some posts created before Mattermost 3.5 and has since been deprecated.
                    #[serde(rename = "filenames", skip_serializing_if = "Option::is_none")]
                    pub filenames: Option<Vec<String>>,
                    #[serde(rename = "file_ids", skip_serializing_if = "Option::is_none")]
                    pub file_ids: Option<Vec<String>>,
                    #[serde(rename = "pending_post_id", skip_serializing_if = "Option::is_none")]
                    pub pending_post_id: Option<String>,
                    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
                    pub metadata: Option<crate::models::PostMetadata>,
                }

                impl Post {
                pub fn new() -> Post {
                Post {
                    id: None,
                    create_at: None,
                    update_at: None,
                    delete_at: None,
                    edit_at: None,
                    user_id: None,
                    channel_id: None,
                    root_id: None,
                    parent_id: None,
                    original_id: None,
                    message: None,
                    _type: None,
                    props: None,
                    hashtag: None,
                    filenames: None,
                    file_ids: None,
                    pending_post_id: None,
                    metadata: None,
                }
                }
                }


