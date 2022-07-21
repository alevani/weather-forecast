use hyper::{Client, Request, Method, Body, client::{ResponseFuture, HttpConnector}};
use hyper_tls::HttpsConnector;

use crate::models::city::City;
use crate::utils::{ Env, get_env };

type HttpsClient = Client<HttpsConnector<HttpConnector>>;

pub fn fetch_current_weather(https_client: &HttpsClient, city: &City) -> ResponseFuture {
    let open_weather_api_key =  get_env(Env::OpenWeatherApiKey);
    let open_weather_api_url =  get_env(Env::OpenWeatherApiUrl);
    
    let uri = format!(
        "{}/onecall?lat={}&lon={}&appid={}&units=metric",
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