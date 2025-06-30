//! Core AI logic, Gemini API, config management for Shellmind

use anyhow::Result;
use config as config_rs;
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;
use tonic::transport::Channel;
use http::uri;

pub mod tools;

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

impl From<dialoguer::Error> for ShellmindError {
    fn from(err: dialoguer::Error) -> Self {
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
    pub allowed_commands: Vec<String>,
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

pub struct ConfigManager;

impl ConfigManager {
    pub fn load_configuration() -> Result<ShellmindConfig, ShellmindError> {
        let api_key_from_env = std::env::var("GEMINI_API_KEY").unwrap_or_default();

        let settings = config_rs::Config::builder()
            // Set default values
            .set_default("api_key", api_key_from_env)?
            .set_default("model_name", "gemini-1.5-flash")?
            .set_default("temperature", 0.2)?
            .set_default("context_window_size", 8)?
            .set_default("api_type", "Rest")?
            .set_default("grpc_endpoint", "https://generativelanguage.googleapis.com")?
            .set_default("system_prompt", "You are Shellmind, an advanced, proactive AI assistant integrated into a Linux terminal. Your primary goal is to understand user requests and directly assist by performing tasks, providing information, or generating and executing appropriate shell commands. You should act as an intelligent agent, anticipating user needs and offering complete solutions. If a task can be directly performed (e.g., file operations, simple data processing), do so. If a command is required, generate it and explain its purpose concisely. Always prioritize direct action and helpfulness over merely translating requests into commands. Maintain context from previous interactions. Be concise, efficient, and user-centric. You should also be able to understand and respond to commands in Turkish.")?
            .set_default("allowed_commands", Vec::<String>::new())?
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

    pub fn save_configuration(config: &ShellmindConfig) -> Result<(), ShellmindError> {
        let home_dir = std::env::var("HOME").unwrap_or(".".to_string());
        let config_dir = format!("{}/.shellmind", home_dir);
        let config_path = format!("{}/config.toml", config_dir);

        
        std::fs::create_dir_all(&config_dir)
            .map_err(|e| ShellmindError::Other(format!("Failed to create config directory: {}", e)))?;

        let toml_string = toml::to_string(config)
            .map_err(|e| ShellmindError::Other(format!("Failed to serialize config to TOML: {}", e)))?;

        std::fs::write(&config_path, toml_string)
            .map_err(|e| ShellmindError::Other(format!("Failed to write config file: {}", e)))?;

        Ok(())
    }

    pub fn add_allowed_command(config: &mut ShellmindConfig, command: &str) {
        if !config.allowed_commands.contains(&command.to_string()) {
            config.allowed_commands.push(command.to_string());
        }
    }

    pub fn validate_configuration(config: &ShellmindConfig) -> Result<(), ShellmindError> {
        if config.api_key.is_empty() {
            return Err(ShellmindError::Other("API Key is not set. Please set it using the config command or GEMINI_API_KEY environment variable.".to_string()));
        }
        Ok(())
    }
}

pub struct SandboxManager;

impl SandboxManager {
    pub async fn create_sandbox(sandbox_type: &str) -> Result<String, ShellmindError> {
        // Placeholder for actual sandbox creation logic
        Ok(format!("Sandbox created: {}", sandbox_type))
    }

    pub async fn execute_safely(command: &str, sandbox_id: &str) -> Result<String, ShellmindError> {
        // Placeholder for safe command execution within sandbox
        Ok(format!("Command '{}' executed safely in sandbox '{}'.", command, sandbox_id))
    }

    pub fn validate_operation(operation: &str) -> Result<(), ShellmindError> {
        // Placeholder for security assessment
        Ok(())
    }
}

pub struct SecurityManager;

impl SecurityManager {
    pub fn assess_tool_safety(tool_name: &str, params: &serde_json::Value) -> SafetyLevel {
        // Placeholder for tool safety assessment
        SafetyLevel::Safe
    }

    pub fn requires_confirmation(operation: &str) -> bool {
        // Placeholder for confirmation logic
        true
    }

    pub fn sanitize_input(input: &str) -> String {
        // Placeholder for input sanitization
        input.to_string()
    }
}

pub enum SafetyLevel {
    Safe,
    Warning,
    Dangerous,
}

pub struct MemoryManager {
    context_files: std::collections::HashMap<String, String>,
    runtime_memory: Vec<String>,
}

impl MemoryManager {
    pub fn new() -> Self {
        MemoryManager {
            context_files: std::collections::HashMap::new(),
            runtime_memory: Vec::new(),
        }
    }

    pub async fn load_hierarchical_context(&mut self) -> Result<(), ShellmindError> {
        // Placeholder for loading context from files (e.g., GEMINI.md)
        // For now, just simulate loading.
        self.context_files.insert("global".to_string(), "Global context loaded.".to_string());
        Ok(())
    }

    pub async fn refresh_context(&mut self) -> Result<(), ShellmindError> {
        // Placeholder for refreshing context (e.g., re-reading files)
        self.load_hierarchical_context().await
    }

    pub fn add_runtime_memory(&mut self, content: String) {
        self.runtime_memory.push(content);
    }

    pub fn get_full_context(&self) -> String {
        let mut full_context = String::new();
        for (name, content) in &self.context_files {
            full_context.push_str(&format!("--- {} Context ---
{}
", name, content));
        }
        if !self.runtime_memory.is_empty() {
            full_context.push_str("--- Runtime Memory ---
");
            full_context.push_str(&self.runtime_memory.join("\n"));
            full_context.push_str("\n");
        }
        full_context
    }
}

pub struct CommandHistoryManager {
    history_file_path: std::path::PathBuf,
    history: Vec<String>,
}

impl CommandHistoryManager {
    pub fn new() -> Result<Self, ShellmindError> {
        let home_dir = dirs::home_dir().ok_or_else(|| ShellmindError::Other("Could not find home directory.".to_string()))?;
        let history_dir = home_dir.join(".shellmind");
        let history_file_path = history_dir.join("history.txt");

        std::fs::create_dir_all(&history_dir)
            .map_err(|e| ShellmindError::Other(format!("Failed to create history directory: {}", e)))?;

        let history = if history_file_path.exists() {
            std::fs::read_to_string(&history_file_path)
                .map_err(|e| ShellmindError::Other(format!("Failed to read history file: {}", e)))?
                .lines()
                .map(|s| s.to_string())
                .collect()
        } else {
            Vec::new()
        };

        Ok(Self { history_file_path, history })
    }

    pub fn add_command(&mut self, command: &str) -> Result<(), ShellmindError> {
        self.history.push(command.to_string());
        self.save_history()
    }

    pub fn get_history(&self) -> &[String] {
        &self.history
    }

    fn save_history(&self) -> Result<(), ShellmindError> {
        let content = self.history.join("\n");
        std::fs::write(&self.history_file_path, content)
            .map_err(|e| ShellmindError::Other(format!("Failed to write history file: {}", e)))?;
        Ok(())
    }
}

use std::pin::Pin;

use std::future::Future;

pub trait BaseTool: Send + Sync {
    fn name(&self) -> &'static str;
    fn display_name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn parameter_schema(&self) -> serde_json::Value;
    fn validate_tool_params(&self, params: &serde_json::Value) -> bool;
    fn get_description(&self, params: &serde_json::Value) -> String;
    fn should_confirm_execute(&self, params: &serde_json::Value) -> Option<ConfirmationDetails>;
    fn execute(&self, params: serde_json::Value, signal: Option<tokio::signal::unix::Signal>) -> Pin<Box<dyn Future<Output = Result<ToolResult, ShellmindError>> + Send>>;
}

pub struct ConfirmationDetails {
    pub message: String,
}

pub enum ToolResult {
    Success(String),
    Error(String),
}

pub struct ToolRegistry {
    tools: std::collections::HashMap<String, Box<dyn BaseTool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        ToolRegistry {
            tools: std::collections::HashMap::new(),
        }
    }

    pub fn register<T: BaseTool + 'static>(&mut self, tool: T) {
        self.tools.insert(tool.name().to_string(), Box::new(tool));
    }

    pub async fn discover_tools(&mut self) -> Result<(), ShellmindError> {
        // Placeholder for tool discovery logic (e.g., from MCP servers, plugins)
        Ok(())
    }

    pub fn get_tool_schemas(&self) -> Vec<serde_json::Value> {
        self.tools.values().map(|tool| tool.parameter_schema()).collect()
    }

    pub fn get_tool(&self, name: &str) -> Option<&dyn BaseTool> {
        self.tools.get(name).map(|b| &**b)
    }
}

// Gemini API structs (for REST and shared types)

impl ShellmindConfig {
    pub fn save(&self) -> Result<(), ShellmindError> {
        ConfigManager::save_configuration(self)
    }

    pub fn add_allowed_command(&mut self, command: &str) {
        ConfigManager::add_allowed_command(self, command)
    }
}
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
