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
                pub struct EnvironmentConfigLdapSettings {
                    #[serde(rename = "Enable", skip_serializing_if = "Option::is_none", default, deserialize_with = "crate::de::parse_bool_opt")]
                    pub enable: Option<bool>,
                    #[serde(rename = "LdapServer", skip_serializing_if = "Option::is_none", default, deserialize_with = "crate::de::parse_bool_opt")]
                    pub ldap_server: Option<bool>,
                    #[serde(rename = "LdapPort", skip_serializing_if = "Option::is_none", default, deserialize_with = "crate::de::parse_bool_opt")]
                    pub ldap_port: Option<bool>,
                    #[serde(rename = "ConnectionSecurity", skip_serializing_if = "Option::is_none", default, deserialize_with = "crate::de::parse_bool_opt")]
                    pub connection_security: Option<bool>,
                    #[serde(rename = "BaseDN", skip_serializing_if = "Option::is_none", default, deserialize_with = "crate::de::parse_bool_opt")]
                    pub base_dn: Option<bool>,
                    #[serde(rename = "BindUsername", skip_serializing_if = "Option::is_none", default, deserialize_with = "crate::de::parse_bool_opt")]
                    pub bind_username: Option<bool>,
                    #[serde(rename = "BindPassword", skip_serializing_if = "Option::is_none", default, deserialize_with = "crate::de::parse_bool_opt")]
                    pub bind_password: Option<bool>,
                    #[serde(rename = "UserFilter", skip_serializing_if = "Option::is_none", default, deserialize_with = "crate::de::parse_bool_opt")]
                    pub user_filter: Option<bool>,
                    #[serde(rename = "FirstNameAttribute", skip_serializing_if = "Option::is_none", default, deserialize_with = "crate::de::parse_bool_opt")]
                    pub first_name_attribute: Option<bool>,
                    #[serde(rename = "LastNameAttribute", skip_serializing_if = "Option::is_none", default, deserialize_with = "crate::de::parse_bool_opt")]
                    pub last_name_attribute: Option<bool>,
                    #[serde(rename = "EmailAttribute", skip_serializing_if = "Option::is_none", default, deserialize_with = "crate::de::parse_bool_opt")]
                    pub email_attribute: Option<bool>,
                    #[serde(rename = "UsernameAttribute", skip_serializing_if = "Option::is_none", default, deserialize_with = "crate::de::parse_bool_opt")]
                    pub username_attribute: Option<bool>,
                    #[serde(rename = "NicknameAttribute", skip_serializing_if = "Option::is_none", default, deserialize_with = "crate::de::parse_bool_opt")]
                    pub nickname_attribute: Option<bool>,
                    #[serde(rename = "IdAttribute", skip_serializing_if = "Option::is_none", default, deserialize_with = "crate::de::parse_bool_opt")]
                    pub id_attribute: Option<bool>,
                    #[serde(rename = "PositionAttribute", skip_serializing_if = "Option::is_none", default, deserialize_with = "crate::de::parse_bool_opt")]
                    pub position_attribute: Option<bool>,
                    #[serde(rename = "SyncIntervalMinutes", skip_serializing_if = "Option::is_none", default, deserialize_with = "crate::de::parse_bool_opt")]
                    pub sync_interval_minutes: Option<bool>,
                    #[serde(rename = "SkipCertificateVerification", skip_serializing_if = "Option::is_none", default, deserialize_with = "crate::de::parse_bool_opt")]
                    pub skip_certificate_verification: Option<bool>,
                    #[serde(rename = "QueryTimeout", skip_serializing_if = "Option::is_none", default, deserialize_with = "crate::de::parse_bool_opt")]
                    pub query_timeout: Option<bool>,
                    #[serde(rename = "MaxPageSize", skip_serializing_if = "Option::is_none", default, deserialize_with = "crate::de::parse_bool_opt")]
                    pub max_page_size: Option<bool>,
                    #[serde(rename = "LoginFieldName", skip_serializing_if = "Option::is_none", default, deserialize_with = "crate::de::parse_bool_opt")]
                    pub login_field_name: Option<bool>,
                }

                impl EnvironmentConfigLdapSettings {
                pub fn new() -> EnvironmentConfigLdapSettings {
                EnvironmentConfigLdapSettings {
                    enable: None,
                    ldap_server: None,
                    ldap_port: None,
                    connection_security: None,
                    base_dn: None,
                    bind_username: None,
                    bind_password: None,
                    user_filter: None,
                    first_name_attribute: None,
                    last_name_attribute: None,
                    email_attribute: None,
                    username_attribute: None,
                    nickname_attribute: None,
                    id_attribute: None,
                    position_attribute: None,
                    sync_interval_minutes: None,
                    skip_certificate_verification: None,
                    query_timeout: None,
                    max_page_size: None,
                    login_field_name: None,
                }
                }
                }


