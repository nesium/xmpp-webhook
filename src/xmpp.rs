use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use anyhow::Result;
use futures::FutureExt;
use prose_xmpp::connector::xmpp_rs::Connector;
use prose_xmpp::stanza::message::MessageType;
use prose_xmpp::stanza::presence::Show;
use prose_xmpp::stanza::Message;
use prose_xmpp::{
    client::Event as ClientEvent, mods, mods::chat::Event as ChatEvent, BareJid, Client,
    ConnectionError, Event, Jid, Secret,
};
use tokio::sync::mpsc;
use tracing::{info, warn};

use crate::config::XMPPSettings;

pub enum RoomId {
    User(BareJid),
    Room(BareJid),
}

impl From<RoomId> for Jid {
    fn from(value: RoomId) -> Self {
        match value {
            RoomId::User(jid) => jid,
            RoomId::Room(jid) => jid,
        }
        .into()
    }
}

#[derive(Clone)]
pub struct XMPPHandle {
    sender: mpsc::Sender<XMPPServiceMessage>,
}

impl XMPPHandle {
    pub fn new(config: XMPPSettings, rooms: Vec<BareJid>) -> Self {
        let (sender, receiver) = mpsc::channel(20);
        let mut actor = XMPPService::new(config, rooms, receiver);
        tokio::spawn(async move { actor.run().await });
        Self { sender }
    }

    pub fn send_message(&self, to: RoomId, message: impl Into<String>) {
        self.sender
            .send(XMPPServiceMessage::SendMessage {
                to,
                body: message.into(),
            })
            .now_or_never();
    }
}

enum XMPPServiceMessage {
    SendMessage { to: RoomId, body: String },
}

struct XMPPService {
    config: XMPPSettings,
    rooms: Vec<BareJid>,
    receiver: mpsc::Receiver<XMPPServiceMessage>,
    client: Client,
    is_connected: Arc<AtomicBool>,
}

impl XMPPService {
    fn new(
        config: XMPPSettings,
        rooms: Vec<BareJid>,
        receiver: mpsc::Receiver<XMPPServiceMessage>,
    ) -> Self {
        let is_connected = Arc::new(AtomicBool::new(false));

        let client = Client::builder()
            .set_connector_provider(Connector::provider())
            .set_event_handler({
                let is_connected = is_connected.clone();
                move |client, event| {
                    let is_connected = is_connected.clone();
                    async move {
                        handle_event(client, event, &is_connected);
                    }
                }
            })
            .build();

        Self {
            config,
            rooms,
            receiver,
            client,
            is_connected,
        }
    }

    async fn run(&mut self) {
        _ = self.connect_if_needed().await;

        while let Some(msg) = self.receiver.recv().await {
            _ = self.connect_if_needed().await;
            _ = self.handle_message(msg).await;
        }
    }

    async fn handle_message(&mut self, msg: XMPPServiceMessage) -> Result<()> {
        match msg {
            XMPPServiceMessage::SendMessage { to, body } => {
                let chat = self.client.get_mod::<mods::Chat>();
                let message_type = match &to {
                    RoomId::User(_) => MessageType::Chat,
                    RoomId::Room(_) => MessageType::Groupchat,
                };
                chat.send_message(to, body, &message_type, None)?;
            }
        }
        Ok(())
    }

    async fn connect_if_needed(&self) -> Result<()> {
        if self.is_connected.load(Ordering::Acquire) {
            return Ok(());
        }

        let jid = self
            .config
            .jid
            .with_resource_str("bot")
            .expect("Failed to append resource string to jid");

        info!("Connecting as {jid}…");
        self.client
            .connect(&jid, Secret::new(self.config.password.clone()))
            .await?;
        info!("Connected.");

        self.client.get_mod::<mods::Status>().send_presence(
            None,
            Some(Show::Chat),
            None,
            None,
            None,
        )?;

        let muc = self.client.get_mod::<mods::MUC>();
        for room in &self.rooms {
            info!("Entering room {room}…");
            muc.enter_room(
                &room.with_resource_str("bot").unwrap(),
                None,
                Some(Show::Chat),
                None,
            )
            .await?;
        }

        let chat = self.client.get_mod::<mods::Chat>();
        chat.send_message(
            Jid::new("marc@prose.org").unwrap(),
            "🚀",
            &MessageType::Chat,
            None,
        )?;

        Ok(())
    }
}

fn handle_event(_client: Client, event: Event, is_connected: &AtomicBool) {
    match event {
        Event::Client(event) => handle_client_event(event, is_connected),
        Event::Chat(ChatEvent::Message(message)) => handle_received_message(message),
        _ => (),
    }
}

fn handle_client_event(event: ClientEvent, is_connected: &AtomicBool) {
    match event {
        ClientEvent::Connected => is_connected.store(true, Ordering::Release),
        ClientEvent::Disconnected {
            error: Some(ConnectionError::InvalidCredentials),
        } => {
            panic!("Invalid credentials for XMPP account.")
        }
        ClientEvent::Disconnected { error } => {
            warn!(
                "Client disconnected. Reason: {}",
                error
                    .map(|error| error.to_string())
                    .unwrap_or_else(|| "<no reason given>".to_string())
            );
            is_connected.store(false, Ordering::Release);
        }
        ClientEvent::PingTimer => {}
    }
}

fn handle_received_message(message: Message) {
    info!(
        "Received message from {}: {}",
        message
            .from
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_else(|| "<unknown>".to_string()),
        message.body().unwrap_or("<no body>")
    );
}
