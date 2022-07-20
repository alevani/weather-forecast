pub enum Env {
    OpenWeatherApiKey
}

impl Env {
    pub fn value(&self) -> &str {
        match *self {
            Env::OpenWeatherApiKey => "OPEN_WEATHER_API_KEY",
        }
    }
}