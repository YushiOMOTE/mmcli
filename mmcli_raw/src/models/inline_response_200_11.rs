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
                pub struct InlineResponse20011 {
                    #[serde(rename = "link", skip_serializing_if = "Option::is_none")]
                    pub link: Option<String>,
                }

                impl InlineResponse20011 {
                pub fn new() -> InlineResponse20011 {
                InlineResponse20011 {
                    link: None,
                }
                }
                }


