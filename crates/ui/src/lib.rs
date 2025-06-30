//! Terminal UI for Shellmind

use figlet_rs::FIGfont;
use ansi_term::Colour;
use std::io::{self, Write};
use std::time::Duration;
use indicatif::{ProgressBar, ProgressStyle};
use rustyline::Editor;
use rustyline::error::ReadlineError;
use rustyline::completion::{Completer, Pair};
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context, Result as RLResult};
use rustyline::line_buffer::LineBuffer;
use std::borrow::Cow::{self, Owned};
use rustyline::history::DefaultHistory;

use std::path::Path;

// Custom completer for rustyline
struct ShellmindCompleter;

impl Completer for ShellmindCompleter {
    type Candidate = Pair;

    fn complete(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> RLResult<(usize, Vec<Pair>)> {
        let path = Path::new(line);
        let mut completions = Vec::new();

        if let Some(parent) = path.parent() {
            if let Ok(entries) = std::fs::read_dir(parent) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let file_name = entry.file_name();
                        let file_name_str = file_name.to_string_lossy();
                        if file_name_str.starts_with(&line[path.parent().unwrap_or(Path::new("")).to_string_lossy().len()..]) {
                            completions.push(Pair {
                                display: file_name_str.to_string(),
                                replacement: file_name_str.to_string(),
                            });
                        }
                    }
                }
            }
        }

        Ok((pos, completions))
    }
}

impl Highlighter for ShellmindCompleter {}
impl Hinter for ShellmindCompleter {
    type Hint = String;

    fn hint(&self, _line: &str, _pos: usize, _ctx: &Context<'_>) -> Option<String> {
        None
    }
}
impl Validator for ShellmindCompleter {}

impl rustyline::Helper for ShellmindCompleter {}

pub struct ThemeManager {
    // Placeholder for theme settings
}

impl ThemeManager {
    pub fn new() -> Self {
        ThemeManager {}
    }

    pub fn get_banner_color(&self) -> Colour {
        Colour::Cyan
    }

    pub fn get_error_color(&self) -> Colour {
        Colour::Red
    }

    pub fn get_prompt_color(&self) -> Colour {
        Colour::Green
    }

    pub fn get_command_color(&self) -> Colour {
        Colour::Yellow
    }

    pub fn get_status_color(&self) -> Colour {
        Colour::Blue
    }

    pub fn get_spinner_color(&self) -> Colour {
        Colour::Green
    }
}

pub struct CLIInterface {
    theme_manager: ThemeManager,
    editor: Editor<ShellmindCompleter, DefaultHistory>,
}

impl CLIInterface {
    pub fn new() -> Result<Self, ReadlineError> {
        let editor = Editor::new()?;
        Ok(CLIInterface {
            theme_manager: ThemeManager::new(),
            editor,
        })
    }

    pub fn print_banner(&self) {
        let standard_font = FIGfont::standard().unwrap();
        let figure = standard_font.convert("Shellmind");
        if let Some(ref fig) = figure {
            println!("{}", self.theme_manager.get_banner_color().paint(fig.to_string()));
        }
    }

    pub fn print_error(&self, message: &str) {
        eprintln!("{}", self.theme_manager.get_error_color().paint(format!("Error: {}", message)));
    }

    pub fn read_user_input(&mut self) -> Result<String, ReadlineError> {
        let p = format!("{}", self.theme_manager.get_prompt_color().paint("> "));
        let readline = self.editor.readline_with_initial(&p, ("", ""));
        match readline {
            Ok(line) => {
                self.editor.add_history_entry(line.as_str());
                Ok(line)
            },
            Err(err) => Err(err),
        }
    }

    pub fn print_command(&self, command: &str) {
        println!("{}", self.theme_manager.get_command_color().paint(command.trim()));
    }

    pub fn print_status(&self, message: &str) {
        println!("{}", self.theme_manager.get_status_color().paint(format!("Status: {}", message)));
    }

    pub fn start_thinking_indicator(&self) -> ProgressBar {
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

    pub fn stop_thinking_indicator(&self, spinner: ProgressBar) {
        spinner.finish_and_clear();
    }
}