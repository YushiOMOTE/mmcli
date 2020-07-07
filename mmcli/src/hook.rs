use anyhow::{anyhow, Result};
use derive_new::new;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, new)]
pub struct Config {
    pub url: String,
    pub hook_id: String,
}

pub struct Hook {
    cfg: Config,
    cli: Client,
}

#[derive(Serialize, Deserialize, Debug, new)]
pub struct Post {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    pub channel: String,
    pub username: String,
    pub text: String,
}

impl Hook {
    pub fn new(cfg: Config) -> Self {
        let cli = Client::new();

        Self { cfg, cli }
    }

    pub async fn post(&mut self, post: Post) -> Result<()> {
        let mut form = HashMap::new();
        form.insert("payload", serde_json::to_string(&post)?);
        let res = self
            .cli
            .post(&format!(
                "https://{}/hooks/{}",
                self.cfg.url, self.cfg.hook_id
            ))
            .form(&form)
            .send()
            .await?;
        if res.status().is_success() {
            Ok(())
        } else {
            Err(anyhow!("Http error: {:?}", res))
        }
    }
}
