use clap::{
    builder::{styling::AnsiColor, Styles},
    Parser, Subcommand, ValueEnum,
};
use clap_complete::Shell;

use crate::color::Color;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(arg_required_else_help = true)]
#[command(styles = get_styles())]
pub struct TermClock {
    #[command(subcommand)]
    pub commands: Commands,

    /// The color of the clock numbers.
    #[arg(
        short,
        long,
        global = true,
        default_value = "green",
        hide_possible_values = true
    )]
    pub color: Color,

    /// The color of the delimiter ':' between numbers.
    #[arg(
        short = ':',
        long,
        global = true,
        default_value = "green",
        hide_possible_values = true,
        value_name = "COLOR"
    )]
    pub color_delimiter: Color,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Use as a clock.
    Clock {
        /// Whether to use a rainbow effect for the clock. Will override the color options.
        #[arg(short, long, conflicts_with = "color")]
        rainbow_mode: bool,

        /// The scale of the clock. Automatically adjusted to fit the terminal size if not set.
        #[arg(short = 'f', long)]
        scale: Option<u8>,

        /// Do not show seconds.
        #[arg(short = 's', long)]
        with_seconds: bool,

        /// The position of the number one in the clock to display.
        #[arg(short = '1', long, default_value = "right", value_name = "POSITION")]
        one_position: OnePosition,

        /// Whether to display date.
        #[arg(short = 'd', long)]
        with_date: bool,

        /// The format of the date.
        #[arg(
            short = 'F',
            long,
            default_value = "%Y/%m/%d %A",
            value_name = "FORMAT",
            requires = "with_date"
        )]
        date_format: String,

        /// Screensaver mode, quit on any key.
        #[arg(short = 'S', long)]
        screensaver: bool,
    },

    /// Print shell auto completions for the specified shell.
    Complete {
        /// The shell to generate auto completions for.
        shell: Shell,
    },
}

fn get_styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Yellow.on_default().bold().underline())
        .usage(AnsiColor::Yellow.on_default().bold().underline())
        .valid(AnsiColor::Green.on_default().bold().underline())
        .invalid(AnsiColor::Red.on_default().bold())
        .placeholder(AnsiColor::White.on_default())
        .error(AnsiColor::Red.on_default().bold())
        .literal(AnsiColor::Green.on_default())
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum OnePosition {
    #[value(alias = "l")]
    Left,
    #[value(alias = "m")]
    Middle,
    #[value(alias = "r")]
    Right,
}
