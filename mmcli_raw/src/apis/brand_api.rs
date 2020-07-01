/*
 * Mattermost API Reference
 *
 * There is also a work-in-progress [Postman API reference](https://documenter.getpostman.com/view/4508214/RW8FERUn). 
 *
 * The version of the OpenAPI document: 4.0.0
 * Contact: feedback@mattermost.com
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused_imports)]
use std::rc::Rc;

use std::option::Option;

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
        // TODO: support file upload for 'image' parameter
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

