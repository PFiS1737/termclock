mod app;
mod cli;
mod clock;
mod color;
mod time;

use std::io::stdout;

use anyhow::Result;
use clap::{CommandFactory, Parser};
use clap_complete::generate;

use cli::{Commands, TermClock};
use clock::clock;
use color::ColorConfig;

fn main() -> Result<()> {
    let cli = TermClock::parse();

    let mut color_config = ColorConfig {
        number_color: cli.color.to_style(),
        delimiter_color: cli.color_delimiter.to_style(),
        rainbow_mode: false,
    };

    match cli.commands {
        Commands::Complete { shell } => {
            generate(
                shell,
                &mut TermClock::command(),
                env!("CARGO_BIN_NAME"),
                &mut stdout(),
            );
        }
        Commands::Clock {
            rainbow_mode,
            scale,
            disable_seconds,
            one_position,
            with_date,
            date_format,
            screensaver,
        } => {
            color_config.rainbow_mode = rainbow_mode;

            clock(
                color_config,
                scale,
                disable_seconds,
                one_position,
                with_date,
                date_format,
                screensaver,
            )?
        }
    };

    Ok(())
}
