//! Core AI logic, Gemini API, config management for Shellmind

use anyhow::Result;
use config as config_rs;
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;
use tonic::transport::Channel;
use http::uri;

pub mod google {
    pub mod generativelanguage {
        pub mod v1beta {
            tonic::include_proto!("google.generativelanguage.v1beta");
        }
    }
}

use google::generativelanguage::v1beta::generative_service_client::GenerativeServiceClient;
use google::generativelanguage::v1beta::{GenerateContentRequest, Content, Part, GenerationConfig};

#[derive(Error, Debug)]
pub enum ShellmindError {
    #[error("Configuration error: {0}")]
    Config(#[from] config_rs::ConfigError),
    #[error("API error: {0}")]
    Api(#[from] reqwest::Error),
    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("gRPC error: {0}")]
    Grpc(#[from] tonic::Status),
    #[error("gRPC transport error: {0}")]
    GrpcTransport(#[from] tonic::transport::Error),
    #[error("Invalid URI: {0}")]
    InvalidUri(#[from] uri::InvalidUri),
    #[error("Other error: {0}")]
    Other(String),
}

impl From<anyhow::Error> for ShellmindError {
    fn from(err: anyhow::Error) -> Self {
        ShellmindError::Other(err.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShellmindConfig {
    pub api_key: String,
    pub model_name: String,
    pub temperature: f32,
    pub context_window_size: usize,
    pub api_type: ApiType,
    pub grpc_endpoint: String,
    pub system_prompt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApiType {
    Rest,
    Grpc,
}

impl Default for ApiType {
    fn default() -> Self {
        ApiType::Rest
    }
}

impl ShellmindConfig {
    pub fn load() -> Result<Self, ShellmindError> {
        let api_key_from_env = std::env::var("GEMINI_API_KEY").unwrap_or_default();

        let settings = config_rs::Config::builder()
            // Set default values
            .set_default("api_key", api_key_from_env)?
            .set_default("model_name", "gemini-1.5-flash")?
            .set_default("temperature", 0.2)?
            .set_default("context_window_size", 8)?
            .set_default("api_type", "Rest")?
            .set_default("grpc_endpoint", "https://generativelanguage.googleapis.com")?
            .set_default("system_prompt", "You are Shellmind, a helpful AI assistant that translates natural language into shell commands. You are running on a Linux system.")?
            // Load config file if it exists
            .add_source(
                config_rs::File::with_name(&format!(
                    "{}/.shellmind/config.toml",
                    std::env::var("HOME").unwrap_or(".".to_string())
                ))
                .required(false),
            )
            // Load environment variables with SHELLMIND_ prefix
            .add_source(config_rs::Environment::with_prefix("SHELLMIND").separator("_"))
            .build().map_err(ShellmindError::Config)?;

        let config: ShellmindConfig = settings.try_deserialize().map_err(ShellmindError::Config)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<(), ShellmindError> {
        let home_dir = std::env::var("HOME").unwrap_or(".".to_string());
        let config_dir = format!("{}/.shellmind", home_dir);
        let config_path = format!("{}/config.toml", config_dir);

        
        std::fs::create_dir_all(&config_dir)
            .map_err(|e| ShellmindError::Other(format!("Failed to create config directory: {}", e)))?;

        let toml_string = toml::to_string(self)
            .map_err(|e| ShellmindError::Other(format!("Failed to serialize config to TOML: {}", e)))?;

        std::fs::write(&config_path, toml_string)
            .map_err(|e| ShellmindError::Other(format!("Failed to write config file: {}", e)))?;

        Ok(())
    }
}

// Gemini API structs (for REST and shared types)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiPart {
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiContent {
    pub role: String,
    pub parts: Vec<GeminiPart>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiRequest {
    pub contents: Vec<GeminiContent>,
    pub generation_config: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiResponse {
    pub candidates: Vec<Candidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candidate {
    pub content: GeminiContent,
}


pub fn get_system_prompt_text(config: &ShellmindConfig) -> String {
    config.system_prompt.clone()
}

pub async fn generate_command_rest(
    config: &ShellmindConfig,
    user_prompt: &str,
    history: &[GeminiContent],
) -> Result<String, ShellmindError> {
    let api_url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        config.model_name,
        config.api_key
    );

    let mut contents = history.to_vec();
    contents.push(GeminiContent {
        role: "user".to_string(),
        parts: vec![GeminiPart { text: user_prompt.to_string() }],
    });

    let req = GeminiRequest {
        contents,
        generation_config: Some(json!({
            "temperature": config.temperature,
        })),
    };

    let client = reqwest::Client::new();
    let resp = client.post(&api_url).json(&req).send().await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let error_body = resp.text().await?;
        return Err(ShellmindError::Other(format!(
            "API request failed with status: {} - {}",
            status,
            error_body
        )));
    }

    let resp_json: GeminiResponse = resp.json().await?;

    let command = resp_json
        .candidates
        .get(0)
        .and_then(|c| c.content.parts.get(0))
        .map(|p| p.text.clone())
        .unwrap_or_else(|| "No command generated".to_string());

    Ok(command)
}

pub async fn generate_command_grpc(
    config: &ShellmindConfig,
    user_prompt: &str,
    history: &[GeminiContent],
) -> Result<String, ShellmindError> {
    let channel = Channel::from_shared(config.grpc_endpoint.clone())?.connect().await?;
    let mut client = GenerativeServiceClient::new(channel);

    let mut contents_grpc: Vec<Content> = history.iter().map(|c| {
        Content {
            role: c.role.clone(),
            parts: c.parts.iter().map(|p| Part { text: p.text.clone() }).collect(),
        }
    }).collect();

    contents_grpc.push(Content {
        role: "user".to_string(),
        parts: vec![Part { text: user_prompt.to_string() }],
    });

    let request = tonic::Request::new(GenerateContentRequest {
        model: format!("models/{}", config.model_name),
        contents: contents_grpc,
        generation_config: Some(GenerationConfig {
            temperature: config.temperature,
        }),
    });

    let response = client.generate_content(request).await?.into_inner();

    let command = response
        .candidates
        .get(0)
        .and_then(|c| c.content.as_ref())
        .and_then(|c| c.parts.get(0))
        .map(|p| p.text.clone())
        .unwrap_or_else(|| "No command generated".to_string());

    Ok(command)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_config_load() {
        let _ = ShellmindConfig {
            api_key: "test".to_string(),
            model_name: "gemini-pro".to_string(),
            temperature: 0.2,
            context_window_size: 8,
            api_type: ApiType::Rest,
            grpc_endpoint: "https://generativelanguage.googleapis.com".to_string(),
        };
    }
}
