use std::sync::{Arc, Mutex};

use xmpp_webhook::app::App;
use xmpp_webhook::config::{get_configuration, RepoSettings};
use xmpp_webhook::services::xmpp_service::RoomId;
use xmpp_webhook::services::XMPPService;

#[ctor::ctor]
fn init() {
    let _ = tracing_subscriber::fmt().with_test_writer().try_init();
}

pub struct TestApp {
    pub address: String,
    pub xmpp: MockXMPPService,
}

pub async fn spawn_app() -> TestApp {
    let mut config = get_configuration().expect("Failed to read configuration");
    // Use a random OS port
    config.app.port = 0;
    config.webhook.repos = vec![RepoSettings {
        repo: "Codertocat/Hello-World".to_string(),
        room: "room@example.org".parse().unwrap(),
    }];

    let xmpp = MockXMPPService::default();

    let app = App::build(config.clone(), xmpp.clone())
        .await
        .expect("Failed to build application");
    let port = app.port();
    let address = format!("http://127.0.0.1:{}", port);
    let _ = tokio::spawn(app.run_until_stopped());

    TestApp { address, xmpp }
}

#[derive(Default, Clone)]
pub struct MockXMPPService {
    inner: Arc<Mutex<MockXMPPServiceInner>>,
}

impl MockXMPPService {
    pub fn sent_messages(&self) -> Vec<SentMessage> {
        self.inner.lock().unwrap().sent_messages.clone()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SentMessage {
    pub to: RoomId,
    pub message: String,
}

impl XMPPService for MockXMPPService {
    fn send_message(&self, to: RoomId, message: String) {
        self.inner
            .lock()
            .unwrap()
            .sent_messages
            .push(SentMessage { to, message })
    }
}

#[derive(Default)]
struct MockXMPPServiceInner {
    sent_messages: Vec<SentMessage>,
}
