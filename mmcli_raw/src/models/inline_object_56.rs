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
                pub struct InlineObject56 {
                        /// The search terms as inputed by the user. To search for posts from a user include `from:someusername`, using a user's username. To search in a specific channel include `in:somechannel`, using the channel name (not the display name).
                    #[serde(rename = "terms")]
                    pub terms: String,
                        /// Set to true if an Or search should be performed vs an And search.
                    #[serde(rename = "is_or_search", deserialize_with = "crate::de::parse_bool")]
                    pub is_or_search: bool,
                        /// Offset from UTC of user timezone for date searches.
                    #[serde(rename = "time_zone_offset", skip_serializing_if = "Option::is_none")]
                    pub time_zone_offset: Option<i64>,
                        /// Set to true if deleted channels should be included in the search. (archived channels)
                    #[serde(rename = "include_deleted_channels", skip_serializing_if = "Option::is_none", default, deserialize_with = "crate::de::parse_bool_opt")]
                    pub include_deleted_channels: Option<bool>,
                        /// The page to select. (Only works with Elasticsearch)
                    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
                    pub page: Option<i64>,
                        /// The number of posts per page. (Only works with Elasticsearch)
                    #[serde(rename = "per_page", skip_serializing_if = "Option::is_none")]
                    pub per_page: Option<i64>,
                }

                impl InlineObject56 {
                pub fn new(terms: String, is_or_search: bool) -> InlineObject56 {
                InlineObject56 {
                    terms,
                    is_or_search,
                    time_zone_offset: None,
                    include_deleted_channels: None,
                    page: None,
                    per_page: None,
                }
                }
                }


