use serde::Deserialize;

use crate::config::get_config_data;

#[derive(Deserialize, Debug)]
pub struct Timings {
    #[serde(rename = "Fajr")]
    pub fajr: String,

    #[serde(rename = "Dhuhr")]
    pub dhuhr: String,

    #[serde(rename = "Asr")]
    pub asr: String,

    #[serde(rename = "Maghrib")]
    pub maghrib: String,

    #[serde(rename = "Isha")]
    pub isha: String,
}

#[derive(Deserialize, Debug)]
struct Data {
    timings: Timings,
}

#[derive(Deserialize, Debug)]
struct ApiResponse {
    data: Data,
}

#[derive(Deserialize, Debug)]
struct DuaData {
    arabic: String,
}

#[derive(Deserialize, Debug)]
struct DuaResponse {
    data: DuaData,
}

pub async fn get_data() -> Result<Timings, Box<dyn std::error::Error>> {
    let city = get_config_data("city".to_string())?;
    let country = get_config_data("country".to_string())?;
    
    let url = format!(
        "https://api.aladhan.com/v1/timingsByCity/16-11-2026?city={}&country={}",
        city, country
    );

    let response = reqwest::get(url).await?;

    let result: ApiResponse = response.json().await?;

    Ok(result.data.timings)
}

pub async fn get_dua() -> Result<String, Box<dyn std::error::Error>> {
    let url = "https://ummahapi.com/api/duas/random";

    let response = reqwest::get(url).await?;

    let result: DuaResponse = response.json().await?;

    Ok(result.data.arabic)
}

