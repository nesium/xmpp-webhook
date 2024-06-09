use anyhow::Result;

use crate::app::App;
use crate::config::get_configuration;
use crate::telemetry::{build_subscriber, init_subscriber};

mod app;
mod config;
mod routes;
mod telemetry;
mod xmpp;

#[tokio::main]
async fn main() -> Result<()> {
    init_subscriber(build_subscriber("xmpp-webhook", "info", std::io::stdout));

    let config = get_configuration().expect("Failed to read configuration");
    let app = App::build(config).await?;
    app.run_until_stopped().await?;
    Ok(())
}
