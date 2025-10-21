use std::env;
use dotenv::dotenv;

#[derive(Clone)]
pub struct AppSettings {
    pub listen: String,
    pub address: String, 
    pub display_topic: String,
    pub buffer_topic: String,
    pub discord: String
}

#[derive(Clone)]
pub struct MqttConfig {
    pub address: String, 
    pub display_topic: String,
    pub buffer_topic: String,
    pub discord: String
}

impl AppSettings {
    pub fn new() -> Result<Self, String> {
        dotenv().ok();

        Ok(AppSettings {
            listen: get_env_key("LISTEN")?,
            address: get_env_key("ADDRESS")?, 
            display_topic: get_env_key("DISPLAY_TOPIC")?,
            buffer_topic: get_env_key("BUFFER_TOPIC")?,
            discord: get_env_key("DISCORD")?,
        })
    }

    pub fn get_mqtt_config(&self) -> MqttConfig {
        MqttConfig {
            address: self.address.clone(),
            display_topic: self.display_topic.clone(),
            buffer_topic: self.buffer_topic.clone(),
            discord: self.discord.clone()
        }
    }
}

fn get_env_key(key: &str) -> Result<String, String> {
    match env::var(key) {
        Ok(v) => Ok(v),
        Err(_) => Err(format!("Could not find `{}` in .env file", key))
    }
}
