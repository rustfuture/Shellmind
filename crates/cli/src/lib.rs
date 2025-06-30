use clap::{Parser, Subcommand};
use anyhow::Result;
use core::{ShellmindError, ShellmindConfig, generate_command_rest, generate_command_grpc};
use ui::CLIInterface;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Displays the version of the CLI
    Version,
    /// Manage Shellmind configuration
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },
    /// Send a prompt to the AI and get a command
    Prompt {
        /// The prompt to send to the AI
        #[arg(short, long)]
        text: String,
    },
}

#[derive(Subcommand, Debug)]
enum ConfigCommands {
    /// Show current configuration
    Show,
    /// Set a configuration value
    Set {
        /// The configuration key to set (e.g., api_key, model_name, temperature, api_type, grpc_endpoint, system_prompt)
        key: String,
        /// The value to set
        value: String,
    },
}

impl Cli {
    pub async fn run(args: Vec<String>, ui: &CLIInterface) -> Result<(), ShellmindError> {
        let cli = Cli::parse_from(args);

        match &cli.command {
            Commands::Version => {
                println!("Shellmind CLI Version: {}", env!("CARGO_PKG_VERSION"));
            }
            Commands::Config { command } => match command {
                ConfigCommands::Show => {
                    let config = core::ConfigManager::load_configuration()?;
                    println!("Current Shellmind Configuration:");
                    println!("  API Key: {}", if config.api_key.is_empty() { "Not set" } else { "********" });
                    println!("  Model Name: {}", config.model_name);
                    println!("  Temperature: {}", config.temperature);
                    println!("  Context Window Size: {}", config.context_window_size);
                    println!("  API Type: {:?}", config.api_type);
                    println!("  gRPC Endpoint: {}", config.grpc_endpoint);
                    println!("  System Prompt: {}", config.system_prompt);
                }
                ConfigCommands::Set { key, value } => {
                    let mut config = core::ConfigManager::load_configuration()?;
                    match key.as_str() {
                        "api_key" => config.api_key = value.clone(),
                        "model_name" => config.model_name = value.clone(),
                        "temperature" => {
                            config.temperature = value.parse().map_err(|_| ShellmindError::Other("Invalid temperature value".to_string()))?;
                        }
                        "context_window_size" => {
                            config.context_window_size = value.parse().map_err(|_| ShellmindError::Other("Invalid context window size value".to_string()))?;
                        }
                        "api_type" => {
                            config.api_type = match value.to_lowercase().as_str() {
                                "rest" => core::ApiType::Rest,
                                "grpc" => core::ApiType::Grpc,
                                _ => return Err(ShellmindError::Other("Invalid API type. Use 'rest' or 'grpc'".to_string())),
                            };
                        }
                        "grpc_endpoint" => config.grpc_endpoint = value.clone(),
                        "system_prompt" => config.system_prompt = value.clone(),
                        _ => return Err(ShellmindError::Other(format!("Unknown config key: {}", key))),
                    }
                    core::ConfigManager::save_configuration(&config)?;
                    println!("Configuration updated successfully.");
                }
            },
            Commands::Prompt { text } => {
                let config = core::ConfigManager::load_configuration()?;
                let indicator = ui.start_thinking_indicator();
                ui.print_status("Generating command...");
                let result = match config.api_type {
                    core::ApiType::Rest => generate_command_rest(&config, text, &[]).await,
                    core::ApiType::Grpc => generate_command_grpc(&config, text, &[]).await,
                };
                ui.stop_thinking_indicator(indicator);
                ui.print_status("Command generation complete.");

                match result {
                    Ok(command) => {
                        ui.print_command(&command);
                    }
                    Err(e) => {
                        ui.print_error(&format!("Error generating command: {}", e));
                    }
                }
            }
        }
        Ok(())
    }
}
