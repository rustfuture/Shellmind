use core::{generate_command_rest, generate_command_grpc, get_system_prompt_text, GeminiContent, ShellmindConfig, ShellmindError, ToolRegistry, SandboxManager, SecurityManager, MemoryManager, CommandHistoryManager};
use core::tools::{ReadFileTool, WriteFileTool, EditTool, LSTool, GrepTool, GlobTool, ShellTool, WebFetchTool, WebSearchTool, MemoryTool, ReadManyFilesTool};
use std::io::{self, Write};
use ui::CLIInterface;
use cli::Cli;
use dialoguer::{Select, theme::ColorfulTheme};
use std::process::Command;
use anyhow::Result;
use rustyline::error::ReadlineError;

struct ShellmindCLI {
    config: ShellmindConfig,
    tool_registry: ToolRegistry,
    sandbox_manager: SandboxManager,
    security_manager: SecurityManager,
    memory_manager: MemoryManager,
    command_history_manager: CommandHistoryManager,
    ui: CLIInterface,
}

impl ShellmindCLI {
    async fn new() -> Result<Self> {
        // Load .env file
        dotenv::dotenv().ok();

        // Load configuration
        let config = core::ConfigManager::load_configuration()?;
        core::ConfigManager::validate_configuration(&config)?;

        let mut tool_registry = ToolRegistry::new();
        tool_registry.register(ReadFileTool);
        tool_registry.register(WriteFileTool);
        tool_registry.register(EditTool);
        tool_registry.register(LSTool);
        tool_registry.register(GrepTool);
        tool_registry.register(GlobTool);
        tool_registry.register(ShellTool);
        tool_registry.register(WebFetchTool);
        tool_registry.register(WebSearchTool);
        tool_registry.register(MemoryTool);
        tool_registry.register(ReadManyFilesTool);

        Ok(Self {
            config,
            tool_registry,
            sandbox_manager: SandboxManager,
            security_manager: SecurityManager,
            memory_manager: MemoryManager::new(),
            command_history_manager: CommandHistoryManager::new()?,
            ui: CLIInterface::new()?,
        })
    }

