use reqwest::{Client, ClientBuilder};

#[derive(Debug, Clone)]
pub struct WikiClientConfig {
    user_agent: String,
}

impl WikiClientConfig {
    fn new(user_agent: String) -> Self {
        Self { user_agent }
    }
}

impl Default for WikiClientConfig {
    fn default() -> Self {
        const PKG_NAME: &str = env!("CARGO_PKG_NAME");
        const VERSION: &str = env!("CARGO_PKG_VERSION");

        Self::new(format!("{PKG_NAME}/{VERSION}"))
    }
}

impl WikiClient {
    fn build_client(config: &WikiClientConfig) -> Client {
        let builder = ClientBuilder::new();

        let builder = builder.user_agent(config.user_agent.clone());

        builder.build().expect("client should build ok")
    }

    pub fn new(config: WikiClientConfig) -> Self {
        let client = Self::build_client(&config);
        Self { config, client }
    }
}

impl Default for WikiClient {
    fn default() -> Self {
        Self::new(WikiClientConfig::default())
    }
}

#[derive(Clone, Debug)]
pub struct WikiClient {
    pub config: WikiClientConfig,
    pub client: Client,
}
