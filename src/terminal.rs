use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Stdout};

/// Manages terminal setup and cleanup for displaying animations
pub struct TerminalManager {
    stdout: Stdout,
}

impl TerminalManager {
    /// Creates a new terminal manager and sets up the terminal
    pub fn new() -> io::Result<Self> {
        let stdout = io::stdout();
        enable_raw_mode()?;
        execute!(stdout, EnterAlternateScreen)?;

        Ok(TerminalManager { stdout })
    }

    /// Clears the screen and moves cursor to top-left
    pub fn clear_screen(&mut self) -> io::Result<()> {
        execute!(
            self.stdout,
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
            crossterm::cursor::MoveTo(0, 0)
        )
    }

    /// Prints text at specific position
    pub fn print_at(&mut self, x: u16, y: u16, text: &str) -> io::Result<()> {
        execute!(
            self.stdout,
            crossterm::cursor::MoveTo(x, y),
            crossterm::style::Print(text)
        )
    }

    /// Gets stdout reference
    pub fn stdout(&mut self) -> &mut Stdout {
        &mut self.stdout
    }
}

impl Drop for TerminalManager {
    /// Restores terminal to normal state when dropped
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(self.stdout, LeaveAlternateScreen);
    }
}