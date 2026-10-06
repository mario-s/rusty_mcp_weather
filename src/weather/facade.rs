use openweathermap_client::models::{City, CurrentWeather, UnitSystem};
use openweathermap_client::{Client, ClientOptions};
use openweathermap_client::error::ClientError;

pub async fn fetch_for_city(language: &str, city: &str) -> Result<CurrentWeather, ClientError> {

    let options = ClientOptions {
        units: UnitSystem::Metric,
        language: language.to_string(),
        ..ClientOptions::default()
    };
    let client = Client::new(options)?;

    client
        .fetch_weather(&City::new(city, "DE"))
        .await
        .map_err(ClientError::from)
}