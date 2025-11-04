use anyhow::{Context, Result};
use crossterm::{
    cursor,
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::CrosstermBackend as Backend;

pub type Frame<'a> = ratatui::Frame<'a>;

pub struct Tui {
    pub terminal: ratatui::Terminal<Backend<std::io::Stdout>>,
}

impl Tui {
    pub fn new() -> Result<Self> {
        let terminal = ratatui::Terminal::new(Backend::new(std::io::stdout()))
            .context("unable to create terminal")?;
        Ok(Self { terminal })
    }

    pub fn enter(&mut self) -> Result<()> {
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(
            std::io::stdout(),
            EnterAlternateScreen,
            EnableMouseCapture,
            cursor::Hide
        )?;
        self.terminal.clear()?;
        Ok(())
    }

    pub fn exit(&mut self) -> Result<()> {
        self.terminal.clear()?;
        crossterm::execute!(
            std::io::stdout(),
            LeaveAlternateScreen,
            DisableMouseCapture,
            cursor::Show
        )?;
        crossterm::terminal::disable_raw_mode()?;
        Ok(())
    }

    pub fn suspend(&mut self) -> Result<()> {
        self.exit()?;
        #[cfg(windows)]
        signal_hook::low_level::raise(signal_hook::consts::signal::SIGABRT)?;
        #[cfg(not(windows))]
        signal_hook::low_level::raise(signal_hook::consts::signal::SIGSTOP)?;
        Ok(())
    }

    pub fn resume(&mut self) -> Result<()> {
        self.enter()?;
        Ok(())
    }
}
