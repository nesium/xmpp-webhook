use anyhow::Result;

use xmpp_webhook::app::App;
use xmpp_webhook::config::get_configuration;
use xmpp_webhook::telemetry::{build_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<()> {
    init_subscriber(build_subscriber("xmpp-webhook", "info", std::io::stdout));

    let config = get_configuration().expect("Failed to read configuration");
    let app = App::build(config).await?;
    app.run_until_stopped().await?;
    Ok(())
}
