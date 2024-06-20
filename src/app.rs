use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{web, HttpServer};
use anyhow::Result;
use tracing::info;
use tracing_actix_web::TracingLogger;

use crate::config::{RepoSettings, Settings};
use crate::routes::{health_check, home, webhook};
use crate::webhook::RepoMapping;
use crate::xmpp::XMPPHandle;

pub struct App {
    server: Server,
}

impl App {
    pub async fn build(config: Settings) -> Result<Self> {
        let address = format!(
            "{host}:{port}",
            host = config.app.host,
            port = config.app.port
        );
        let listener = TcpListener::bind(address)?;
        let port = listener
            .local_addr()
            .expect("Could not determine port.")
            .port();

        info!("Started server on {port}.");

        let server = run(
            listener,
            XMPPHandle::new(
                config.xmpp,
                config
                    .webhook
                    .repos
                    .iter()
                    .map(|setting| setting.room.clone())
                    .collect(),
            ),
            ApplicationBaseUrl(config.app.base_url),
            config.webhook.repos,
        )?;

        Ok(Self { server })
    }

    pub async fn run_until_stopped(self) -> Result<()> {
        Ok(self.server.await?)
    }
}

pub struct ApplicationBaseUrl(pub String);

pub fn run(
    listener: TcpListener,
    xmpp: XMPPHandle,
    base_url: ApplicationBaseUrl,
    repo_settings: Vec<RepoSettings>,
) -> Result<Server> {
    let xmpp = web::Data::new(xmpp);
    let base_url = web::Data::new(base_url);
    let repo_mapping = web::Data::new(RepoMapping::new(repo_settings));

    let server = HttpServer::new(move || {
        actix_web::App::new()
            .wrap(TracingLogger::default())
            .route("/", web::get().to(home))
            .route("/health_check", web::get().to(health_check))
            .route("/webhook", web::post().to(webhook))
            .app_data(xmpp.clone())
            .app_data(base_url.clone())
            .app_data(repo_mapping.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
