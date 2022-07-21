extern crate pretty_env_logger;

use chrono::{TimeZone, Utc};
use hyper::{Client};
use hyper_tls::HttpsConnector;

pub mod utils;
use utils::{parse_response};

pub mod models;
use models::open_weather_api::*;
use models::city::*;

pub mod services;
use services::open_weather_api::*;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenv().ok();

    let city = City { lat: 55.676098, long: 12.568337, name: String::from("Copenhagen") };
    
    let https = HttpsConnector::new();
    let https_client = Client::builder().build(https);
    
    // Fetch OpenWeatherApi current weather forecast data
    let current_weather_raw_data = match fetch_current_weather(&https_client, &city).await {
        Ok(response_body) => response_body,
        Err(_) => panic!("Could not fetch OpenWeatherApi current weather for: {}", city.name)
    };

    // Parse it to program usable data
    let current_weather_data = match parse_response::<OpenWeatherApiOneCallResponse>(current_weather_raw_data).await {
        Ok(content) => content,
        Err(_) => panic!("Could not parse content of current_weather_raw_data")
    };

    println!(
    "
    Current weather information for {}:
    The sun will rise at {} and set at {}
    It is currently {}°C but it feels like {}°C.
    ",
    city.name,
    //todo: Add a 2 hours UTC for the CPH city
    Utc.timestamp(current_weather_data.current.sunrise, 0).time(),
    Utc.timestamp(current_weather_data.current.sunset, 0).time(),
    current_weather_data.current.temp,
    current_weather_data.current.feels_like
    );
}
