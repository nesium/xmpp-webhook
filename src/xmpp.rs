use anyhow::Result;
use futures::FutureExt;
use jid::Jid;
use prose_xmpp::connector::xmpp_rs::Connector;
use prose_xmpp::mods::Status;
use prose_xmpp::stanza::presence::Show;
use prose_xmpp::stanza::Message;
use prose_xmpp::{
    client::Event as ClientEvent, mods, mods::chat::Event as ChatEvent, Client, ConnectionError,
    Event, Secret,
};
use tracing::{error, info, warn};

use crate::config::XMPPSettings;

#[derive(Clone)]
pub struct XMPP {
    client: Client,
}

impl XMPP {
    pub async fn build(config: XMPPSettings) -> Result<Self> {
        let client = Client::builder()
            .set_connector_provider(Connector::provider())
            .set_event_handler(|client, event| handle_event(client, event).map(|f| f.unwrap()))
            .build();

        let jid = (config.jid + "/bot").parse()?;

        info!("Connecting as {jid}…");
        client.connect(&jid, Secret::new(config.password)).await?;
        info!("Connected.");

        client
            .get_mod::<Status>()
            .send_presence(None, Some(Show::Chat), None, None, None)?;

        Ok(Self { client })
    }

    pub fn send_message(&self, to: impl Into<Jid>, message: impl Into<String>) -> Result<()> {
        let chat = self.client.get_mod::<mods::Chat>();
        chat.send_message(to, message, &Default::default(), None)?;
        Ok(())
    }
}

async fn handle_event(_client: Client, event: Event) -> Result<()> {
    match event {
        Event::Client(event) => handle_client_event(event).await,
        Event::Chat(ChatEvent::Message(message)) => handle_received_message(message).await,
        _ => Ok(()),
    }
}

async fn handle_client_event(event: ClientEvent) -> Result<()> {
    match event {
        ClientEvent::Connected => {}
        ClientEvent::Disconnected {
            error: Some(ConnectionError::InvalidCredentials),
        } => {
            error!("Invalid credentials.")
        }
        ClientEvent::Disconnected { error } => {
            warn!(
                "Client disconnected. Reason: {}",
                error
                    .map(|error| error.to_string())
                    .unwrap_or_else(|| "<no reason given>".to_string())
            )
        }
    }

    Ok(())
}

async fn handle_received_message(message: Message) -> Result<()> {
    info!(
        "Received message from {}: {}",
        message
            .from
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_else(|| "<unknown>".to_string()),
        message.body().unwrap_or("<no body>")
    );
    Ok(())
}
