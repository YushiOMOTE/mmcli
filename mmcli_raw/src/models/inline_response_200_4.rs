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
                pub struct InlineResponse2004 {
                        /// The teams that matched the query.
                    #[serde(rename = "teams", skip_serializing_if = "Option::is_none")]
                    pub teams: Option<Vec<crate::models::Team>>,
                        /// The total number of results, regardless of page and per_page requested.
                    #[serde(rename = "total_count", skip_serializing_if = "Option::is_none")]
                    pub total_count: Option<f32>,
                }

                impl InlineResponse2004 {
                pub fn new() -> InlineResponse2004 {
                InlineResponse2004 {
                    teams: None,
                    total_count: None,
                }
                }
                }


