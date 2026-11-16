use serde::Deserialize;

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

pub async fn get_data() -> Result<Timings, Box<dyn std::error::Error>> {
    let url = "https://api.aladhan.com/v1/timingsByCity/16-11-2026?city=Casablanca&country=Morocco";

    let response = reqwest::get(url).await?;

    let result: ApiResponse = response.json().await?;

    Ok(result.data.timings)
}