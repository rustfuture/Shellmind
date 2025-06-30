use core::{generate_command_rest, generate_command_grpc, get_system_prompt_text, GeminiContent, ShellmindConfig, ShellmindError};
use std::io::{self};
use ui::{print_banner, print_error, print_user_prompt, print_command, start_thinking_indicator, stop_thinking_indicator};
use cli::Cli;

#[tokio::main]
async fn main() -> Result<(), ShellmindError> {
    // Load .env file
    dotenv::dotenv().ok();

    // Load configuration
    let config = ShellmindConfig::load()?;

    // Check for CLI arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        // If arguments are present, pass them to the CLI crate and exit
        Cli::run(args).await?;
        return Ok(());
    }

    // Show banner
    print_banner();

    println!("Shellmind is initialized. Type 'exit' to quit.");

    // Initialize conversation history with the system prompt
    let mut history = vec![
        GeminiContent {
            role: "user".to_string(),
            parts: vec![core::GeminiPart {
                text: get_system_prompt_text(&config),
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
        print_user_prompt();

        let input_result = tokio::task::spawn_blocking(|| {
            let mut input = String::new();
            io::stdin().read_line(&mut input).map(|_| input)
        })
        .await
        .map_err(|e| ShellmindError::Other(e.to_string()))?;
        
        let input = input_result.map_err(|e| ShellmindError::Other(e.to_string()))?;
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        if input.is_empty() {
            continue;
        }

        let indicator = start_thinking_indicator();
        ui::print_status("Generating command...");
        // Generate the command by calling the core library
        let result = match config.api_type {
            core::ApiType::Rest => generate_command_rest(&config, input, &history).await,
            core::ApiType::Grpc => generate_command_grpc(&config, input, &history).await,
        };
        stop_thinking_indicator(indicator);
        ui::print_status("Command generation complete.");

        match result {
            Ok(command) => {
                // Print the generated command for the user
                print_command(&command);

                // Add the user's query to the history
                history.push(GeminiContent {
                    role: "user".to_string(),
                    parts: vec![core::GeminiPart {
                        text: input.to_string(),
                    }],
                });
                
                history.push(GeminiContent {
                    role: "model".to_string(),
                    parts: vec![core::GeminiPart { text: command }],
                });
            }
            Err(e) => {
                print_error(&format!("Error generating command: {}", e));
            }
        }
    }

    println!("Shutting down Shellmind.");

    Ok(())
}