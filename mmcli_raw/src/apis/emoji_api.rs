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


/// struct for typed errors of method `emoji_autocomplete_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmojiAutocompleteGetError {
    Status400(crate::models::AppError),
    Status401(crate::models::AppError),
    Status403(crate::models::AppError),
    Status501(crate::models::AppError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `emoji_emoji_id_delete`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmojiEmojiIdDeleteError {
    Status400(crate::models::AppError),
    Status401(crate::models::AppError),
    Status403(crate::models::AppError),
    Status501(crate::models::AppError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `emoji_emoji_id_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmojiEmojiIdGetError {
    Status400(crate::models::AppError),
    Status401(crate::models::AppError),
    Status404(crate::models::AppError),
    Status501(crate::models::AppError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `emoji_emoji_id_image_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmojiEmojiIdImageGetError {
    Status401(crate::models::AppError),
    Status403(crate::models::AppError),
    Status404(crate::models::AppError),
    Status500(crate::models::AppError),
    Status501(crate::models::AppError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `emoji_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmojiGetError {
    Status400(crate::models::AppError),
    Status401(crate::models::AppError),
    Status403(crate::models::AppError),
    Status501(crate::models::AppError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `emoji_name_emoji_name_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmojiNameEmojiNameGetError {
    Status400(crate::models::AppError),
    Status401(crate::models::AppError),
    Status404(crate::models::AppError),
    Status501(crate::models::AppError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `emoji_post`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmojiPostError {
    Status400(crate::models::AppError),
    Status401(crate::models::AppError),
    Status403(crate::models::AppError),
    Status413(crate::models::AppError),
    Status501(crate::models::AppError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `emoji_search_post`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmojiSearchPostError {
    Status400(crate::models::AppError),
    Status401(crate::models::AppError),
    Status403(crate::models::AppError),
    Status501(crate::models::AppError),
    UnknownValue(serde_json::Value),
}


/// Get a list of custom emoji with names starting with or matching the provided name. Returns a maximum of 100 results. ##### Permissions Must be authenticated.  __Minimum server version__: 4.7 
pub async fn emoji_autocomplete_get(configuration: &configuration::Configuration, name: &str) -> Result<crate::models::Emoji, Error<EmojiAutocompleteGetError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/emoji/autocomplete", configuration.base_path);
    let mut req_builder = client.get(uri_str.as_str());

    req_builder = req_builder.query(&[("name", &name.to_string())]);
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
        let entity: Option<EmojiAutocompleteGetError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Delete a custom emoji. ##### Permissions Must have the `manage_team` or `manage_system` permissions or be the user who created the emoji. 
pub async fn emoji_emoji_id_delete(configuration: &configuration::Configuration, emoji_id: &str) -> Result<crate::models::Emoji, Error<EmojiEmojiIdDeleteError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/emoji/{emoji_id}", configuration.base_path, emoji_id=crate::apis::urlencode(emoji_id));
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
        let entity: Option<EmojiEmojiIdDeleteError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Get some metadata for a custom emoji. ##### Permissions Must be authenticated. 
pub async fn emoji_emoji_id_get(configuration: &configuration::Configuration, emoji_id: &str) -> Result<crate::models::Emoji, Error<EmojiEmojiIdGetError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/emoji/{emoji_id}", configuration.base_path, emoji_id=crate::apis::urlencode(emoji_id));
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
        let entity: Option<EmojiEmojiIdGetError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Get the image for a custom emoji. ##### Permissions Must be authenticated. 
pub async fn emoji_emoji_id_image_get(configuration: &configuration::Configuration, emoji_id: &str) -> Result<(), Error<EmojiEmojiIdImageGetError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/emoji/{emoji_id}/image", configuration.base_path, emoji_id=crate::apis::urlencode(emoji_id));
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
        let entity: Option<EmojiEmojiIdImageGetError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Get a page of metadata for custom emoji on the system. Since server version 4.7, sort using the `sort` query parameter. ##### Permissions Must be authenticated. 
pub async fn emoji_get(configuration: &configuration::Configuration, page: Option<i64>, per_page: Option<i64>, sort: Option<&str>) -> Result<crate::models::Emoji, Error<EmojiGetError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/emoji", configuration.base_path);
    let mut req_builder = client.get(uri_str.as_str());

    if let Some(ref s) = page {
        req_builder = req_builder.query(&[("page", &s.to_string())]);
    }
    if let Some(ref s) = per_page {
        req_builder = req_builder.query(&[("per_page", &s.to_string())]);
    }
    if let Some(ref s) = sort {
        req_builder = req_builder.query(&[("sort", &s.to_string())]);
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
        let entity: Option<EmojiGetError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Get some metadata for a custom emoji using its name. ##### Permissions Must be authenticated.  __Minimum server version__: 4.7 
pub async fn emoji_name_emoji_name_get(configuration: &configuration::Configuration, emoji_name: &str) -> Result<crate::models::Emoji, Error<EmojiNameEmojiNameGetError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/emoji/name/{emoji_name}", configuration.base_path, emoji_name=crate::apis::urlencode(emoji_name));
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
        let entity: Option<EmojiNameEmojiNameGetError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Create a custom emoji for the team. ##### Permissions Must be authenticated. 
pub async fn emoji_post(configuration: &configuration::Configuration, image: std::path::PathBuf, emoji: &str) -> Result<crate::models::Emoji, Error<EmojiPostError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/emoji", configuration.base_path);
    let mut req_builder = client.post(uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    let mut form = reqwest::multipart::Form::new();
    // TODO: use async fs
    let mut f = std::fs::File::open(image.clone()).unwrap();
    use std::io::Read;
    let mut b = Vec::<u8>::new();
    f.read_to_end(&mut b).unwrap();
    form = form.part(
        "image",
        reqwest::multipart::Part::bytes(b)
            .file_name(
                image
                    .file_name()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|| "".into()),
            )
            .mime_str(mime_guess::from_path(image).first_or_octet_stream().as_ref())
            .unwrap(),
    );
    form = form.text("emoji", emoji.to_string());
    req_builder = req_builder.multipart(form);

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if status.is_success() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<EmojiPostError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Search for custom emoji by name based on search criteria provided in the request body. A maximum of 200 results are returned. ##### Permissions Must be authenticated.  __Minimum server version__: 4.7 
pub async fn emoji_search_post(configuration: &configuration::Configuration, inline_object63: crate::models::InlineObject63) -> Result<Vec<crate::models::Emoji>, Error<EmojiSearchPostError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/emoji/search", configuration.base_path);
    let mut req_builder = client.post(uri_str.as_str());

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&inline_object63);

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if status.is_success() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<EmojiSearchPostError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

