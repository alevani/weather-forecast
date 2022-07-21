use hyper::{Body, Response};
use hyper::body as BodyParser;

pub enum Env {
    OpenWeatherApiKey,
    OpenWeatherApiUrl
}

impl Env {
    pub fn value(&self) -> &str {
        match *self {
            Env::OpenWeatherApiKey => "OPEN_WEATHER_API_KEY",
            Env::OpenWeatherApiUrl => "OPEN_WEATHER_API_URL",
        }
    }
}

async fn parse_response<T: serde::de::DeserializeOwned>(response: Response<Body>) -> Result<T, String> {
    match BodyParser::to_bytes(response.into_body()).await {
        Ok(content) => match serde_json::from_slice::<T>(&content) {
            Ok(parsed_content) => Ok(parsed_content),
            Err(err) => Err(err.to_string()),
        },
        Err(err) => Err(err.to_string()),
    }
}