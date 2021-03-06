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


/// struct for typed errors of method `brand_image_delete`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrandImageDeleteError {
    Status401(crate::models::AppError),
    Status403(crate::models::AppError),
    Status404(crate::models::AppError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `brand_image_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrandImageGetError {
    Status404(crate::models::AppError),
    Status501(crate::models::AppError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `brand_image_post`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrandImagePostError {
    Status400(crate::models::AppError),
    Status401(crate::models::AppError),
    Status403(crate::models::AppError),
    Status413(crate::models::AppError),
    Status501(crate::models::AppError),
    UnknownValue(serde_json::Value),
}


/// Deletes the previously uploaded brand image. Returns 404 if no brand image has been uploaded. ##### Permissions Must have `manage_system` permission. __Minimum server version: 5.6__ 
pub async fn brand_image_delete(configuration: &configuration::Configuration, ) -> Result<crate::models::StatusOk, Error<BrandImageDeleteError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/brand/image", configuration.base_path);
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
        let entity: Option<BrandImageDeleteError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Get the previously uploaded brand image. Returns 404 if no brand image has been uploaded. ##### Permissions No permission required. 
pub async fn brand_image_get(configuration: &configuration::Configuration, ) -> Result<String, Error<BrandImageGetError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/brand/image", configuration.base_path);
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
        let entity: Option<BrandImageGetError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

/// Uploads a brand image. ##### Permissions Must have `manage_system` permission. 
pub async fn brand_image_post(configuration: &configuration::Configuration, image: std::path::PathBuf) -> Result<crate::models::StatusOk, Error<BrandImagePostError>> {

    let client = &configuration.client;

    let uri_str = format!("{}/brand/image", configuration.base_path);
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
    req_builder = req_builder.multipart(form);

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if status.is_success() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<BrandImagePostError> = serde_json::from_str(&content).ok();
        let error = ResponseContent { status, content, entity };
        Err(Error::ResponseError(error))
    }
}

