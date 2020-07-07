/*
 * Mattermost API Reference
 *
 * There is also a work-in-progress [Postman API reference](https://documenter.getpostman.com/view/4508214/RW8FERUn). 
 *
 * The version of the OpenAPI document: 4.0.0
 * Contact: feedback@mattermost.com
 * Generated by: https://openapi-generator.tech
 */

            /// OpenGraphVideos : Video object used in OpenGraph metadata of a webpage



                #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
                pub struct OpenGraphVideos {
                    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
                    pub url: Option<String>,
                    #[serde(rename = "secure_url", skip_serializing_if = "Option::is_none")]
                    pub secure_url: Option<String>,
                    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
                    pub _type: Option<String>,
                    #[serde(rename = "width", skip_serializing_if = "Option::is_none")]
                    pub width: Option<i64>,
                    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
                    pub height: Option<i64>,
                }

                impl OpenGraphVideos {
                    /// Video object used in OpenGraph metadata of a webpage
                pub fn new() -> OpenGraphVideos {
                OpenGraphVideos {
                    url: None,
                    secure_url: None,
                    _type: None,
                    width: None,
                    height: None,
                }
                }
                }


