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
                pub struct ConfigClusterSettings {
                    #[serde(rename = "Enable", skip_serializing_if = "Option::is_none", default, deserialize_with = "crate::de::parse_bool_opt")]
                    pub enable: Option<bool>,
                    #[serde(rename = "InterNodeListenAddress", skip_serializing_if = "Option::is_none")]
                    pub inter_node_listen_address: Option<String>,
                    #[serde(rename = "InterNodeUrls", skip_serializing_if = "Option::is_none")]
                    pub inter_node_urls: Option<Vec<String>>,
                }

                impl ConfigClusterSettings {
                pub fn new() -> ConfigClusterSettings {
                ConfigClusterSettings {
                    enable: None,
                    inter_node_listen_address: None,
                    inter_node_urls: None,
                }
                }
                }


