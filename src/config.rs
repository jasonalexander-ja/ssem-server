use std::env;
use dotenv::dotenv;

#[derive(Clone)]
pub struct AppSettings {
    pub listen: String,
    pub address: String, 
    pub topic: String,
}

#[derive(Clone)]
pub struct MqttConfig {
    pub address: String, 
    pub topic: String,
}

impl AppSettings {
    pub fn new() -> Result<Self, String> {
        dotenv().ok();

        Ok(AppSettings {
            listen: get_env_key("LISTEN")?,
            address: get_env_key("ADDRESS")?, 
            topic: get_env_key("TOPIC")?,
        })
    }

    pub fn get_mqtt_config(&self) -> MqttConfig {
        MqttConfig {
            address: self.address.clone(),
            topic: self.topic.clone(),
        }
    }
}

fn get_env_key(key: &str) -> Result<String, String> {
    match env::var(key) {
        Ok(v) => Ok(v),
        Err(_) => Err(format!("Could not find `{}` in .env file", key))
    }
}
