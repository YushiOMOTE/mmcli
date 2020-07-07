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
                pub struct MarketplacePlugin {
                        /// URL that leads to the homepage of the plugin.
                    #[serde(rename = "homepage_url", skip_serializing_if = "Option::is_none")]
                    pub homepage_url: Option<String>,
                        /// Base64 encoding of a plugin icon SVG.
                    #[serde(rename = "icon_data", skip_serializing_if = "Option::is_none")]
                    pub icon_data: Option<String>,
                        /// URL to download the plugin.
                    #[serde(rename = "download_url", skip_serializing_if = "Option::is_none")]
                    pub download_url: Option<String>,
                        /// URL that leads to the release notes of the plugin.
                    #[serde(rename = "release_notes_url", skip_serializing_if = "Option::is_none")]
                    pub release_notes_url: Option<String>,
                        /// A list of the plugin labels.
                    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
                    pub labels: Option<Vec<String>>,
                        /// Base64 encoded signature of the plugin.
                    #[serde(rename = "signature", skip_serializing_if = "Option::is_none")]
                    pub signature: Option<String>,
                    #[serde(rename = "manifest", skip_serializing_if = "Option::is_none")]
                    pub manifest: Option<crate::models::PluginManifest>,
                        /// Version number of the already installed plugin, if any.
                    #[serde(rename = "installed_version", skip_serializing_if = "Option::is_none")]
                    pub installed_version: Option<String>,
                }

                impl MarketplacePlugin {
                pub fn new() -> MarketplacePlugin {
                MarketplacePlugin {
                    homepage_url: None,
                    icon_data: None,
                    download_url: None,
                    release_notes_url: None,
                    labels: None,
                    signature: None,
                    manifest: None,
                    installed_version: None,
                }
                }
                }


