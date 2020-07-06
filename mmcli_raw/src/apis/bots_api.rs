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


/// struct for typed errors of method `bots_bot_user_id_assign_user_id_post`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BotsBotUserIdAssignUserIdPostError {
    Status400(crate::models::AppError),
    Status401(crate::models::AppError),
    Status403(crate::models::AppError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `bots_bot_user_id_convert_to_user_post`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BotsBotUserIdConvertToUserPostError {
    Status400(crate::models::AppError),
    Status401(crate::models::AppError),
    Status403(crate::models::AppError),
    Status404(crate::models::AppError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `bots_bot_user_id_disable_post`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BotsBotUserIdDisablePostError {
    Status400(crate::models::AppError),
    Status401(crate::models::AppError),
    Status403(crate::models::AppError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `bots_bot_user_id_enable_post`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BotsBotUserIdEnablePostError {
    Status400(crate::models::AppError),
    Status401(crate::models::AppError),
    Status403(crate::models::AppError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `bots_bot_user_id_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BotsBotUserIdGetError {
    Status400(crate::models::AppError),
    Status401(crate::models::AppError),
    Status403(crate::models::AppError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `bots_bot_user_id_icon_delete`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BotsBotUserIdIconDeleteError {
    Status400(crate::models::AppError),
    Status401(crate::models::AppError),
    Status403(crate::models::AppError),
    Status404(crate::models::AppError),
    Status500(crate::models::AppError),
    Status501(crate::models::AppError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `bots_bot_user_id_icon_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BotsBotUserIdIconGetError {
    Status400(crate::models::AppError),
    Status401(crate::models::AppError),
    Status403(crate::models::AppError),
    Status404(crate::models::AppError),
    Status500(crate::models::AppError),
    Status501(crate::models::AppError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `bots_bot_user_id_icon_post`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BotsBotUserIdIconPostError {
    Status400(crate::models::AppError),
    Status401(crate::models::AppError),
    Status403(crate::models::AppError),
    Status413(crate::models::AppError),
    Status500(crate::models::AppError),
    Status501(crate::models::AppError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `bots_bot_user_id_put`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BotsBotUserIdPutError {
    Status400(crate::models::AppError),
    Status401(crate::models::AppError),
    Status403(crate::models::AppError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `bots_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BotsGetError {
    Status400(crate::models::AppError),
    Status401(crate::models::AppError),
    Status403(crate::models::AppError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `bots_post`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BotsPostError {
    Status400(crate::models::AppError),
    Status401(crate::models::AppError),
    Status403(crate::models::AppError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `users_user_id_convert_to_bot_post`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UsersUserIdConvertToBotPostError {
    Status400(crate::models::AppError),
    Status401(crate::models::AppError),
    Status403(crate::models::AppError),
    Status404(crate::models::AppError),
    UnknownValue(serde_json::Value),
}


/// Assign a bot to a specified user. ##### Permissions Must have `manage_bots` permission.  __Minimum server version__: 5.10 
pub async fn bots_bot_user_id_assign_user_id_post(configuration: &configuration::Configuration, bot_user_id: &str, user_id: &str) -> Result<crate::models::Bot, Error<BotsBotUserIdAssignUserIdPostError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/bots/{bot_user_id}/assign/{user_id}", configuration.base_path, bot_user_id=crate::apis::urlencode(bot_user_id), user_id=crate::apis::urlencode(user_id));
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
        let entity: Option<BotsBotUserIdAssignUserIdPostError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Convert a bot into a user.  __Minimum server version__: 5.26  ##### Permissions Must have `manage_system` permission. 
pub async fn bots_bot_user_id_convert_to_user_post(configuration: &configuration::Configuration, bot_user_id: &str, inline_object86: crate::models::InlineObject86, set_system_admin: Option<bool>) -> Result<crate::models::StatusOk, Error<BotsBotUserIdConvertToUserPostError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/bots/{bot_user_id}/convert_to_user", configuration.base_path, bot_user_id=crate::apis::urlencode(bot_user_id));
    let mut req_builder = client.post(uri_str.as_str());

    if let Some(ref s) = set_system_admin {
        req_builder = req_builder.query(&[("set_system_admin", &s.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&inline_object86);

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if status.is_success() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<BotsBotUserIdConvertToUserPostError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Disable a bot. ##### Permissions Must have `manage_bots` permission.  __Minimum server version__: 5.10 
pub async fn bots_bot_user_id_disable_post(configuration: &configuration::Configuration, bot_user_id: &str) -> Result<crate::models::Bot, Error<BotsBotUserIdDisablePostError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/bots/{bot_user_id}/disable", configuration.base_path, bot_user_id=crate::apis::urlencode(bot_user_id));
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
        let entity: Option<BotsBotUserIdDisablePostError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Enable a bot. ##### Permissions Must have `manage_bots` permission.  __Minimum server version__: 5.10 
pub async fn bots_bot_user_id_enable_post(configuration: &configuration::Configuration, bot_user_id: &str) -> Result<crate::models::Bot, Error<BotsBotUserIdEnablePostError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/bots/{bot_user_id}/enable", configuration.base_path, bot_user_id=crate::apis::urlencode(bot_user_id));
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
        let entity: Option<BotsBotUserIdEnablePostError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Get a bot specified by its bot id. ##### Permissions Must have `read_bots` permission for bots you are managing, and `read_others_bots` permission for bots others are managing. __Minimum server version__: 5.10 
pub async fn bots_bot_user_id_get(configuration: &configuration::Configuration, bot_user_id: &str, include_deleted: Option<bool>) -> Result<crate::models::Bot, Error<BotsBotUserIdGetError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/bots/{bot_user_id}", configuration.base_path, bot_user_id=crate::apis::urlencode(bot_user_id));
    let mut req_builder = client.get(uri_str.as_str());

    if let Some(ref s) = include_deleted {
        req_builder = req_builder.query(&[("include_deleted", &s.to_string())]);
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
        let entity: Option<BotsBotUserIdGetError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Delete bot's LHS icon image based on bot_user_id string parameter. ##### Permissions Must have `manage_bots` permission. __Minimum server version__: 5.14 
pub async fn bots_bot_user_id_icon_delete(configuration: &configuration::Configuration, bot_user_id: &str) -> Result<crate::models::StatusOk, Error<BotsBotUserIdIconDeleteError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/bots/{bot_user_id}/icon", configuration.base_path, bot_user_id=crate::apis::urlencode(bot_user_id));
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
        let entity: Option<BotsBotUserIdIconDeleteError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Get a bot's LHS icon image based on bot_user_id string parameter. ##### Permissions Must be logged in. __Minimum server version__: 5.14 
pub async fn bots_bot_user_id_icon_get(configuration: &configuration::Configuration, bot_user_id: &str) -> Result<(), Error<BotsBotUserIdIconGetError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/bots/{bot_user_id}/icon", configuration.base_path, bot_user_id=crate::apis::urlencode(bot_user_id));
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
        Ok(())
    } else {
        let entity: Option<BotsBotUserIdIconGetError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Set a bot's LHS icon image based on bot_user_id string parameter. Icon image must be SVG format, all other formats are rejected. ##### Permissions Must have `manage_bots` permission. __Minimum server version__: 5.14 
pub async fn bots_bot_user_id_icon_post(configuration: &configuration::Configuration, bot_user_id: &str, image: std::path::PathBuf) -> Result<crate::models::StatusOk, Error<BotsBotUserIdIconPostError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/bots/{bot_user_id}/icon", configuration.base_path, bot_user_id=crate::apis::urlencode(bot_user_id));
    let mut req_builder = client.post(uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    let mut form = reqwest::multipart::Form::new();
    // TODO: support file upload for 'image' parameter
    req_builder = req_builder.multipart(form);

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if status.is_success() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<BotsBotUserIdIconPostError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Partially update a bot by providing only the fields you want to update. Omitted fields will not be updated. The fields that can be updated are defined in the request body, all other provided fields will be ignored. ##### Permissions Must have `manage_bots` permission.  __Minimum server version__: 5.10 
pub async fn bots_bot_user_id_put(configuration: &configuration::Configuration, bot_user_id: &str, inline_object84: crate::models::InlineObject84) -> Result<crate::models::Bot, Error<BotsBotUserIdPutError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/bots/{bot_user_id}", configuration.base_path, bot_user_id=crate::apis::urlencode(bot_user_id));
    let mut req_builder = client.put(uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&inline_object84);

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if status.is_success() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<BotsBotUserIdPutError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Get a page of a list of bots. ##### Permissions Must have `read_bots` permission for bots you are managing, and `read_others_bots` permission for bots others are managing. __Minimum server version__: 5.10 
pub async fn bots_get(configuration: &configuration::Configuration, page: Option<i32>, per_page: Option<i32>, include_deleted: Option<bool>, only_orphaned: Option<bool>) -> Result<Vec<crate::models::Bot>, Error<BotsGetError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/bots", configuration.base_path);
    let mut req_builder = client.get(uri_str.as_str());

    if let Some(ref s) = page {
        req_builder = req_builder.query(&[("page", &s.to_string())]);
    }
    if let Some(ref s) = per_page {
        req_builder = req_builder.query(&[("per_page", &s.to_string())]);
    }
    if let Some(ref s) = include_deleted {
        req_builder = req_builder.query(&[("include_deleted", &s.to_string())]);
    }
    if let Some(ref s) = only_orphaned {
        req_builder = req_builder.query(&[("only_orphaned", &s.to_string())]);
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
        let entity: Option<BotsGetError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Create a new bot account on the system. Username is required. ##### Permissions Must have `create_bot` permission. __Minimum server version__: 5.10 
pub async fn bots_post(configuration: &configuration::Configuration, inline_object83: crate::models::InlineObject83) -> Result<crate::models::Bot, Error<BotsPostError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/bots", configuration.base_path);
    let mut req_builder = client.post(uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&inline_object83);

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if status.is_success() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<BotsPostError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Convert a user into a bot.  __Minimum server version__: 5.26  ##### Permissions Must have `manage_system` permission. 
pub async fn users_user_id_convert_to_bot_post(configuration: &configuration::Configuration, user_id: &str) -> Result<crate::models::StatusOk, Error<UsersUserIdConvertToBotPostError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/users/{user_id}/convert_to_bot", configuration.base_path, user_id=crate::apis::urlencode(user_id));
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
        let entity: Option<UsersUserIdConvertToBotPostError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

