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
                pub struct InlineObject71 {
                        /// The image to be uploaded
                    #[serde(rename = "image")]
                    pub image: std::path::PathBuf,
                }

                impl InlineObject71 {
                pub fn new(image: std::path::PathBuf) -> InlineObject71 {
                InlineObject71 {
                    image,
                }
                }
                }


