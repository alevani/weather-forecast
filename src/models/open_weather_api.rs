use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OpenWeatherApiOneCallResponse {
    pub current: CurrentWeatherData
}

#[derive(Serialize, Deserialize)]
pub struct CurrentWeatherData {
    pub sunrise: i64,
    pub sunset: i64,
    pub temp: f32,
    pub feels_like: f32
}