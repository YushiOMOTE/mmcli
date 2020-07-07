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
                pub struct UserAuthData {
                        /// Service-specific authentication data
                    #[serde(rename = "auth_data", skip_serializing_if = "Option::is_none")]
                    pub auth_data: Option<String>,
                        /// The authentication service such as \"email\", \"gitlab\", or \"ldap\"
                    #[serde(rename = "auth_service", skip_serializing_if = "Option::is_none")]
                    pub auth_service: Option<String>,
                        /// The password used for email authentication
                    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
                    pub password: Option<String>,
                }

                impl UserAuthData {
                pub fn new() -> UserAuthData {
                UserAuthData {
                    auth_data: None,
                    auth_service: None,
                    password: None,
                }
                }
                }


