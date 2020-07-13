use anyhow::{anyhow, Result};
use log::*;
use mmcli::*;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    #[structopt(name = "url")]
    url: String,
    #[structopt(name = "token")]
    token: String,
    #[structopt(name = "emoji")]
    name: String,
    #[structopt(name = "image")]
    image: PathBuf,
    #[structopt(name = "username")]
    username: String,
    #[structopt(short = "d", long = "disable-tls")]
    disable_tls: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let opt = Opt::from_args();
    let mut cfg = Config::new(opt.url.clone(), opt.token.clone());
    cfg.disable_tls = opt.disable_tls;

    info!("Creating emoji {} from {}", opt.name, opt.image.display());

    let mut api = Api::new(cfg).await?;

    let user = api.usernames(vec![opt.username.clone()]).await?;
    let user = user
        .get(0)
        .ok_or_else(|| anyhow!("No user info for {}", opt.username))?;
    let id = user
        .id
        .as_ref()
        .ok_or_else(|| anyhow!("No user id for {}", opt.username))?;

    info!("{:?}", user);

    let rep = api.create_emoji(opt.image.clone(), &opt.name, &id).await?;

    info!("Created: {:?}", rep);

    Ok(())
}
