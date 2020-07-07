use crate::hook;
use anyhow::{anyhow, bail, Context, Result};
use derive_new::new;
use futures::{
    future::{abortable, AbortHandle},
    prelude::*,
};
use log::*;
use mmcli_raw::{
    apis::{configuration::Configuration, *},
    models,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message as WsMessage};

macro_rules! t {
    ($e:expr) => {
        $e.map_err(|e| anyhow!("Request error: {:?}", e))
    };
}

pub type WebhookPost = hook::Post;

#[derive(Serialize, Deserialize, Debug, new, Clone)]
pub struct AuthToken {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug, new, Clone)]
pub struct Auth {
    pub seq: usize,
    pub action: String,
    pub data: AuthToken,
}

#[derive(Serialize, Deserialize, Debug, new, Clone)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum Event {
    AddedToTeam,
    AuthenticationChallenge,
    ChannelConverted,
    ChannelCreated,
    ChannelDeleted,
    ChannelMemberUpdated,
    ChannelUpdated,
    ChannelViewed,
    ConfigChanged,
    DeleteTeam,
    DirectAdded,
    EmojiAdded,
    EphemeralMessage,
    GroupAdded,
    Hello,
    LeaveTeam,
    LicenseChanged,
    MemberroleUpdated,
    NewUser,
    PluginDisabled,
    PluginEnabled,
    PluginStatusesChanged,
    PostDeleted,
    PostEdited,
    PostUnread,
    Posted(Posted),
    PreferenceChanged,
    PreferencesChanged,
    PreferencesDeleted,
    ReactionAdded,
    ReactionRemoved,
    Response,
    RoleUpdated,
    StatusChange,
    Typing,
    UpdateTeam,
    UserAdded,
    UserRemoved,
    UserRoleUpdated,
    UserUpdated,
    DialogOpened,
    #[serde(other)]
    Unsupported,
}

#[derive(Serialize, Deserialize, Debug, new, Clone)]
#[serde(untagged)]
pub enum Message {
    Event(Event),
    Status(Status),
}

#[derive(Serialize, Deserialize, Debug, new, Clone)]
pub struct Status {
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug, new, Clone)]
pub struct Posted {
    pub data: PostedData,
    pub broadcast: Broadcast,
}

#[derive(Serialize, Deserialize, Debug, new, Clone)]
pub struct PostedData {
    pub channel_display_name: String,
    pub channel_name: String,
    pub channel_type: String,
    pub post: String,
    pub sender_name: String,
    pub team_id: String,
}

#[derive(Serialize, Deserialize, Debug, new, Clone)]
pub struct PostedContent {
    pub id: String,
    pub user_id: String,
    pub channel_id: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, new, Clone)]
pub struct Broadcast {
    pub channel_id: String,
    pub user_id: String,
    pub team_id: String,
}

#[derive(Serialize, Deserialize, Debug, new, Clone)]
pub struct Config {
    pub url: String,
    #[new(default)]
    pub base_path: Option<String>,
    pub token: String,
    #[new(default)]
    pub webhook_token: Option<String>,
}

type Sender = mpsc::UnboundedSender<WsMessage>;
type Receiver = mpsc::UnboundedReceiver<WsMessage>;

pub struct Api {
    down_rx: Receiver,
    up_tx: Sender,
    hook: Option<hook::Hook>,
    cfg: Config,
    raw: Configuration,
    handle: AbortHandle,
}

impl Drop for Api {
    fn drop(&mut self) {
        self.handle.abort();
    }
}

impl Api {
    /// Create a new client instance.
    pub async fn new(cfg: Config) -> Result<Self> {
        let hook = cfg
            .webhook_token
            .as_ref()
            .map(|token| hook::Hook::new(hook::Config::new(cfg.url.clone(), token.to_string())));

        let urlstr = format!(
            "wss://{}{}/websocket",
            cfg.url,
            cfg.base_path.as_deref().unwrap_or("")
        );
        let url =
            url::Url::parse(&urlstr).with_context(|| format!("Couldn't parse url: {}", urlstr))?;

        let (mut sock, _) = connect_async(url)
            .await
            .with_context(|| format!("Couldn't connect websocket: {}", urlstr))?;

        let (up_tx, mut up_rx) = mpsc::unbounded_channel();
        let (down_tx, down_rx) = mpsc::unbounded_channel();

        let (task, handle) = abortable(async move {
            let mut hbs = tokio::time::interval(std::time::Duration::from_secs(5));

            loop {
                enum E<A, B, C> {
                    A(A),
                    B(B),
                    C(C),
                }

                let msg = tokio::select! {
                    up = up_rx.next() => { E::A(up) },
                    down = sock.next() => { E::B(down) },
                    hb = hbs.next() => { E::C(hb) },
                };

                match msg {
                    E::A(up) => {
                        let m = match up {
                            None => break,
                            Some(m) => m,
                        };

                        if let Err(e) = sock.send(m).await {
                            error!("Couldn't send websocket message: {}", e);
                            break;
                        }
                    }
                    E::B(down) => {
                        let m = match down {
                            None => break,
                            Some(Ok(m)) => m,
                            Some(Err(e)) => {
                                error!("Couldn't receive websocket message: {}", e);
                                break;
                            }
                        };
                        match &m {
                            WsMessage::Ping(p) => {
                                if let Err(e) = sock.send(WsMessage::Pong(p.to_vec())).await {
                                    error!("Couldn't send websocket pong: {}", e);
                                    break;
                                }
                            }
                            WsMessage::Pong(_) => {
                                continue;
                            }
                            WsMessage::Close(_) => {
                                let _ = down_tx.send(m);
                                info!("Connection closed by peer");
                                break;
                            }
                            _ => {}
                        }

                        if let Err(_) = down_tx.send(m) {
                            break;
                        }
                    }
                    E::C(_) => {
                        if let Err(e) = sock.send(WsMessage::Ping(vec![])).await {
                            error!("Couldn't send websocket ping: {}", e);
                            break;
                        }
                    }
                }
            }
        });

        tokio::spawn(task);

        let mut raw = Configuration::new();

        raw.base_path = format!(
            "https://{}{}",
            cfg.url,
            cfg.base_path.as_deref().unwrap_or(""),
        );
        raw.bearer_access_token = Some(cfg.token.clone());

        Ok(Self {
            down_rx,
            up_tx,
            hook,
            cfg,
            raw,
            handle,
        })
    }

