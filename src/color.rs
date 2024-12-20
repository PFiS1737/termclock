use clap::{
    builder::styling::{self, AnsiColor, Style},
    ValueEnum,
};

#[derive(Debug, Clone, Copy)]
pub struct ColorConfig {
    pub number_color: Style,
    pub delimiter_color: Style,
    pub rainbow_mode: bool,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    #[value(alias = "brblock")]
    BrightBlack,
    #[value(alias = "brred")]
    BrightRed,
    #[value(alias = "brgreen")]
    BrightGreen,
    #[value(alias = "bryellow")]
    BrightYellow,
    #[value(alias = "brblue")]
    BrightBlue,
    #[value(alias = "brmagenta")]
    BrightMagenta,
    #[value(alias = "brcyan")]
    BrightCyan,
    #[value(alias = "brwhite")]
    BrightWhite,
}

pub const RAINBOW: [Color; 6] = [
    Color::Red,
    Color::Yellow,
    Color::Green,
    Color::Cyan,
    Color::Blue,
    Color::Magenta,
];

impl Color {
    pub fn to_style(self) -> Style {
        Style::new().bg_color(Some(styling::Color::Ansi(Into::<AnsiColor>::into(self))))
    }
}
impl Default for Color {
    fn default() -> Self {
        Self::White
    }
}

impl From<Color> for AnsiColor {
    fn from(value: Color) -> Self {
        match value {
            Color::Black => AnsiColor::Black,
            Color::Red => AnsiColor::Red,
            Color::Green => AnsiColor::Green,
            Color::Yellow => AnsiColor::Yellow,
            Color::Blue => AnsiColor::Blue,
            Color::Magenta => AnsiColor::Magenta,
            Color::Cyan => AnsiColor::Cyan,
            Color::White => AnsiColor::White,
            Color::BrightBlack => AnsiColor::BrightBlack,
            Color::BrightRed => AnsiColor::BrightRed,
            Color::BrightGreen => AnsiColor::BrightGreen,
            Color::BrightYellow => AnsiColor::BrightYellow,
            Color::BrightBlue => AnsiColor::BrightBlue,
            Color::BrightMagenta => AnsiColor::BrightMagenta,
            Color::BrightCyan => AnsiColor::BrightCyan,
            Color::BrightWhite => AnsiColor::BrightWhite,
        }
    }
}
