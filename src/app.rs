use std::net::TcpListener;
use std::sync::Arc;

use actix_web::dev::Server;
use actix_web::{web, HttpServer};
use anyhow::Result;
use minijinja::Environment;
use tracing::info;
use tracing_actix_web::TracingLogger;

use crate::config::{RepoSettings, Settings};
use crate::routes::{health_check, home, webhook};
use crate::services::XMPPService;
use crate::templates::get_environment;
use crate::webhook::RepoMapping;

pub struct App {
    server: Server,
    port: u16,
}

impl App {
    pub async fn build<X: XMPPService + 'static>(
        config: Settings,
        xmpp_service: X,
    ) -> Result<Self> {
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
            Arc::new(xmpp_service),
            ApplicationBaseUrl(config.app.base_url),
            config.webhook.repos,
            get_environment()?,
        )?;

        Ok(Self { server, port })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<()> {
        Ok(self.server.await?)
    }
}

pub struct ApplicationBaseUrl(pub String);

pub fn run(
    listener: TcpListener,
    xmpp: Arc<dyn XMPPService>,
    base_url: ApplicationBaseUrl,
    repo_settings: Vec<RepoSettings>,
    environment: Environment<'static>,
) -> Result<Server> {
    let xmpp = web::Data::new(xmpp);
    let base_url = web::Data::new(base_url);
    let repo_mapping = web::Data::new(RepoMapping::new(repo_settings));
    let environment = web::Data::new(environment);

    let server = HttpServer::new(move || {
        actix_web::App::new()
            .wrap(TracingLogger::default())
            .route("/", web::get().to(home))
            .route("/health_check", web::get().to(health_check))
            .route("/webhook", web::post().to(webhook))
            .app_data(xmpp.clone())
            .app_data(base_url.clone())
            .app_data(repo_mapping.clone())
            .app_data(environment.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