    /// Login to the mettermost for the websocket channel.
    pub async fn login(&mut self) -> Result<()> {
        self.send_ws(Auth::new(
            1,
            "authentication_challenge".into(),
            AuthToken::new(self.cfg.token.clone()),
        ))
        .await
        .context("Couldn't send authentication request")?;

        let msg = self
            .recv_ws()
            .await
            .context("Couldn't receive hello to the authentication request")?;

        match msg {
            Some(Message::Event(Event::Hello)) => {}
            Some(msg) => bail!("Unexpected message: {:?}", msg),
            None => bail!("Couldn't receive hello after connect"),
        }

        let msg = self
            .recv_ws()
            .await
            .context("Couldn't receive initial status response")?;

        match msg {
            Some(Message::Status(s)) => {
                if s.status == "OK" {
                    Ok(())
                } else {
                    bail!("Received bad status: {:?}", s)
                }
            }
            Some(msg) => bail!("Unexpected message: {:?}", msg),
            None => bail!("Couldn't receive status after login"),
        }
    }

    /// Post a message to the channel.
    pub async fn post(&mut self, channel_id: &str, message: &str) -> Result<models::Post> {
        let p = posts_api::posts_post(
            &self.raw,
            models::InlineObject52::new(channel_id.into(), message.into()),
            None,
        )
        .await;

        Ok(t!(p).with_context(|| format!("Couldn't post: {}: {}", channel_id, message))?)
    }

    /// Post via incoming webhook.
    pub async fn post_as(
        &mut self,
        channel_name: &str,
        username: &str,
        message: &str,
        icon_url: Option<&str>,
    ) -> Result<()> {
        if let Some(hook) = self.hook.as_mut() {
            hook.post(hook::Post::new(
                icon_url.map(|s| s.into()),
                channel_name.into(),
                username.into(),
                message.into(),
            ))
            .await
            .with_context(|| {
                format!(
                    "Couldn't post via incoming webhook: {}: {}: {}",
                    channel_name, username, message,
                )
            })?;

            Ok(())
        } else {
            bail!("Webhook is not configured")
        }
    }

    /// Get user info by usernames
    pub async fn usernames(&mut self, names: Vec<String>) -> Result<Vec<models::User>> {
        let p = users_api::users_usernames_post(&self.raw, names.clone()).await;

        Ok(t!(p).with_context(|| format!("Couldn't get usernames: {:?}", names))?)
    }

    /// Create a reaction
    pub async fn reaction(&mut self, react: models::Reaction) -> Result<models::Reaction> {
        let p = reactions_api::reactions_post(&self.raw, react.clone()).await;

        Ok(t!(p).with_context(|| format!("Couldn't set reaction: {:?}", react))?)
    }

    /// Send a message to the channel via websocket.
    async fn send_ws<T: Serialize>(&mut self, msg: T) -> Result<()> {
        let msg = serde_json::to_string(&msg)
            .context("Couldn't pack message to json to send to websocket")?;
        self.up_tx
            .send(WsMessage::Text(msg.clone()))
            .with_context(|| format!("Couldn't send message to websocket: {}", msg))?;
        Ok(())
    }

    /// Receive an event from a websocket.
    pub async fn poll_events(&mut self) -> Result<Option<Message>> {
        self.recv_ws().await
    }

    async fn recv_ws<T: DeserializeOwned>(&mut self) -> Result<Option<T>> {
        while let Some(msg) = self.down_rx.next().await {
            match msg {
                WsMessage::Text(msg) => {
                    debug!("Received text from websocket: {}", msg);
                    return Ok(Some(serde_json::from_str(&msg).with_context(|| {
                        format!("Couldn't parse text message: {}", msg)
                    })?));
                }
                WsMessage::Binary(msg) => {
                    debug!(
                        "Received binary from websocket: {}",
                        String::from_utf8_lossy(&msg)
                    );
                    return Ok(Some(serde_json::from_slice(&msg).with_context(|| {
                        format!("Couldn't parse binary message: {:x?}", msg)
                    })?));
                }
                WsMessage::Ping(_) => {}
                WsMessage::Pong(_) => {}
                WsMessage::Close(_) => return Ok(None),
            }
        }

        Ok(None)
    }
}
