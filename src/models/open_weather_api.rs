use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OpenWeatherApiOneCallResponse {
    pub current: CurrentWeatherData,
    pub alerts: Option<Vec<AlertsData>>
}

#[derive(Serialize, Deserialize)]
pub struct CurrentWeatherData {
    pub sunrise: i64,
    pub sunset: i64,
    pub temp: f32,
    pub feels_like: f32,
    pub clouds: i32,
    pub weather: Vec<WeatherData>,
    pub uvi: f32
}

#[derive(Serialize, Deserialize)]
pub struct AlertsData {
    pub sender_name: String,
    pub event: String,
    pub description: String
}

#[derive(Serialize, Deserialize)]
pub struct WeatherData {
    pub main: WeatherType,
    pub description: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]

pub enum WeatherType {
    Thunderstorm,
    Drizzle,
    Rain,
    Snow,
    Mist,
    Smoke,
    Haze,
    Dust,
    Fog,
    Sand,
    Ash,
    Squall,
    Tornado,
    Clear,
    Clouds,

    #[default]
    NoWeatherTypeReturned
}


impl fmt::Display for WeatherType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}