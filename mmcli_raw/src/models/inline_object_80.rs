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
                pub struct InlineObject80 {
                        /// The URL to get Open Graph Metadata.
                    #[serde(rename = "url")]
                    pub url: String,
                }

                impl InlineObject80 {
                pub fn new(url: String) -> InlineObject80 {
                InlineObject80 {
                    url,
                }
                }
                }


