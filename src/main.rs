mod weather;
use weather::facade;
use weather::types::{WeatherRequest, WeatherResponse};

use rmcp::{
    Json, ServiceExt,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    tool, tool_handler, tool_router,
    transport::stdio,
};

#[derive(Clone)]
pub struct StructuredOutputServer {
    tool_router: ToolRouter<Self>,
}

#[tool_handler(router = self.tool_router)]
impl rmcp::ServerHandler for StructuredOutputServer {}

impl Default for StructuredOutputServer {
    fn default() -> Self {
        Self::new()
    }
}

#[tool_router(router = tool_router)]
impl StructuredOutputServer {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    /// Get server info (returns unstructured text)
    #[tool(name = "get_info", description = "Get server information")]
    pub async fn get_info(&self) -> String {
        "Structured Output Example Server v1.0".to_string()
    }

    /// Get weather information for a city (returns structured data)
    #[tool(name = "get_weather", description = "Get current weather for a city")]
    pub async fn get_weather(
        &self,
        params: Parameters<WeatherRequest>,
    ) -> Result<Json<WeatherResponse>, String> {
        let city = &params.0.city;
        if city.is_empty() {
            return Err("No city provided".to_string());
        }

        let reading = facade::fetch_for_city("de", city)
            .await
            .map_err(|e| format!("Failed to fetch weather: {e}"))?;
        
        let weather = WeatherResponse {
            temperature: reading.main.temp,
            description: reading.weather[0].description.clone(),
            humidity: reading.main.humidity,
            wind_speed: reading.wind.speed,
        };

        Ok(Json(weather))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if std::env::var("API_KEY").is_err() {
        anyhow::bail!("Environment variable API_KEY is not set");
    }

    eprintln!("Starting weather server...");
    eprintln!();
    eprintln!("Tools available:");
    eprintln!("- get_weather: Returns structured weather data");
    eprintln!("- get_info: Returns plain text");
    eprintln!();

    let server = StructuredOutputServer::new();

    // Print the tools with their schemas for demonstration
    eprintln!("Tool schemas:");
    for tool in server.tool_router.list_all() {
        eprintln!("\n{}: {}", tool.name, tool.description.unwrap_or_default());
        if let Some(output_schema) = &tool.output_schema {
            eprintln!(
                "  Output schema: {}",
                serde_json::to_string_pretty(output_schema).unwrap()
            );
        } else {
            eprintln!("  Output: Unstructured text");
        }
    }
    eprintln!();

    // Start the server
    eprintln!("Starting server. Connect with an MCP client to test the tools.");
    eprintln!("Press Ctrl+C to stop.");

    let service = server.serve(stdio()).await?;
    service.waiting().await?;

    Ok(())
}