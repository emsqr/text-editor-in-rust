use crossterm::event::{Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers, read};
mod terminal;
use std::io::Error;
use terminal::Terminal;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn draw_rows() -> Result<(), Error> {
        let (_, rows) = Terminal::size()?;

        for curr_row in 0..rows {
            // Clearing the row before drawing
            Terminal::clear_line()?;
            Terminal::print("~")?;
            if curr_row + 1 < rows {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => (),
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
        } else {
            Self::draw_rows()?;
            Self::draw_welcome_message()?;
            Terminal::move_cursor_to(0, 0)?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }

    fn draw_welcome_message() -> Result<(), Error> {
        let (cols, rows) = Terminal::size()?;
        let mut msg = format!("{NAME} -- {VERSION}");
        let msg_len = msg.len() as u16;
        msg.truncate(rows as usize);
        // can't directly substract like this (cols - msg_len)
        // if msg_len is greater than cols, then it will throw error 
        // - attempt to substract with overflow
        Terminal::move_cursor_to((cols.saturating_sub(msg_len)) / 2, rows / 2)?;
        Terminal::print(msg)?;
        Ok(())
    }
}
