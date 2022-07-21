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