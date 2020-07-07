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
                pub struct InlineObject39 {
                        /// The string to search in the channel name, display name, and purpose.
                    #[serde(rename = "term")]
                    pub term: String,
                        /// A group id to exclude channels that are associated to that group via GroupChannel records.
                    #[serde(rename = "not_associated_to_group", skip_serializing_if = "Option::is_none")]
                    pub not_associated_to_group: Option<String>,
                        /// Exclude default channels from the results by setting this parameter to true.
                    #[serde(rename = "exclude_default_channels", skip_serializing_if = "Option::is_none", default, deserialize_with = "crate::de::parse_bool_opt")]
                    pub exclude_default_channels: Option<bool>,
                        /// The page number to return, if paginated. If this parameter is not present with the `per_page` parameter then the results will be returned un-paged.
                    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
                    pub page: Option<String>,
                        /// The number of entries to return per page, if paginated. If this parameter is not present with the `page` parameter then the results will be returned un-paged.
                    #[serde(rename = "per_page", skip_serializing_if = "Option::is_none")]
                    pub per_page: Option<String>,
                }

                impl InlineObject39 {
                pub fn new(term: String) -> InlineObject39 {
                InlineObject39 {
                    term,
                    not_associated_to_group: None,
                    exclude_default_channels: None,
                    page: None,
                    per_page: None,
                }
                }
                }


