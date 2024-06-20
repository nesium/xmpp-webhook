use anyhow::format_err;
use config::{Config, ConfigError, File};
use prose_xmpp::BareJid;
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Deserialize, Clone, Debug)]
pub struct Settings {
    pub app: AppSettings,
    pub xmpp: XMPPSettings,
    pub webhook: WebhookSettings,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AppSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub base_url: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct XMPPSettings {
    pub jid: BareJid,
    pub password: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct WebhookSettings {
    pub repos: Vec<RepoSettings>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct RepoSettings {
    pub repo: String,
    pub room: BareJid,
}

pub enum Environment {
    Local,
    Production,
}

pub fn get_configuration() -> Result<Settings, ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("config");

    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");
    let environment_filename = format!("{}.yaml", environment.as_str());

    let settings = Config::builder()
        .add_source(File::from(configuration_directory.join("base.yaml")))
        .add_source(File::from(
            configuration_directory.join(&environment_filename),
        ))
        .add_source(File::from(configuration_directory.join("secrets.yaml")).required(false))
        // Add in settings from environment variables (with a prefix of APP and
        // '__' as separator)
        // E.g. `APP_APPLICATION__PORT=5001 would set `Settings.application.port`
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    settings.try_deserialize()
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format_err!(
                r#"{} is not a supported environment.
          Use either `local` or `production`."#,
                other
            )),
        }
    }
}
