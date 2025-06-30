//! Terminal UI for Shellmind

use figlet_rs::FIGfont;
use ansi_term::Colour;
use std::io::{self, Write};
use std::time::Duration;
use indicatif::{ProgressBar, ProgressStyle};

pub fn print_banner() {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("Shellmind");
    if let Some(ref fig) = figure {
        println!("{}", Colour::Cyan.paint(fig.to_string()));
    }
}

pub fn print_error(message: &str) {
    eprintln!("{}", Colour::Red.paint(format!("Error: {}", message)));
}

pub fn print_user_prompt() {
    print!("{}", Colour::Green.paint("> "));
    io::stdout().flush().unwrap();
}

pub fn print_command(command: &str) {
    println!("{}", Colour::Yellow.paint(command.trim()));
}

pub fn print_status(message: &str) {
    println!("{}", Colour::Blue.paint(format!("Status: {}", message)));
}

pub fn start_thinking_indicator() -> ProgressBar {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::with_template("{spinner:.green} {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    spinner.set_message("Thinking...");
    spinner.enable_steady_tick(Duration::from_millis(100));
    spinner
}

pub fn stop_thinking_indicator(spinner: ProgressBar) {
    spinner.finish_and_clear();
}


