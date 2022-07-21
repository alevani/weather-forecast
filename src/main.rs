extern crate pretty_env_logger;

use hyper::{Client, Request, Method, Body, Response, client::{ResponseFuture, HttpConnector}};
use hyper_tls::HttpsConnector;

use chrono::{Datelike, Timelike, Utc};

pub mod utils;
use utils::{Env, parse_response, get_env};

type HttpsClient = Client<HttpsConnector<HttpConnector>>;

pub struct City {
    lat: f32,
    long: f32,
    name: String
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OpenWeatherApiOneCallResponse {
    current: CurrentWeatherData
}

#[derive(Serialize, Deserialize)]
pub struct CurrentWeatherData {
    sunrise: i32,
    sunset: i32,
    temp: f32,
    feels_like: f32
}

fn fetch_current_weather(https_client: &HttpsClient, city: &City) -> ResponseFuture {
    let open_weather_api_key =  get_env(Env::OpenWeatherApiKey);
    let open_weather_api_url =  get_env(Env::OpenWeatherApiUrl);
    
    
    let uri = format!(
        "{}/onecall?lat={}&lon={}&appid={}",
        open_weather_api_url,
        city.lat,
        city.long,
        open_weather_api_key
    );
    
    let req = Request::builder()
    .method(Method::GET)
    .uri(uri)
    .body(Body::empty()).unwrap(); // todo match here

    https_client.request(req)
}



#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let city = City { lat: 55.676098, long: 12.568337, name: String::from("city") };
    
    let https = HttpsConnector::new();
    let https_client = Client::builder().build(https);
    let current_weather_raw_data = match fetch_current_weather(&https_client, &city).await {
        Ok(response_body) => response_body,
        Err(_) => panic!("Could not fetch OpenWeatherApi current weather for: {}", city.name)
    };

    let current_weather_data = match parse_response::<OpenWeatherApiOneCallResponse>(current_weather_raw_data).await {
        Ok(content) => content,
        Err(_) => panic!("Could not parse content of current_weather_raw_data")
    };

    print!(
    "
    Current weather information for {}:
    The sun will rise at {} and set at {}
    It is currently {} but it feels like {}.
    ",
    city.name,
    current_weather_data.current.sunrise,
    current_weather_data.current.sunset,
    current_weather_data.current.temp,
    current_weather_data.current.feels_like
    );
}
