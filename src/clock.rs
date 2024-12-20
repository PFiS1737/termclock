use chrono::Local;
use console::measure_text_width;
use crossterm::terminal::size;

use crate::{app::App, cli::OnePosition, color::ColorConfig, time::Time, Result};

pub fn clock(
    color_config: ColorConfig,
    scale: Option<u8>,
    disable_seconds: bool,
    one_position: OnePosition,
    with_date: bool,
    date_format: String,
    screensaver_mode: bool,
) -> Result<()> {
    let app = App::new();
    let mut clock = Clock::new(date_format);

    app.run(
        || -> Result<()> {
            clock.update();

            let lines =
                clock.to_string_lines(color_config, scale, disable_seconds, one_position)?;

            let (cols, rows) = size()?;
            let start_col = cols.saturating_sub(measure_text_width(&lines[0]) as u16) / 2;
            let mut start_row = rows.saturating_sub(lines.len() as u16) / 2;

            for line in lines.iter() {
                app.move_to(start_col, start_row)?;
                app.print(line)?;
                start_row = start_row.saturating_add(1);
            }

            if with_date {
                let date = &clock.date;
                let start_col = cols.saturating_sub(date.len() as u16) / 2;
                let start_row = start_row.saturating_add(1);
                app.move_to(start_col, start_row)?;
                app.print(date)?;
            }

            Ok(())
        },
        |_| !screensaver_mode,
    )?;

    Ok(())
}

pub struct Clock {
    time: Time,
    date: String,
    date_format: String,
}

impl Clock {
    pub fn new(date_format: String) -> Self {
        Self {
            time: Time::default(),
            date: String::new(),
            date_format,
        }
    }

    pub fn update(&mut self) {
        let now = Local::now();
        self.time.update(now);
        self.date = now.format(&self.date_format).to_string();
    }

    pub fn to_string_lines(
        &self,
        color_config: ColorConfig,
        scale: Option<u8>,
        disable_seconds: bool,
        one_position: OnePosition,
    ) -> Result<Vec<String>> {
        self.time
            .to_string_lines(color_config, scale, disable_seconds, one_position)
    }
}
