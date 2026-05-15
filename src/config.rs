use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub llm: LlmConfig,
}

#[derive(Debug, Deserialize)]
pub struct LlmConfig {
    pub base_url: String,
    pub api_key: String,
}

impl Config {
    /// Loads config layered in order: committed `config.toml` placeholders,
    /// then `config.local.toml` overrides (gitignored, optional), then
    /// `AGENTIC_HARNESS_LLM__BASE_URL` style env-var overrides.
    ///
    /// # Errors
    /// Returns the underlying figment error if a present file is malformed
    /// or required fields are absent across all layers.
    pub fn load() -> Result<Self, Box<figment::Error>> {
        Figment::new()
            .merge(Toml::file("config.toml"))
            .merge(Toml::file("config.local.toml"))
            .merge(Env::prefixed("AGENTIC_HARNESS_").split("__"))
            .extract()
            .map_err(Box::new)
    }
}
