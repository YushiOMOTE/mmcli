use anyhow::Result;
use log::*;
use mmcli::*;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    #[structopt(name = "url")]
    url: String,
    #[structopt(name = "token")]
    token: String,
    #[structopt(name = "base_path")]
    base_path: Option<String>,
    #[structopt(name = "webhook_token")]
    webhook_token: Option<String>,
    #[structopt(short = "d", long = "disable-tls")]
    disable_tls: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let opt = Opt::from_args();
    let mut cfg = Config::new(opt.url, opt.token);
    cfg.base_path = opt.base_path.clone();
    cfg.webhook_token = opt.webhook_token.clone();
    cfg.disable_tls = opt.disable_tls;
    let mut api = Api::new(cfg).await?;

    api.login().await?;

    while let Some(msg) = api.poll_events().await? {
        match msg {
            Message::Event(Event::Posted(posted)) => {
                debug!("{:#?}", posted);

                let value: PostedContent = serde_json::from_str(&posted.data.post)?;
                info!(
                    "User {} posted in channel {} (channel_id: {}): {}",
                    posted.data.sender_name,
                    posted.data.channel_name,
                    posted.broadcast.channel_id,
                    value.message
                );

                if value.message.starts_with("@test-bot") {
                    api.post_as(
                        &posted.data.channel_name,
                        "test-bot",
                        "hello from test-bot",
                        None,
                    )
                    .await?;
                }
            }
            _ => {}
        }
    }
    Ok(())
}
