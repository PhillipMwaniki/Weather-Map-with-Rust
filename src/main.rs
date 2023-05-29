use reqwest::Url;
use structopt::StructOpt;
use exitfailure::{ExitFailure};
use serde_derive::{Deserialize, Serialize};
use dotenv::dotenv;
use std::env;

#[derive(StructOpt)]
struct Cli {
    city: String,
    country_code: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Forecast {
    coord: Coord,
    weather: Weather,
    base: String,
    main: Temps,
    visibility: i32,
    wind: Wind,
    clouds: Clouds,
    dt: i32,
    sys: Sys,
    timezone: i32,
    id: i32,
    name: String,
    cod: i32
}

#[derive(Serialize, Deserialize, Debug)]
struct Coord {
    lon: f64,
    lat: f64
}

#[derive(Serialize, Deserialize, Debug)]
struct Weather {
    details: Details
}

#[derive(Serialize, Deserialize, Debug)]
struct Details {
    id: i32,
    main: String,
    description: String,
    icon: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Temps {
    temp: f64,
    feels_like: f64,
    // temp_main: f64,
    temp_max: f64,
    pressure: i32,
    humidity: i32
}

#[derive(Serialize, Deserialize, Debug)]
struct Wind {
    speed: f64,
    deg: i32
}

#[derive(Serialize, Deserialize, Debug)]
struct Clouds {
    all: i32
}

#[derive(Serialize, Deserialize, Debug)]
struct Sys {
    r#type: f64,
    id: i32,
    country: String,
    sunrise: i32,
    sunset: i32
}

impl Forecast {
    async fn get(city: &String, country_code: &String) -> Result<Self, ExitFailure> {
        let url = format!("https://api.openweathermap.org/data/2.5/weather?q={},{}&appid={:?}
", city, country_code, env::var("WEATHER_MAP_API_KEY"));
        let url = Url::parse(&*url)?;

        let response = reqwest::get(url)
                            .await?.json::<Forecast>()
                            .await?;

        Ok(response)
    }
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure>{
    dotenv().ok();
    let args = Cli::from_args();
    println!("Your city is {} and your country code is {}", args.city, args.country_code);

    let response = Forecast::get(&args.city, &args.country_code).await?;

    println!("Current temperature on location is {}", response.main.temp);

    Ok(())
}
