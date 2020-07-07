# mmcli

Mattermost API client in Rust.

```rust
use mmcli::*;

#[tokio::main]
async fn main() {
    let mut cfg = Config::new("mattermost.something.com".into(), "token".into());
    cfg.base_path = Some("/api/v4".into());
    cfg.webhook_token = Some("webhook_token".into());
    let mut api = Api::new(cfg).await.unwrap();

    /// Login.
    api.login().await.unwrap();

    /// Post as something.
    api.post_as(
        "channel-name",
        "test-bot",
        "hello from test-bot",
			  None,
    )
        .await.unwrap();

    /// Keep receiving events.
    while let Some(msg) = api.poll_events().await.unwrap() {
        println!("Received: {:?}", msg);
    }
}
```

* `mmcli` ... High-level API.
* `mmcli-raw` ... OpenAPI-generated low-level API.

Still experimental ... `_(┐「ε:)_`
