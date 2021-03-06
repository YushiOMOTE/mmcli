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
                pub struct InlineObject69 {
                        /// The public certificate file
                    #[serde(rename = "certificate")]
                    pub certificate: std::path::PathBuf,
                }

                impl InlineObject69 {
                pub fn new(certificate: std::path::PathBuf) -> InlineObject69 {
                InlineObject69 {
                    certificate,
                }
                }
                }


