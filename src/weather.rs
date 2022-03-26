use anyhow;
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Weather {
    nearest_area: Vec<NearestArea>,
    current_condition: Vec<CurrentCondition>,
    weather: Vec<DayWeather>,
}

#[derive(Deserialize, Serialize)]
struct CurrentCondition {
    #[serde(alias = "weatherDesc")]
    weather_desc: Vec<WeatherDesc>,
    #[serde(alias = "FeelsLikeC")]
    feels_like_c: String,
    #[serde(alias = "temp_C")]
    temp_c: String,
    #[serde(alias = "windspeedKmph")]
    windspeed_kmph: String,
    humidity: String,
}

#[derive(Deserialize, Serialize)]
struct NearestArea {
    #[serde(alias = "areaName")]
    area_name: Vec<AreaName>,
    region: Vec<Region>,
}

#[derive(Deserialize, Serialize)]
struct DayWeather {
    date: String,
    #[serde(alias = "maxtempC")]
    max_temp_c: String,
    #[serde(alias = "mintempC")]
    min_temp_c: String,
    astronomy: Vec<Astronomy>,
    hourly: Vec<HourWeather>,
}

#[derive(Deserialize, Serialize)]
struct HourWeather {
    time: String,
    #[serde(alias = "weatherCode")]
    weather_code: String,
    #[serde(alias = "tempC")]
    temp_c: String,
    #[serde(alias = "weatherDesc")]
    weather_desc: Vec<WeatherDesc>,
}

#[derive(Deserialize, Serialize)]
struct Astronomy {
    sunrise: String,
    sunset: String,
}

#[derive(Deserialize, Serialize)]
struct AreaName {
    value: String,
}

#[derive(Deserialize, Serialize)]
struct Region {
    value: String,
}

#[derive(Deserialize, Serialize)]
struct WeatherDesc {
    value: String,
}

pub async fn get_weather_by_location(latitude: f64, longitude: f64) -> anyhow::Result<Option<Weather>> {
    let response = reqwest::get(format!(
        "https://wttr.in/{},{}?format=j1",
        latitude, longitude
    ))
    .await?;

    let weather = if response.status().is_success() {
        let weather = response.json().await?;
        Some(weather)
    } else {
        None
    };

    Ok(weather)

}

pub async fn get_weather_by_name(name: &str) -> anyhow::Result<Option<Weather>> {
    let response = reqwest::get(format!(
        "https://wttr.in/{}?format=j1",
        name
    ))
    .await?;
    
    let weather = if response.status().is_success() {
        let weather = response.json().await?;
        Some(weather)
    } else {
        None
    };

    Ok(weather)
}