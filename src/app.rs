use std::{
    cell::RefCell,
    io::{stdout, Stdout, Write},
    panic::{catch_unwind, resume_unwind, AssertUnwindSafe},
    time::Duration,
};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{
        poll, read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, MouseEvent,
    },
    queue,
    style::Print,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    Command, ExecutableCommand,
};

use crate::Result;

pub struct App {
    stdout: RefCell<Stdout>,
}

impl App {
    pub fn new() -> Self {
        Self {
            stdout: RefCell::new(stdout()),
        }
    }
}

impl App {
    fn start(&self) -> Result<()> {
        enable_raw_mode()?;

        self.stdout
            .borrow_mut()
            .execute(EnterAlternateScreen)?
            .execute(EnableMouseCapture)?
            .execute(Hide)?;

        Ok(())
    }

    fn finish(&self) -> Result<()> {
        disable_raw_mode()?;

        self.stdout
            .borrow_mut()
            .execute(LeaveAlternateScreen)?
            .execute(DisableMouseCapture)?
            .execute(Show)?;

        Ok(())
    }

    fn exec(&self, command: impl Command) -> Result<()> {
        queue!(self.stdout.borrow_mut(), command)?;

        Ok(())
    }

    pub fn clear(&self) -> Result<()> {
        self.exec(Clear(ClearType::All))?;
        self.exec(MoveTo(0, 0))?;

        Ok(())
    }

    pub fn print(&self, text: &str) -> Result<()> {
        self.exec(Print(text))?;

        Ok(())
    }

    pub fn move_to(&self, x: u16, y: u16) -> Result<()> {
        self.exec(MoveTo(x, y))?;

        Ok(())
    }

    pub fn run<F, K, M>(&self, mut func: F, mut key_events: K, mut mouse_event: M) -> Result<()>
    where
        F: FnMut() -> Result<()>,
        K: FnMut(KeyEvent) -> bool,
        M: FnMut(MouseEvent) -> bool,
    {
        self.start()?;

        let result = catch_unwind(AssertUnwindSafe(|| -> Result<()> {
            loop {
                self.clear()?;

                func()?;

                self.stdout.borrow_mut().flush()?;

                if poll(Duration::from_secs(1))? {
                    match read()? {
                        Event::Key(event) => match event.code {
                            KeyCode::Char('q') => {
                                break;
                            }
                            _ => {
                                if !key_events(event) {
                                    break;
                                }
                            }
                        },
                        Event::Mouse(event) => {
                            if !mouse_event(event) {
                                break;
                            }
                        }
                        _ => {}
                    }
                }
            }

            Ok(())
        }));

        self.finish()?;

        if let Err(err) = result {
            if let Some(message) = err.downcast_ref::<&str>() {
                eprintln!("Panic: {}", message)
            }
            resume_unwind(err);
        }

        Ok(())
    }
}
