use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WeatherRequest {
    pub city: String,
    pub units: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WeatherResponse {
    pub temperature: f64,
    pub description: String,
    pub humidity: f64,
    pub wind_speed: f64,
}