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
pub struct ChannelNotifyProps {
    /// Set to \"true\" to enable email notifications, \"false\" to disable, or \"default\" to use the global user notification setting.
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<bool>,
    /// Set to \"all\" to receive push notifications for all activity, \"mention\" for mentions and direct messages only, \"none\" to disable, or \"default\" to use the global user notification setting.
    #[serde(rename = "push", skip_serializing_if = "Option::is_none")]
    pub push: Option<String>,
    /// Set to \"all\" to receive desktop notifications for all activity, \"mention\" for mentions and direct messages only, \"none\" to disable, or \"default\" to use the global user notification setting.
    #[serde(rename = "desktop", skip_serializing_if = "Option::is_none")]
    pub desktop: Option<String>,
    /// Set to \"all\" to mark the channel unread for any new message, \"mention\" to mark unread for new mentions only. Defaults to \"all\".
    #[serde(rename = "mark_unread", skip_serializing_if = "Option::is_none")]
    pub mark_unread: Option<String>,
}

impl ChannelNotifyProps {
    pub fn new() -> ChannelNotifyProps {
        ChannelNotifyProps {
            email: None,
            push: None,
            desktop: None,
            mark_unread: None,
        }
    }
}


