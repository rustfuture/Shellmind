//! Terminal UI for Shellmind

use figlet_rs::FIGfont;

pub fn print_banner() {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("Shellmind");
    if let Some(ref fig) = figure {
        println!("{}", fig);
    }
    // TODO: Add color and theme support with crossterm/ansi_term
}

// TODO: Add color, font, and theme management using crossterm and ansi_term

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_banner() {
        print_banner();
    }
}
