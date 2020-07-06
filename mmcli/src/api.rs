use crate::hook;
use anyhow::{bail, Context, Result};
use derive_new::new;
use futures::{
    future::{abortable, AbortHandle},
    prelude::*,
};
use log::*;
use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message as WsMessage};

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
pub struct Post {
    pub channel_id: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, new, Clone)]
pub struct PostRep {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, new, Clone)]
pub struct Reaction {
    pub user_id: String,
    pub post_id: String,
    pub emoji_name: String,
    pub create_at: i64,
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

#[derive(Serialize, Deserialize, Debug, new, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
}

type Sender = mpsc::UnboundedSender<WsMessage>;
type Receiver = mpsc::UnboundedReceiver<WsMessage>;

pub struct Api {
    down_rx: Receiver,
    up_tx: Sender,
    cli: Client,
    hook: Option<hook::Hook>,
    cfg: Config,
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
        let cli = Client::new();

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

        Ok(Self {
            down_rx,
            up_tx,
            cli,
            hook,
            cfg,
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
    pub async fn post(&mut self, post: Post) -> Result<PostRep> {
        self.post_http("/posts", post.clone())
            .await
            .with_context(|| format!("Couldn't post message: {:?}", post))
    }

    /// Post to incoming webhook.
    pub async fn post_webhook(&mut self, post: hook::Post) -> Result<()> {
        if let Some(hook) = self.hook.as_mut() {
            hook.post(post)
                .await
                .context("Cannot post to incoming webhook")
        } else {
            bail!("Incoming webhook is not configured")
        }
    }

    /// Get user info by usernames
    pub async fn usernames(&mut self, names: Vec<String>) -> Result<Vec<User>> {
        self.post_http("/users/usernames", names.clone())
            .await
            .with_context(|| format!("Couldn't post message: {:?}", names))
    }

    /// Create a reaction
    pub async fn reaction(&mut self, react: Reaction) -> Result<()> {
        let rep: Reaction = self
            .post_http("/reactions", react.clone())
            .await
            .with_context(|| format!("Couldn't create a reaction: {:?}", react))?;
        debug!("Reaction succeeds with response: {:?}", rep);
        Ok(())
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

    fn https_url(&self, path: &str) -> String {
        format!(
            "https://{}{}{}",
            self.cfg.url,
            self.cfg.base_path.as_deref().unwrap_or(""),
            path
        )
    }

    /// Post a message to the channel via http.
    pub async fn post_http<T: Serialize, U: DeserializeOwned>(
        &mut self,
        path: &str,
        msg: T,
    ) -> Result<U> {
        let url = self.https_url(path);
        let res = self
            .cli
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.cfg.token))
            .json(&msg)
            .send()
            .await
            .with_context(|| format!("Couldn't post a HTTP request: {}", path))?;
        if res.status().is_success() {
            Ok(res
                .json()
                .await
                .with_context(|| format!("Couldn't parse response as json: {}", path))?)
        } else {
            bail!("Http error: {:?}", res)
        }
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
