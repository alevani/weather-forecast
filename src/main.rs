use std::{env};
extern crate pretty_env_logger;

use hyper::{Client, Request, Method, Body, Response, client::{ResponseFuture, HttpConnector}};
use hyper_tls::HttpsConnector;

// use std::str::FromStr;

pub mod utils;
use utils::Env;

type HttpsClient = Client<HttpsConnector<HttpConnector>>;

pub struct City {
    lat: f32,
    long: f32,
    name: String
}

async fn fetch_current_weather(https_client: &HttpsClient, city: &City) -> ResponseFuture {
    
    let uri = format!(
        "{}/onecall=lat{}&lon={}&appid={:?}",
        Env::OpenWeatherApiUrl.value(),
        city.lat, city.long,
        env::var(Env::OpenWeatherApiKey.value())
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

    let copenhagen = City { lat: 55.676098, long: 12.568337, name: String::from("Copenhagen") };
    
    let https = HttpsConnector::new();
    let https_client = Client::builder().build(https);
    fetch_current_weather(&https_client, &copenhagen).await;
}
