extern crate slack_api;
extern crate json;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate time;
extern crate url;

mod owm;
mod weather_types;

use std::env;
use json::object;
use std::collections::HashMap;

fn main() {
    println!("Loading app...");
    let icons: HashMap<&str, &str> = [
        ("01", ":sunny:"), //sun
        ("02", ":partly_sunny:"), //sun+cloud
        ("03", ":barely_sunny:"), //cloud
        ("04", ":cloud:"), //cloud++
        ("09", ":umbrella_with_rain_drops:"), //cloud+rain
        ("10", ":partly_sunny_rain:"), //cloud+sun+rain
        ("11", ":zap:"), //cloud+thunder
        ("13", ":snowman:"), //snow
        ("50", ":fog:") //mist
    ].iter().cloned().collect();

    let slack_token = env::var("SLACK_API_TOKEN").expect("SLACK_API_TOKEN not set.");
    let owm_token = env::var("OWM_API_TOKEN").expect("OWM_API_TOKEN not set.");

    let loc = owm::LocationSpecifier::CityAndCountryName{city:"Paris", country:"France"};
    let weather_res = owm::get_current_weather(loc, &owm_token);
    let weather = weather_res.unwrap();
    println!("base {:?}", weather.weather[0].main);
    println!("Right now it is {}K {}%", weather.main.temp, weather.main.humidity);
    let main: &str = &*(weather.weather[0].main);
    let icon: &str = &weather.weather[0].icon[..2];

    let client = slack_api::default_client().unwrap();
    let data = object!{
        "status_text" => format!("{}, temperature: {:.2}, humidity: {}%", main, weather.main.temp - 273.15, weather.main.humidity),
        "status_emoji" => format!("{}", icons.get(icon).unwrap())
    };
    println!("icon: {}", icons.get(icon).unwrap());
    let response = slack_api::users_profile::set(&client, &slack_token, &slack_api::users_profile::SetRequest {
        profile: Some(&data.dump()),
        ..slack_api::users_profile::SetRequest::default()
    });

    if let Ok(response) = response {
        println!("ok");
    } else {
        println!("{:?}", response);
    }
}
