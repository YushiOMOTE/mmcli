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
pub struct EnvironmentConfigLogSettings {
    #[serde(rename = "EnableConsole", skip_serializing_if = "Option::is_none")]
    pub enable_console: Option<bool>,
    #[serde(rename = "ConsoleLevel", skip_serializing_if = "Option::is_none")]
    pub console_level: Option<bool>,
    #[serde(rename = "EnableFile", skip_serializing_if = "Option::is_none")]
    pub enable_file: Option<bool>,
    #[serde(rename = "FileLevel", skip_serializing_if = "Option::is_none")]
    pub file_level: Option<bool>,
    #[serde(rename = "FileLocation", skip_serializing_if = "Option::is_none")]
    pub file_location: Option<bool>,
    #[serde(rename = "EnableWebhookDebugging", skip_serializing_if = "Option::is_none")]
    pub enable_webhook_debugging: Option<bool>,
    #[serde(rename = "EnableDiagnostics", skip_serializing_if = "Option::is_none")]
    pub enable_diagnostics: Option<bool>,
}

impl EnvironmentConfigLogSettings {
    pub fn new() -> EnvironmentConfigLogSettings {
        EnvironmentConfigLogSettings {
            enable_console: None,
            console_level: None,
            enable_file: None,
            file_level: None,
            file_location: None,
            enable_webhook_debugging: None,
            enable_diagnostics: None,
        }
    }
}


