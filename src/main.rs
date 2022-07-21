use std::{env};
extern crate pretty_env_logger;

use hyper::{Client, Request, Method, Body, Response, Error, client::{ResponseFuture, HttpConnector}};
use hyper_tls::HttpsConnector;

// use std::str::FromStr;

pub mod utils;
use utils::Env;

type HttpsClient = Client<HttpsConnector<HttpConnector>>;

async fn fetch_current_weather(https_client: &HttpsClient) -> ResponseFuture {
    
    let uri = format!();

    let req = Request::builder()
    .method(Method::GET)
    // .uri(format!(
    //     "{}/MemberIdentities/{}?code={}&idtype={}",
    //     get_env(Env::MisBaseUrl),
    //     user_uuid,
    //     get_env(Env::MisFunctionKey),
    //     user_login_method_type
    // ))
    .body(Body::empty()).unwrap(); // todo match here

    https_client.request(req)
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // let api_key = env::var(Env::OpenWeatherApiKey.value());
    let https = HttpsConnector::new();
    let https_client = Client::builder().build(https);
    fetch_current_weather(&https_client).await;
}
