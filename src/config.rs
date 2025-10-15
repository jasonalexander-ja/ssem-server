use std::env;
use dotenv::dotenv;

#[derive(Clone)]
pub struct AppSettings {
    pub client_id: String,
    pub client_secret: String,
    pub issuer_url: String,
    pub redirect_url: String,
    pub redirect_uri: String,
    pub origin: String,
    pub listen: String,
    pub auth_cookie: String,
}

impl AppSettings {
    pub fn new() -> Result<Self, String> {
        dotenv().ok();

        Ok(AppSettings {
            client_id: get_env_key("CLIENT_ID")?,
            client_secret: get_env_key("CLIENT_SECRET")?,
            issuer_url: get_env_key("ISSUER_URL")?,
            redirect_url: get_env_key("REDIRECT_URL")?,
            redirect_uri: get_env_key("REDIRECT_URI")?,
            origin: get_env_key("ORIGIN")?,
            listen: get_env_key("LISTEN")?,
            auth_cookie: get_env_key("AUTH_COOKIE")?,
        })
    }
}

fn get_env_key(key: &str) -> Result<String, String> {
    match env::var(key) {
        Ok(v) => Ok(v),
        Err(_) => Err(format!("Could not find `{}` in .env file", key))
    }
}
