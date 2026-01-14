use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::execute;
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size};
use std::io::{Error, stdout};

pub struct Terminal {}

impl Terminal {
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(0, 0)?;
        Ok(())
    }

    pub fn hide_cursor() -> Result<(), Error> {
        execute!(stdout(), Hide)?;
        Ok(())
    }

    pub fn show_cursor() -> Result<(), Error> {
        execute!(stdout(), Show)?;
        Ok(())
    }

    pub fn print(string: &str) -> Result<(), Error> {
        execute!(stdout(), Print(string))?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), Error> {
        execute!(stdout(), Clear(ClearType::All))?;
        Ok(())
    }

    pub fn terminate() -> Result<(), Error> {
        disable_raw_mode()?;
        Ok(())
    }

    pub fn move_cursor_to(x: u16, y: u16) -> Result<(), Error> {
        execute!(stdout(), MoveTo(x, y))?;
        Ok(())
    }

    pub fn size() -> Result<(u16, u16), Error> {
        size()
    }
}
