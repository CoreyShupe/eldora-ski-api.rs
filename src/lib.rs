pub mod alerts;
#[cfg(feature = "display")]
pub mod display;
pub mod lifts;
pub mod snow_alerts;
pub mod trails;
pub mod weather_forecast;

use crate::alerts::GetAlertsResponse;
use crate::lifts::GetLiftsResponse;
use crate::snow_alerts::GetSnowAlertsResponse;
use crate::trails::GetTrailsResponse;
use crate::weather_forecast::GetWeatherForecastResponse;
use serde::Deserialize;
use std::time::Duration;

const ELDORA_SKI_API_URL: &str = "https://api.eldora.com/api/v1/dor";

pub trait EldoraSkiApi {
    fn get_trails(&self) -> impl Future<Output = GetTrailsResponse>;

    fn get_lifts(&self) -> impl Future<Output = GetLiftsResponse>;

    fn get_alerts(&self) -> impl Future<Output = GetAlertsResponse>;

    fn get_snow_alerts(&self) -> impl Future<Output = GetSnowAlertsResponse>;

    fn get_weather_forecast(&self) -> impl Future<Output = GetWeatherForecastResponse>;
}

pub struct EldoraSkiApiClient {
    reqwest_client: reqwest::Client,
}

impl EldoraSkiApiClient {
    pub fn create() -> Self {
        let client = reqwest::ClientBuilder::new()
            .connect_timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to build client.");

        Self {
            reqwest_client: client,
        }
    }

    async fn expect_http_get<T: for<'de> Deserialize<'de>>(&self, api_url: &str) -> T {
        self.reqwest_client
            .get(format!("{}{}", ELDORA_SKI_API_URL, api_url))
            .send()
            .await
            .expect("Failed to get trails")
            .json()
            .await
            .expect("Failed to parse json")
    }
}

impl EldoraSkiApi for EldoraSkiApiClient {
    async fn get_trails(&self) -> GetTrailsResponse {
        // /drupal/trails
        self.expect_http_get("/drupal/trails").await
    }

    async fn get_lifts(&self) -> GetLiftsResponse {
        // /drupal/lifts
        self.expect_http_get("/drupal/lifts").await
    }

    async fn get_alerts(&self) -> GetAlertsResponse {
        // /drupal/alerts
        self.expect_http_get("/drupal/alerts").await
    }

    async fn get_snow_alerts(&self) -> GetSnowAlertsResponse {
        // /drupal/snow-reports
        self.expect_http_get("/drupal/snow-reports").await
    }

    async fn get_weather_forecast(&self) -> GetWeatherForecastResponse {
        // /weather-forecast
        self.expect_http_get("/weather-forecast").await
    }
}
