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
                pub struct InlineObject34 {
                    #[serde(rename = "scheme_admin", deserialize_with = "crate::de::parse_bool")]
                    pub scheme_admin: bool,
                    #[serde(rename = "scheme_user", deserialize_with = "crate::de::parse_bool")]
                    pub scheme_user: bool,
                }

                impl InlineObject34 {
                pub fn new(scheme_admin: bool, scheme_user: bool) -> InlineObject34 {
                InlineObject34 {
                    scheme_admin,
                    scheme_user,
                }
                }
                }


