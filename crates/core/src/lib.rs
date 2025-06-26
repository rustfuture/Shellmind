//! Core AI logic, Gemini API, config management for Shellmind

use serde::{Deserialize, Serialize};
use anyhow::Result;
use config as config_rs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShellmindConfig {
    pub api_key: String,
    pub model_name: String,
    pub temperature: f32,
    pub context_window_size: usize,
    pub api_type: ApiType,
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
    pub fn load() -> Result<Self> {
        let mut settings = config_rs::Config::builder();
        let home = std::env::var("HOME").unwrap_or(".".to_string());
        let config_path = format!("{}/.shellmind/config.toml", home);
        settings = settings.add_source(config_rs::File::with_name(&config_path).required(false));
        settings = settings.add_source(config_rs::Environment::with_prefix("SHELLMIND").separator("_"));
        let settings = settings.build()?;
        let mut config: ShellmindConfig = settings.try_deserialize()?;
        // Fallbacks
        if config.api_key.is_empty() {
            config.api_key = std::env::var("SHELLMIND_API_KEY").unwrap_or_default();
        }
        Ok(config)
    }
}

// Gemini API structs
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
    pub candidates: Vec<GeminiContent>,
}

pub fn get_system_prompt_text() -> String {
    // TODO: Make this customizable
    "You are Shellmind, a helpful AI assistant that translates natural language into shell commands.".to_string()
}

pub async fn generate_command_rest(
    config: &ShellmindConfig,
    user_prompt: &str,
    history: &[GeminiContent],
) -> Result<String> {
    // TODO: Use config.model_name, config.temperature, etc.
    let api_url = format!("https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        config.model_name, config.api_key);
    let mut contents = history.to_vec();
    contents.push(GeminiContent {
        role: "user".to_string(),
        parts: vec![GeminiPart { text: user_prompt.to_string() }],
    });
    let req = GeminiRequest {
        contents,
        generation_config: None, // TODO: Use config
    };
    let client = reqwest::Client::new();
    let resp = client.post(&api_url)
        .json(&req)
        .send()
        .await?;
    let resp_json: serde_json::Value = resp.json().await?;
    // TODO: Parse response properly
    let command = resp_json["candidates"][0]["parts"][0]["text"].as_str().unwrap_or("").to_string();
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
        };
    }
}
