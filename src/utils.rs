use hyper::{Body, Response};
use hyper::body as BodyParser;
use std::env;

pub const OPEN_WEATHER_API_URL: &str = "https://api.openweathermap.org/data/2.5";

pub enum Env {
    OpenWeatherApiKey,
}

pub fn get_env(key: Env) -> String {
    let key = match key {
        Env::OpenWeatherApiKey => "OPEN_WEATHER_API_KEY"
    };

    return match env::var(key) {
        Ok(base_url) => base_url,
        Err(_) => panic!("Could not find env key: {}", key)
    };
}

pub async fn parse_response<T: serde::de::DeserializeOwned>(response: Response<Body>) -> Result<T, String> {
    match BodyParser::to_bytes(response.into_body()).await {
        Ok(content) => match serde_json::from_slice::<T>(&content) {
            Ok(parsed_content) => Ok(parsed_content),
            Err(err) => Err(err.to_string()),
        },
        Err(err) => Err(err.to_string()),
    }
}