extern crate reqwest;
extern crate serde;
extern crate serde_json;

use std::env;
use std::io::Read;
use serde_json::{Value, Error};

fn main() {
    let args: Vec<String> = env::args().collect();
    let api_key = env::var("WUG_API_KEY").unwrap();
    let zip: String;

    if args.len() == 2 {
        zip = args[1].clone();
    } else {
        zip = String::from("67206");
    }

    let json = get_temp(&zip, &api_key).unwrap();
    let ref feels_like = json["current_observation"]["feelslike_string"].as_str().unwrap();
    let ref weather = json["current_observation"]["weather"].as_str().unwrap().to_lowercase();
    println!("Weather for {} is currently {} and feels like {}", zip, weather, feels_like);
}

fn get_temp(zip: &str, api_key: &str) -> Result<Value, Error> {
    let url = format!("http://api.wunderground.com/api/{}/conditions/q/{}.json", api_key, zip);
    let mut resp = reqwest::get(&url).unwrap();
    assert!(resp.status().is_success());

    let mut content = String::new();
    let _ = resp.read_to_string(&mut content).unwrap();

    serde_json::from_str(&content)
}