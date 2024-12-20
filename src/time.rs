use std::iter::repeat;

use anyhow::bail;
use chrono::{DateTime, Local, Timelike};
use clap::builder::styling::Style;
use crossterm::terminal::size;

use crate::{
    cli::OnePosition,
    color::{Color, ColorConfig, RAINBOW},
    Result,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct Time {
    hours: u8,
    minutes: u8,
    seconds: u8,
}

impl Time {
    pub fn update(&mut self, now: DateTime<Local>) {
        let now = now.time();
        self.hours = now.hour() as u8;
        self.minutes = now.minute() as u8;
        self.seconds = now.second() as u8;
    }

    // Returns somethng like this:
    // (the '#' will be replace a ' ' with background color)
    //
    // ###### ######      ###### ######      ######     ##
    // ##  ## ##      ##      ##     ##  ##      ##     ##
    // ##  ## ######      ######     ##      ######     ##
    // ##  ## ##  ##  ##      ##     ##  ##  ##         ##
    // ###### ######      ######     ##      ######     ##
    //
    // - width: 51
    // - height : 5
    pub fn to_string_lines(
        self,
        color_config: ColorConfig,
        scale: Option<u8>,
        with_seconds: bool,
        one_position: OnePosition,
    ) -> Result<Vec<String>> {
        let (width, height) = size()?;

        if width < 51 || height < 5 {
            bail!("The minimum terminal size to display the clock is 51x5.")
        }

        let scale = scale.unwrap_or((width / 51).min(height / 5) as u8) as usize;
        let mut rainbow_index = 0;

        Ok((if with_seconds {
            format!("{:02}:{:02}:{:02}", self.hours, self.minutes, self.seconds)
        } else {
            format!("{:02}:{:02}", self.hours, self.minutes)
        })
        .chars()
        .flat_map(|c| vec![c, ' '])
        .map(|c| {
            format_char(
                c,
                if color_config.rainbow_mode {
                    if c == ':' {
                        Color::White.to_style()
                    } else if c != ' ' {
                        let style = RAINBOW[rainbow_index].to_style();
                        rainbow_index += 1;
                        style
                    } else {
                        Color::default().to_style()
                    }
                } else if c == ':' {
                    color_config.delimiter_color
                } else {
                    color_config.number_color
                },
                scale,
                one_position,
            )
            .into_iter()
            .flat_map(|v| repeat(v).take(scale))
            .collect::<Vec<_>>()
        })
        .reduce(|acc, c| {
            acc.into_iter()
                .zip(c)
                .map(|(a, b)| format!("{}{}", a, b))
                .collect()
        })
        .unwrap_or_default())
    }
}

macro_rules! push {
    ($target:expr; $( $item:expr ),+) => {
        {
            $(
                $target.push($item);
            )+
        }
    }
}

fn format_char(c: char, color: Style, scale: usize, one_position: OnePosition) -> Vec<String> {
    let mut output = vec![];

    let cell = format!("{:width$}", "", width = 2 * scale);
    let cell_thin = format!("{:width$}", "", width = scale);

    let lines = [
        format!("{color}{cell}{cell}{cell}{color:#}"), // 0: "######"
        format!("{color}{cell}{color:#}{cell}{color}{cell}{color:#}"), // 1: "##  ##"
        format!("{color}{cell}{color:#}{cell}{cell}"), // 2: "##    "
        format!("{cell}{cell}{color}{cell}{color:#}"), // 3: "    ##"
        format!("{cell}{color}{cell}{color:#}{cell}"), // 4: "  ##  "
        format!("{cell}{cell}"),                       // 5: "    "
        format!("{cell_thin}{color}{cell}{color:#}{cell_thin}"), // 6: " ## "
        cell_thin.to_string(),                         // 7: " "
    ];

    match c {
        '0' => push!(output; 0, 1, 1, 1, 0),
        '1' => match one_position {
            OnePosition::Left => push!(output; 2, 2, 2, 2, 2),
            OnePosition::Middle => push!(output; 4, 4 ,4, 4, 4),
            OnePosition::Right => push!(output; 3, 3, 3, 3, 3),
        },
        '2' => push!(output; 0, 3, 0, 2, 0),
        '3' => push!(output; 0, 3, 0, 3, 0),
        '4' => push!(output; 1, 1, 0, 3, 3),
        '5' => push!(output; 0, 2, 0, 3, 0),
        '6' => push!(output; 0, 2, 0, 1, 0),
        '7' => push!(output; 0, 3, 3, 3, 3),
        '8' => push!(output; 0, 1, 0, 1, 0),
        '9' => push!(output; 0, 1, 0, 3, 0),
        ':' => push!(output; 5, 6, 5, 6, 5),
        ' ' => push!(output; 7, 7, 7, 7, 7),
        _ => panic!("Unexpected char '{c}'."),
    }

    output
        .into_iter()
        .map(|i| lines[i as usize].clone())
        .collect()
}
