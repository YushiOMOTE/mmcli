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
    #[structopt(name = "channel_id")]
    channel_id: String,
    #[structopt(name = "message")]
    message: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let opt = Opt::from_args();
    let cfg = Config::new(opt.url, opt.token);

    info!(
        "Posting message {} to channel id {}",
        opt.message, opt.channel_id
    );

    let mut api = Api::new(cfg).await?;
    let rep = api.post(&opt.channel_id, &opt.message).await?;

    info!("Posted: {:?}", rep.id);

    Ok(())
}
