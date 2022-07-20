use std::env;

pub mod utils;
use utils::Env;

fn main() {

    let api_key = env::var(Env::OpenWeatherApiKey.value());
    println!("Hello, world!");

}
