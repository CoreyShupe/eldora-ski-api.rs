use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CurrentForecast {
    pub temperature: i8,
    pub weather_icon: String,
}

#[derive(Serialize, Deserialize)]
pub struct AvailableForecast {
    pub description_long: String,
    pub description_short: String,
    pub details: String,
    pub weather_icon: String,
    pub id: u64,
    pub date: String,
    pub day: String,
    pub temperature_high: i8,
    pub temperature_low: i8,
    pub precipitation_high: i8,
    pub precipitation_low: i8,
    pub wind_speed: i8,
    pub wind_direction: String,
}

#[derive(Serialize, Deserialize)]
pub struct SensorData {
    pub id: u64,
    pub name: String,
    #[serde(rename = "pubDate")]
    pub pub_date: String,
    #[serde(rename = "rh", skip_serializing_if = "Option::is_none")]
    pub relative_humidity: Option<i8>,
    #[serde(rename = "temp", skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
}

#[derive(Serialize, Deserialize)]
pub struct GetWeatherForecastResponse {
    pub current: CurrentForecast,
    pub forecast: Vec<AvailableForecast>,
    pub sensors: Vec<SensorData>,
}
