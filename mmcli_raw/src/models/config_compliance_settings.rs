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
                pub struct ConfigComplianceSettings {
                    #[serde(rename = "Enable", skip_serializing_if = "Option::is_none", default, deserialize_with = "crate::de::parse_bool_opt")]
                    pub enable: Option<bool>,
                    #[serde(rename = "Directory", skip_serializing_if = "Option::is_none")]
                    pub directory: Option<String>,
                    #[serde(rename = "EnableDaily", skip_serializing_if = "Option::is_none", default, deserialize_with = "crate::de::parse_bool_opt")]
                    pub enable_daily: Option<bool>,
                }

                impl ConfigComplianceSettings {
                pub fn new() -> ConfigComplianceSettings {
                ConfigComplianceSettings {
                    enable: None,
                    directory: None,
                    enable_daily: None,
                }
                }
                }


