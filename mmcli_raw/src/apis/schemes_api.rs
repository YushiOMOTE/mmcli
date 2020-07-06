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


/// struct for typed errors of method `schemes_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SchemesGetError {
    Status400(crate::models::AppError),
    Status401(crate::models::AppError),
    Status403(crate::models::AppError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `schemes_post`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SchemesPostError {
    Status400(crate::models::AppError),
    Status401(crate::models::AppError),
    Status403(crate::models::AppError),
    Status501(crate::models::AppError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `schemes_scheme_id_channels_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SchemesSchemeIdChannelsGetError {
    Status400(crate::models::AppError),
    Status401(crate::models::AppError),
    Status403(crate::models::AppError),
    Status404(crate::models::AppError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `schemes_scheme_id_delete`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SchemesSchemeIdDeleteError {
    Status400(crate::models::AppError),
    Status401(crate::models::AppError),
    Status403(crate::models::AppError),
    Status501(crate::models::AppError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `schemes_scheme_id_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SchemesSchemeIdGetError {
    Status401(crate::models::AppError),
    Status404(crate::models::AppError),
    Status501(crate::models::AppError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `schemes_scheme_id_patch_put`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SchemesSchemeIdPatchPutError {
    Status400(crate::models::AppError),
    Status401(crate::models::AppError),
    Status403(crate::models::AppError),
    Status404(crate::models::AppError),
    Status501(crate::models::AppError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `schemes_scheme_id_teams_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SchemesSchemeIdTeamsGetError {
    Status400(crate::models::AppError),
    Status401(crate::models::AppError),
    Status403(crate::models::AppError),
    Status404(crate::models::AppError),
    UnknownValue(serde_json::Value),
}


/// Get a page of schemes. Use the query parameters to modify the behaviour of this endpoint.  ##### Permissions Must have `manage_system` permission.  __Minimum server version__: 5.0 
pub async fn schemes_get(configuration: &configuration::Configuration, scope: Option<&str>, page: Option<i32>, per_page: Option<i32>) -> Result<Vec<crate::models::Scheme>, Error<SchemesGetError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/schemes", configuration.base_path);
    let mut req_builder = client.get(uri_str.as_str());

    if let Some(ref s) = scope {
        req_builder = req_builder.query(&[("scope", &s.to_string())]);
    }
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
        let entity: Option<SchemesGetError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Create a new scheme.  ##### Permissions Must have `manage_system` permission.  __Minimum server version__: 5.0 
pub async fn schemes_post(configuration: &configuration::Configuration, inline_object78: crate::models::InlineObject78) -> Result<crate::models::Scheme, Error<SchemesPostError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/schemes", configuration.base_path);
    let mut req_builder = client.post(uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&inline_object78);

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if status.is_success() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<SchemesPostError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Get a page of channels which use this scheme. The provided Scheme ID should be for a Channel-scoped Scheme. Use the query parameters to modify the behaviour of this endpoint.  ##### Permissions `manage_system` permission is required.  __Minimum server version__: 5.0 
pub async fn schemes_scheme_id_channels_get(configuration: &configuration::Configuration, scheme_id: &str, page: Option<i32>, per_page: Option<i32>) -> Result<Vec<crate::models::Channel>, Error<SchemesSchemeIdChannelsGetError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/schemes/{scheme_id}/channels", configuration.base_path, scheme_id=crate::apis::urlencode(scheme_id));
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
        let entity: Option<SchemesSchemeIdChannelsGetError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Soft deletes a scheme, by marking the scheme as deleted in the database.  ##### Permissions Must have `manage_system` permission.  __Minimum server version__: 5.0 
pub async fn schemes_scheme_id_delete(configuration: &configuration::Configuration, scheme_id: &str) -> Result<crate::models::StatusOk, Error<SchemesSchemeIdDeleteError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/schemes/{scheme_id}", configuration.base_path, scheme_id=crate::apis::urlencode(scheme_id));
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
        let entity: Option<SchemesSchemeIdDeleteError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Get a scheme from the provided scheme id.  ##### Permissions Must have `manage_system` permission.  __Minimum server version__: 5.0 
pub async fn schemes_scheme_id_get(configuration: &configuration::Configuration, scheme_id: &str) -> Result<crate::models::Scheme, Error<SchemesSchemeIdGetError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/schemes/{scheme_id}", configuration.base_path, scheme_id=crate::apis::urlencode(scheme_id));
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
        let entity: Option<SchemesSchemeIdGetError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Partially update a scheme by providing only the fields you want to update. Omitted fields will not be updated. The fields that can be updated are defined in the request body, all other provided fields will be ignored.  ##### Permissions `manage_system` permission is required.  __Minimum server version__: 5.0 
pub async fn schemes_scheme_id_patch_put(configuration: &configuration::Configuration, scheme_id: &str, inline_object79: crate::models::InlineObject79) -> Result<crate::models::Scheme, Error<SchemesSchemeIdPatchPutError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/schemes/{scheme_id}/patch", configuration.base_path, scheme_id=crate::apis::urlencode(scheme_id));
    let mut req_builder = client.put(uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&inline_object79);

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if status.is_success() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<SchemesSchemeIdPatchPutError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Get a page of teams which use this scheme. The provided Scheme ID should be for a Team-scoped Scheme. Use the query parameters to modify the behaviour of this endpoint.  ##### Permissions `manage_system` permission is required.  __Minimum server version__: 5.0 
pub async fn schemes_scheme_id_teams_get(configuration: &configuration::Configuration, scheme_id: &str, page: Option<i32>, per_page: Option<i32>) -> Result<Vec<crate::models::Team>, Error<SchemesSchemeIdTeamsGetError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/schemes/{scheme_id}/teams", configuration.base_path, scheme_id=crate::apis::urlencode(scheme_id));
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
        let entity: Option<SchemesSchemeIdTeamsGetError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

