use std::sync::OnceLock;

use config::Config;

pub struct Configuration;

static INSTANCE: OnceLock<Config> = OnceLock::new();

impl Configuration {
    pub fn get_instance() -> &'static Config {
        INSTANCE.get_or_init(|| {
            Config::builder()
                .add_source(config::Environment::with_prefix("APP"))
                .build()
                .unwrap()
        })
    }
}