    async fn start(&mut self) -> Result<()> {
        // Discover tools
        self.tool_registry.discover_tools().await?;

        // Load hierarchical context
        self.memory_manager.load_hierarchical_context().await?;

        // Check for CLI arguments
        let args: Vec<String> = std::env::args().collect();
        if args.len() > 1 {
            // If arguments are present, pass them to the CLI crate and exit
            Cli::run(args, &self.ui).await?;
            return Ok(());
        }

        // Show banner
        self.ui.print_banner();

        println!("Shellmind is initialized. Type 'exit' to quit.");

        // Initialize conversation history with the system prompt
        let mut history = vec![
            GeminiContent {
                role: "user".to_string(),
                parts: vec![core::GeminiPart {
                    text: get_system_prompt_text(&self.config),
                }],
            },
            GeminiContent {
                role: "model".to_string(),
                parts: vec![core::GeminiPart {
                    text: "Okay, I'm ready. What can I help you with?".to_string(),
                }],
            },
        ];

        // Main interactive loop
        loop {
            let input = match self.ui.read_user_input() {
                Ok(line) => line,
                Err(ReadlineError::Interrupted) => {
                    println!("Ctrl-C received, exiting.");
                    break;
                },
                Err(ReadlineError::Eof) => {
                    println!("Ctrl-D received, exiting.");
                    break;
                },
                Err(err) => {
                    self.ui.print_error(&format!("Error reading input: {}", err));
                    continue;
                },
            };
            let input = input.trim();

            if input.eq_ignore_ascii_case("exit") {
                break;
            }

            if input.is_empty() {
                continue;
            }

            let indicator = self.ui.start_thinking_indicator();
            self.ui.print_status("Generating command...");
            
            let result = match self.config.api_type {
                core::ApiType::Rest => generate_command_rest(&self.config, input, &history).await,
                core::ApiType::Grpc => generate_command_grpc(&self.config, input, &history).await,
            };
            self.ui.stop_thinking_indicator(indicator);
            self.ui.print_status("Command generation complete.");

            match result {
                Ok(command) => {
                    self.ui.print_command(&command);

                    // Check if the command contains a newline, indicating it’s an informational message
                    if command.contains('\n') {
                        println!("\n{}", command); // Print the informational message
                        history.push(GeminiContent {
                            role: "user".to_string(),
                            parts: vec![core::GeminiPart {
                                text: input.to_string(),
                            }],
                        });
                        
                        history.push(GeminiContent {
                            role: "model".to_string(),
                            parts: vec![core::GeminiPart {
                                text: command
                            }],
                        });
                        continue; // Skip command execution and prompt for next input
                    }

                    // Attempt to parse as a tool call
                    let tool_call_regex = regex::Regex::new(r"^([a-zA-Z_]+)\((.*)\)$").unwrap();
                    if let Some(captures) = tool_call_regex.captures(&command) {
                        let tool_name = captures.get(1).unwrap().as_str();
                        let params_str = captures.get(2).unwrap().as_str();

                        if let Some(tool) = self.tool_registry.get_tool(tool_name) {
                            let params: serde_json::Value = serde_json::from_str(params_str).unwrap_or_else(|_| serde_json::json!({}));
                            
                            if let Some(confirmation_details) = tool.should_confirm_execute(&params) {
                                let confirmed = dialoguer::Confirm::with_theme(&ColorfulTheme::default())
                                    .with_prompt(&confirmation_details.message)
                                    .interact()?;

                                if confirmed {
                                    self.ui.print_status(&format!("Executing tool: {}", tool.display_name()));
                                    let tool_result = tool.execute(params, None).await?;
                                    match tool_result {
                                        core::ToolResult::Success(output) => self.ui.print_status(&format!("Tool output: {}", output)),
                                        core::ToolResult::Error(err) => self.ui.print_error(&format!("Tool error: {}", err)),
                                    }
                                } else {
                                    self.ui.print_status("Tool execution cancelled.");
                                }
                            } else { // No confirmation needed, execute directly
                                self.ui.print_status(&format!("Executing tool: {}", tool.display_name()));
                                let tool_result = tool.execute(params, None).await?;
                                match tool_result {
                                    core::ToolResult::Success(output) => self.ui.print_status(&format!("Tool output: {}", output)),
                                    core::ToolResult::Error(err) => self.ui.print_error(&format!("Tool error: {}", err)),
                                }
                            }
                        } else {
                            self.ui.print_error(&format!("Unknown tool: {}", tool_name));
                        }
                    } else { // Not a tool call, treat as a regular shell command
                        let options = &["Evet (Bir Kez Çalıştır)", "Her Zaman İzin Ver", "Hayır"];
                        let selection = Select::with_theme(&ColorfulTheme::default())
                            .with_prompt("Bu komutu çalıştırmak ister misiniz?")
                            .default(0)
                            .items(&options[..])
                            .interact_opt()?;

                        match selection {
                            Some(0) => { // Evet (Bir Kez Çalıştır)
                                run_command(&command)?;
                            },
                            Some(1) => { // Her Zaman İzin Ver
                                core::ConfigManager::add_allowed_command(&mut self.config, &command);
                                core::ConfigManager::save_configuration(&self.config)?;
                                run_command(&command)?;
                            },
                            _ => { // Hayır veya iptal
                                println!("Komut çalıştırılmadı.");
                            }
                        }
                    }

                    self.command_history_manager.add_command(input)?;

                    history.push(GeminiContent {
                        role: "user".to_string(),
                        parts: vec![core::GeminiPart {
                            text: input.to_string(),
                        }],
                    });
                    
                    history.push(GeminiContent {
                        role: "model".to_string(),
                        parts: vec![core::GeminiPart {
                            text: command
                        }],
                    });
                },
                Err(e) => {
                    self.ui.print_error(&format!("Error generating command: {}", e));
                }
            }
        }

        println!("Shutting down Shellmind.");

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut cli = ShellmindCLI::new().await?;
    cli.start().await
}

fn run_command(command_str: &str) -> Result<(), ShellmindError> {
    println!("Çalıştırılıyor: {}", command_str);
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", command_str])
            .output()
            .map_err(|e| ShellmindError::Other(format!("Komut çalıştırılamadı: {}", e)))?
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(command_str)
            .output()
            .map_err(|e| ShellmindError::Other(format!("Komut çalıştırılamadı: {}", e)))?
    };

    io::stdout().write_all(&output.stdout).map_err(|e| ShellmindError::Other(e.to_string()))?;
    io::stderr().write_all(&output.stderr).map_err(|e| ShellmindError::Other(e.to_string()))?;

    if !output.status.success() {
        return Err(ShellmindError::Other(format!("Komut hata koduyla çıktı: {:?}", output.status.code())));
    }
    Ok(())
}