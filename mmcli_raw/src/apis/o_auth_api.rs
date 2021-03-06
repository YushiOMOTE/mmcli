/*
 * Mattermost API Reference
 *
 * There is also a work-in-progress [Postman API reference](https://documenter.getpostman.com/view/4508214/RW8FERUn). 
 *
 * The version of the OpenAPI document: 4.0.0
 * Contact: feedback@mattermost.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method `oauth_apps_app_id_delete`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OauthAppsAppIdDeleteError {
    Status400(crate::models::AppError),
    Status401(crate::models::AppError),
    Status403(crate::models::AppError),
    Status404(crate::models::AppError),
    Status501(crate::models::AppError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `oauth_apps_app_id_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OauthAppsAppIdGetError {
    Status400(crate::models::AppError),
    Status401(crate::models::AppError),
    Status403(crate::models::AppError),
    Status404(crate::models::AppError),
    Status501(crate::models::AppError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `oauth_apps_app_id_info_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OauthAppsAppIdInfoGetError {
    Status400(crate::models::AppError),
    Status401(crate::models::AppError),
    Status404(crate::models::AppError),
    Status501(crate::models::AppError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `oauth_apps_app_id_put`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OauthAppsAppIdPutError {
    Status400(crate::models::AppError),
    Status401(crate::models::AppError),
    Status403(crate::models::AppError),
    Status404(crate::models::AppError),
    Status501(crate::models::AppError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `oauth_apps_app_id_regen_secret_post`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OauthAppsAppIdRegenSecretPostError {
    Status400(crate::models::AppError),
    Status401(crate::models::AppError),
    Status403(crate::models::AppError),
    Status404(crate::models::AppError),
    Status501(crate::models::AppError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `oauth_apps_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OauthAppsGetError {
    Status400(crate::models::AppError),
    Status401(crate::models::AppError),
    Status403(crate::models::AppError),
    Status501(crate::models::AppError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `oauth_apps_post`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OauthAppsPostError {
    Status400(crate::models::AppError),
    Status401(crate::models::AppError),
    Status403(crate::models::AppError),
    Status501(crate::models::AppError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `users_user_id_oauth_apps_authorized_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UsersUserIdOauthAppsAuthorizedGetError {
    Status400(crate::models::AppError),
    Status401(crate::models::AppError),
    Status403(crate::models::AppError),
    Status501(crate::models::AppError),
    UnknownValue(serde_json::Value),
}


/// Delete and unregister an OAuth 2.0 client application  ##### Permissions If app creator, must have `mange_oauth` permission otherwise `manage_system_wide_oauth` permission is required. 
pub async fn oauth_apps_app_id_delete(configuration: &configuration::Configuration, app_id: &str) -> Result<crate::models::StatusOk, Error<OauthAppsAppIdDeleteError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/oauth/apps/{app_id}", configuration.base_path, app_id=crate::apis::urlencode(app_id));
    let mut req_builder = client.delete(uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if status.is_success() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<OauthAppsAppIdDeleteError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Get an OAuth 2.0 client application registered with Mattermost. ##### Permissions If app creator, must have `mange_oauth` permission otherwise `manage_system_wide_oauth` permission is required. 
pub async fn oauth_apps_app_id_get(configuration: &configuration::Configuration, app_id: &str) -> Result<crate::models::OAuthApp, Error<OauthAppsAppIdGetError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/oauth/apps/{app_id}", configuration.base_path, app_id=crate::apis::urlencode(app_id));
    let mut req_builder = client.get(uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if status.is_success() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<OauthAppsAppIdGetError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Get public information about an OAuth 2.0 client application registered with Mattermost. The application's client secret will be blanked out. ##### Permissions Must be authenticated. 
pub async fn oauth_apps_app_id_info_get(configuration: &configuration::Configuration, app_id: &str) -> Result<crate::models::OAuthApp, Error<OauthAppsAppIdInfoGetError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/oauth/apps/{app_id}/info", configuration.base_path, app_id=crate::apis::urlencode(app_id));
    let mut req_builder = client.get(uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if status.is_success() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<OauthAppsAppIdInfoGetError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Update an OAuth 2.0 client application based on OAuth struct. ##### Permissions If app creator, must have `mange_oauth` permission otherwise `manage_system_wide_oauth` permission is required. 
pub async fn oauth_apps_app_id_put(configuration: &configuration::Configuration, app_id: &str, inline_object76: crate::models::InlineObject76) -> Result<crate::models::OAuthApp, Error<OauthAppsAppIdPutError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/oauth/apps/{app_id}", configuration.base_path, app_id=crate::apis::urlencode(app_id));
    let mut req_builder = client.put(uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&inline_object76);

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if status.is_success() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<OauthAppsAppIdPutError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Regenerate the client secret for an OAuth 2.0 client application registered with Mattermost. ##### Permissions If app creator, must have `mange_oauth` permission otherwise `manage_system_wide_oauth` permission is required. 
pub async fn oauth_apps_app_id_regen_secret_post(configuration: &configuration::Configuration, app_id: &str) -> Result<crate::models::OAuthApp, Error<OauthAppsAppIdRegenSecretPostError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/oauth/apps/{app_id}/regen_secret", configuration.base_path, app_id=crate::apis::urlencode(app_id));
    let mut req_builder = client.post(uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if status.is_success() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<OauthAppsAppIdRegenSecretPostError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Get a page of OAuth 2.0 client applications registered with Mattermost. ##### Permissions With `manage_oauth` permission, the apps registered by the logged in user are returned. With `manage_system_wide_oauth` permission, all apps regardless of creator are returned. 
pub async fn oauth_apps_get(configuration: &configuration::Configuration, page: Option<i64>, per_page: Option<i64>) -> Result<Vec<crate::models::OAuthApp>, Error<OauthAppsGetError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/oauth/apps", configuration.base_path);
    let mut req_builder = client.get(uri_str.as_str());

    if let Some(ref s) = page {
        req_builder = req_builder.query(&[("page", &s.to_string())]);
    }
    if let Some(ref s) = per_page {
        req_builder = req_builder.query(&[("per_page", &s.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if status.is_success() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<OauthAppsGetError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Register an OAuth 2.0 client application with Mattermost as the service provider. ##### Permissions Must have `manage_oauth` permission. 
pub async fn oauth_apps_post(configuration: &configuration::Configuration, inline_object75: crate::models::InlineObject75) -> Result<crate::models::OAuthApp, Error<OauthAppsPostError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/oauth/apps", configuration.base_path);
    let mut req_builder = client.post(uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&inline_object75);

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if status.is_success() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<OauthAppsPostError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Get a page of OAuth 2.0 client applications authorized to access a user's account. ##### Permissions Must be authenticated as the user or have `edit_other_users` permission. 
pub async fn users_user_id_oauth_apps_authorized_get(configuration: &configuration::Configuration, user_id: &str, page: Option<i64>, per_page: Option<i64>) -> Result<Vec<crate::models::OAuthApp>, Error<UsersUserIdOauthAppsAuthorizedGetError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/users/{user_id}/oauth/apps/authorized", configuration.base_path, user_id=crate::apis::urlencode(user_id));
    let mut req_builder = client.get(uri_str.as_str());

    if let Some(ref s) = page {
        req_builder = req_builder.query(&[("page", &s.to_string())]);
    }
    if let Some(ref s) = per_page {
        req_builder = req_builder.query(&[("per_page", &s.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if status.is_success() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<UsersUserIdOauthAppsAuthorizedGetError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

